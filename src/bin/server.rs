use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").expect("Couldn't bind a socket!");
    let mut buf = [0; 100];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data!");
    let filled_buf = &mut buf[..number_of_bytes];
    println!("BUF:{:?}\n", filled_buf);
}
