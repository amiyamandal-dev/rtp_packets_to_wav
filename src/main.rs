use rtp_rs::*;
use std::fs::File;
use std::io::prelude::*;
use std::net::UdpSocket;
use std::str;
use std::thread;

fn main() {
    let socket = match UdpSocket::bind("localhost:8080") {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e),
    };

    let mut file = File::create("foo.txt").unwrap();

    let mut buf = [0; 2048 * 2];
    let mut buf2: Vec<u8> = vec![];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                println!("amt: {}", amt);
                println!("src: {}", src);
                // println!("{}", str::from_utf8(&buf).unwrap_or(""));
            }
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
        if let Ok(rtp) = RtpReader::new(&buf) {
            println!("Sequence number {:?}", rtp.sequence_number());
            println!("Payload length {:?}", rtp.payload().len());
            println!("Payload timestamp {:?}", rtp.timestamp());
            println!("{:?}", str::from_utf8(&buf).unwrap_or(""));
            for i in rtp.payload() {
                buf2.push(*i);
                file.write(&[*i]).unwrap();
            }
        }
    }

    println!("closing file");
}
