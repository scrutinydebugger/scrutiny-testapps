// Same subfunctions as the C++ testapp:
//   0: 8 bytes [subfn, 0, 1, ..., 6]
//   1: fills the whole response buffer [subfn, 1, 2, ...]
//   2: reports a length bigger than the buffer (error case)
//   3: empty response
//   4: echoes the request after [subfn]
//   other: [0xFF]
pub fn user_command(subfn: u8, request: &[u8], response: &mut [u8], response_len: &mut u16) {
    print!("User command: Subfunction #{subfn} with {} data bytes: ", request.len());
    for b in request {
        print!("{b:02x}");
    }
    println!();

    let max_len = response.len();
    if max_len < 1 {
        *response_len = 0;
        return;
    }

    if subfn == 0 && max_len >= 8 {
        response[0] = subfn;
        for i in 0..7 {
            response[i + 1] = i as u8;
        }
        *response_len = 8;
    } else if subfn == 1 {
        response[0] = subfn;
        for (i, b) in response.iter_mut().enumerate().skip(1) {
            *b = i as u8;
        }
        *response_len = max_len as u16;
    } else if subfn == 2 {
        *response_len = (max_len + 1) as u16;
    } else if subfn == 3 {
        *response_len = 0;
    } else if subfn == 4 {
        let echo_len = request.len().min(max_len - 1);
        response[0] = subfn;
        response[1..echo_len + 1].copy_from_slice(&request[..echo_len]);
        *response_len = (echo_len + 1) as u16;
    } else {
        response[0] = 0xFF;
        *response_len = 1;
    }
}
