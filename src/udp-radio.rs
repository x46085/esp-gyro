use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:43434").expect("couldn't bind to address");

    let mut buf = [0u8; 16];
    loop {
        match socket.recv(&mut buf) {
            Ok(received) => println!("received {received} bytes {:?}", &buf[..received]),
            Err(e) => println!("recv function failed: {e:?}"),
        }
    }
}
