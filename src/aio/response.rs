#[derive(Copy, Clone)]
pub struct Response {}

impl Response {
    pub async fn respond(&self, content: String) {
        println!("----- response {}", content);
    }
}
