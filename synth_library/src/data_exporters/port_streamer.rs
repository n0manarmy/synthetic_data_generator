use std::net::UdpSocket;

pub struct PortStreamer {
    connect_from: String,
    stream_to: String,
}

impl PortStreamer {
    const COLON: &'static str = ":";

    pub fn new(src_ip: &str, src_port: &str, dst_port: &str) -> PortStreamer {
        PortStreamer {
            connect_from: [&src_ip, Self::COLON, &src_port].concat(),
            stream_to: [&src_ip, Self::COLON, &dst_port].concat(),
        }
    }

    pub fn connect_and_send(&self, record: String) {
        //create stream for dumping data
        let udp_out =
            UdpSocket::bind(&self.connect_from).expect("failed to bind to source address");
        udp_out
            .connect(&self.stream_to)
            .expect("failed to bind to source address");
        udp_out
            .send(&record.as_bytes())
            .expect("failed to transmit packet");
    }

    pub fn get_dst(&self) -> String {
        self.stream_to.clone()
    }
}
