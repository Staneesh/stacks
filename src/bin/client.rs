use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").expect("Couldn't bind a socket!");
    socket
        .send_to(&[0; 10], "70.34.254.33:8080")
        .expect("Didn't send!");
}
