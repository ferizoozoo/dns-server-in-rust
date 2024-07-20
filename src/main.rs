// Uncomment this block to pass the first stage
use std::net::UdpSocket;

struct Request {
    header: DnsHeader,
}

impl Request {
    fn parse_request(buf: &[u8]) -> Self {
        let header = DnsHeader::new(buf);
        Self { header }
    }
}

struct DnsHeader {
    ID: u16,
    QR: bool,
    OPCODE: u8,
    AA: bool,
    TC: bool,
    RD: bool,
    RA: bool,
    Z: u8,
    RCODE: u8,
    QDCOUNT: u16,
    ANCOUNT: u16,
    NSCOUNT: u16,
    ARCOUNT: u16,
}

impl DnsHeader {
    fn new(buf: &[u8]) -> Self {
        let ID = ((buf[0] as u16) << 8) | buf[1] as u16;
        let flags = ((buf[2] as u16) << 8) | buf[3] as u16;

        let QR = (flags >> 15) & 0b1 != 0;
        let OPCODE = ((flags >> 11) & 0b1111) as u8;
        let AA = (flags >> 10) & 0b1 != 0;
        let TC = (flags >> 9) & 0b1 != 0;
        let RD = (flags >> 8) & 0b1 != 0;
        let RA = (flags >> 7) & 0b1 != 0;
        let Z = ((flags >> 4) & 0b111) as u8;
        let RCODE = (flags & 0b1111) as u8;

        let QDCOUNT = ((buf[4] as u16) << 8) | buf[5] as u16;
        let ANCOUNT = ((buf[6] as u16) << 8) | buf[7] as u16;
        let NSCOUNT = ((buf[8] as u16) << 8) | buf[9] as u16;
        let ARCOUNT = ((buf[10] as u16) << 8) | buf[11] as u16;

        Self {
            ID,
            QR,
            OPCODE,
            AA,
            TC,
            RD,
            RA,
            Z,
            RCODE,
            QDCOUNT,
            ANCOUNT,
            NSCOUNT,
            ARCOUNT,
        }
    }

    fn to_bytes(&self) -> [u8; 12] {
        [
            (self.ID >> 8) as u8,
            (self.ID & 0xff) as u8,
            ((self.QR as u8) << 7)
                | (self.OPCODE << 3)
                | ((self.AA as u8) << 2)
                | ((self.TC as u8) << 1)
                | (self.RD as u8),
            ((self.RA as u8) << 7) | (self.Z << 3) | self.RCODE,
            (self.QDCOUNT >> 8) as u8,
            (self.QDCOUNT & 0xff) as u8,
            (self.ANCOUNT) as u8,
            (self.ANCOUNT & 0xff) as u8,
            (self.NSCOUNT >> 8) as u8,
            (self.NSCOUNT & 0xff) as u8,
            (self.ARCOUNT >> 8) as u8,
            (self.ARCOUNT & 0xff) as u8,
        ]
    }

    fn set_qr(&mut self, qr: bool) {
        self.QR = qr
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let mut req = Request::parse_request(&buf as &[u8]);
                req.header.set_qr(true);
                let response = req.header.to_bytes();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
