// Prints a memory region of this process, 16 bytes per line, in the same format as the C++ testapp:
//   0x<16 hex digits address>:\t<hex bytes>
pub fn memdump(start_addr: usize, length: usize) {
    let end_addr = start_addr + length;
    let mut addr = start_addr;
    while addr < end_addr {
        let n_to_print = (end_addr - addr).min(16);
        let mut line = format!("0x{addr:016x}:\t");
        for i in 0..n_to_print {
            // Volatile read: arbitrary addresses given on the command line, must not be optimized.
            let byte = unsafe { core::ptr::read_volatile((addr + i) as *const u8) };
            line.push_str(&format!("{byte:02x}"));
        }
        println!("{line}");
        addr += n_to_print;
    }
}

// Accepts decimal or 0x-prefixed hexadecimal.
pub fn parse_number(s: &str) -> Option<usize> {
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        usize::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<usize>().ok()
    }
}
