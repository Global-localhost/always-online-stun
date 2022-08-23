use std::io::{Read, Write};
use stun_proto::rfc5389::*;

/**
    In this example, we are calling a TCP stun server
    To get our external IP and port
 */
fn main() {
    let mut buf = [0u8; 32];

    let mut writer = Writer::new(&mut buf);
    writer.set_message_type(MessageType::BindingRequest).unwrap();
    writer.set_transaction_id(1).unwrap();
    let bytes_written = writer.finish().unwrap();

    let mut socket = std::net::TcpStream::connect("stun.stunprotocol.org:3478").unwrap();
    socket.write_all(&buf[0..bytes_written as usize]).unwrap();

    socket.set_read_timeout(Some(std::time::Duration::from_millis(1000))).unwrap();
    let bytes_read = socket.read(&mut buf).unwrap();

    // we assume that the entire message has been read in one go
    // a more robust client would look at the message length and keep reading if necessary

    let reader = Reader::new(&buf[0..bytes_read]);
    if let MessageType::BindingResponse = reader.get_message_type().unwrap() {
        assert_eq!(1u128, reader.get_transaction_id().unwrap());
        let external_addr = reader.get_attributes()
            .map(|attr| {
                match attr {
                    Ok(ReaderAttribute::MappedAddress(addr)) => Some(addr.get_address().unwrap()),
                    Ok(ReaderAttribute::XorMappedAddress(addr)) => Some(addr.get_address().unwrap()),
                    _ => None,
                }
            })
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .next()
            .unwrap();

        let std_external_addr = match external_addr {
            SocketAddr::V4(ip, port) => {
                std::net::SocketAddr::new(std::net::IpAddr::from(ip.to_be_bytes()), port)
            },
            SocketAddr::V6(ip, port) => {
                std::net::SocketAddr::new(std::net::IpAddr::from(ip.to_be_bytes()), port)
            },
        };

        println!("NAT mapping {} -> {}", socket.local_addr().unwrap(), std_external_addr);
    } else {
        assert!(false, "Response is not a BindingResponse")
    }
}
