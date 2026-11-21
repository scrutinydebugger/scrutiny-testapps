mod file1;
mod file2;
mod file3;
mod file4;
mod file5;
mod file6;
mod memdump;
mod rpv;
mod udp_listener;
mod user_command;

use rpv::RPV_STORAGE;
use scrutiny_embedded_rust::config::Config;
use scrutiny_embedded_rust::loop_handler::{FixedFreqLoopHandler, LoopHandler, VariableFreqLoopHandler};
use scrutiny_embedded_rust::main_handler::MainHandler;
use std::io;
use std::time::{Duration, Instant};
use udp_listener::UdpListener;

// 100Hz = 10ms = 100000 x 100ns
static FF_LOOP: FixedFreqLoopHandler = FixedFreqLoopHandler::new("100Hz Loop", 100_000);
static VF_LOOP: VariableFreqLoopHandler = VariableFreqLoopHandler::new("Variable freq loop");
static LOOP_HANDLERS: [&LoopHandler; 2] = [FF_LOOP.handler(), VF_LOOP.handler()];

const DL_BUF_SIZE: usize = 4096;

fn mainfunc1() {
    #[used]
    static mut MAINFUNC1_VAR: i32 = 7777777;
}

fn mainfunc1_with_arg(_x: i32) {
    #[used]
    static mut MAINFUNC1_VAR: f64 = 8888888.88;
}

fn init_all_values() {
    file1::file1_set_values();
    file2::file2_set_values();
    file3::file3_set_values();
    file4::file4_set_values();
    file5::file5_set_values();
    file6::file6_set_values();
    file1::func_in_file1(1, 2);
    file2::file2func1();
    file2::file2func1_with_arg(123);
    file6::File6WithMethod::method_with_static();
    mainfunc1();
    mainfunc1_with_arg(123);
    rpv::rpv_set_values();
}

// Variables meant to be changed from the Scrutiny GUI while the app runs.
fn process_interactive_data() {
    #[used]
    static mut ENABLE: bool = false;
    #[used]
    static mut COUNTER: i16 = 0;
    #[used]
    static mut STEP: i16 = 1;

    unsafe {
        // Volatile: written externally by the debugger.
        let enable = core::ptr::read_volatile(&raw const ENABLE);
        let step = core::ptr::read_volatile(&raw const STEP);

        if RPV_STORAGE.rpv_id_5000 {
            RPV_STORAGE.rpv_id_5001 = RPV_STORAGE.rpv_id_5001.wrapping_add(step as u16);
        }

        if enable {
            let counter = core::ptr::read_volatile(&raw const COUNTER);
            core::ptr::write_volatile(&raw mut COUNTER, counter.wrapping_add(step));
        }
    }
}

fn datalogger_triggered(caller: Option<&'static str>) {
    println!("Graph Triggered! (loop: {})", caller.unwrap_or("<unknown>"));
}

fn print_frame(elapsed_us: u128, direction: &str, data: &[u8]) {
    let mut line = format!("{elapsed_us}  {direction}: ({:2})  ", data.len());
    for b in data {
        line.push_str(&format!("{b:02x}"));
    }
    println!("{line}");
}

fn run(port: u16) -> io::Result<()> {
    let mut listener = UdpListener::new(port)?;

    let mut rx_buf = [0u8; 128];
    let mut tx_buf = [0u8; 256];
    // Staging buffer for socket I/O — large enough for a single framed message.
    let mut io_buf = [0u8; 1024];

    let mut config = Config::new();
    config.set_buffers(&mut rx_buf, &mut tx_buf);
    config.max_bitrate = 100_000;
    config.display_name = "rust-testapp";
    config.session_counter_seed = 0xdead_beef;
    config.set_loops(&LOOP_HANDLERS);
    config.set_published_values(&rpv::RPVS, Some(rpv::rpv_read), Some(rpv::rpv_write));
    config.set_user_command_callback(user_command::user_command);
    // Leaked once at startup to obtain the 'static buffer the datalogger requires.
    let dl_buf: &'static mut [u8] = Box::leak(Box::new([0u8; DL_BUF_SIZE]));
    config.set_datalogger_buffer(dl_buf, Some(datalogger_triggered));

    let mut handler = MainHandler::new(config)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "scrutiny handler init failed"))?;

    let start = Instant::now();
    let mut last_tick = Instant::now();

    loop {
        process_interactive_data();
        let n = listener.poll(&mut io_buf)?;

        let now = Instant::now();
        // Elapsed in 100 ns units, clamped to u32 to match Timediff type.
        let timestep_100ns = (now.duration_since(last_tick).as_micros() * 10).min(u128::from(u32::MAX)) as u32;
        last_tick = now;

        if n > 0 {
            print_frame(start.elapsed().as_micros(), "in", &io_buf[..n]);
        }

        handler.receive_data(&io_buf[..n]);
        handler.process(timestep_100ns);

        let to_send = handler.data_to_send() as usize;
        if to_send > 0 {
            let sent = handler.pop_data(&mut io_buf[..to_send]) as usize;
            listener.send(&io_buf[..sent])?;
            print_frame(start.elapsed().as_micros(), "out", &io_buf[..sent]);
        }

        VF_LOOP.process_with_time(timestep_100ns);
        FF_LOOP.process();

        std::thread::sleep(Duration::from_millis(10));
    }
}

fn print_usage(prog: &str) {
    eprintln!("Usage: {prog} <command> [args]");
    eprintln!("Commands:");
    eprintln!("  udp-listen <port>                       Listen on UDP and serve the Scrutiny protocol");
    eprintln!("  memdump <addr> <len> [<addr> <len>...]  Dump memory regions (decimal or 0x-prefixed hex)");
}

fn cmd_memdump(prog: &str, args: &[String]) -> i32 {
    if args.is_empty() || !args.len().is_multiple_of(2) {
        eprintln!("Usage: {prog} memdump <addr> <len> [<addr> <len>...]");
        return 1;
    }

    for pair in args.chunks(2) {
        let start = memdump::parse_number(&pair[0]);
        let length = memdump::parse_number(&pair[1]);
        match (start, length) {
            (Some(start), Some(length)) => memdump::memdump(start, length),
            _ => {
                eprintln!("Invalid memory region: {} {}", pair[0], pair[1]);
                return 1;
            }
        }
    }
    0
}

fn cmd_udp_listen(prog: &str, args: &[String]) -> i32 {
    if args.len() != 1 {
        eprintln!("Usage: {prog} udp-listen <port>");
        return 1;
    }
    let port: u16 = match args[0].parse() {
        Ok(p) if p > 0 => p,
        _ => {
            eprintln!("Invalid port '{}': must be 1–65535", args[0]);
            return 1;
        }
    };
    if let Err(e) = run(port) {
        eprintln!("Error: {e}");
        return 1;
    }
    0
}

fn main() {
    #[used]
    static mut STATIC_INT_IN_MAIN_FUNC: i32 = 22222;

    init_all_values();

    let args: Vec<String> = std::env::args().collect();
    let prog = &args[0];

    if args.len() < 2 {
        print_usage(prog);
        std::process::exit(1);
    }

    let errorcode = match args[1].to_lowercase().as_str() {
        "memdump" => cmd_memdump(prog, &args[2..]),
        "udp-listen" => cmd_udp_listen(prog, &args[2..]),
        cmd => {
            eprintln!("Unknown command: {cmd}");
            print_usage(prog);
            1
        }
    };
    std::process::exit(errorcode);
}
