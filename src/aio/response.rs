
use tokio::net::TcpStream;


// #[derive(Copy, Clone)]
pub struct Response {
    // socket: &'a TcpStream
}

impl Response  {
    pub async fn respond(&self, content: String) {
        println!("----- response {}", content);
    }
}
