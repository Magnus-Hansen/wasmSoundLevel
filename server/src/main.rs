use async_std::net::UdpSocket;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:8080").await?;
        println!("Listening on {}", socket.local_addr()?);

        let mut buf = vec![0u8; 1024];

        loop
        {
            let (recv, peer) = socket.recv_from(&mut buf).await?;
            let data = i32::from_be_bytes(buf[0..4].try_into().unwrap());

            println!("Received {} from {:?}", data, peer);
        }
    }
}