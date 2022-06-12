use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3401").expect("Couldn't bind a socket!");
    socket
        .send_to(&[0; 10], "127.0.0.1:3400")
        .expect("Didn't receive data!");
}
