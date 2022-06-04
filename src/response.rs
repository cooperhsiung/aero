use std::collections::HashMap;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1", // not into...
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        content_type: &'a str,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default(); // not type
        if status_code != "200" {
            response.status_code = status_code.into();
        };

        response.headers = match headers {
            Some(mut sss) => {
                if content_type != "" {
                    sss.insert("Content-Type", content_type);
                }
                Some(sss)
            }
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                if content_type != "" {
                    h.insert("Content-Type", content_type);
                }
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        response.body = body;
        response
    }

    pub async fn send_response(&self, write_stream: &mut TcpStream) -> io::Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        // write_stream.
        // let _ = write!(write_stream, "{}", response_string);
        write_stream.write_all(response_string.as_bytes()).await?;
        // write_stream
        Ok(())
    }
}

impl<'a> HttpResponse<'a> {
    fn version(&self) -> &str {
        self.version
    }
    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_text(&self) -> &str {
        self.status_text
    }
    fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_string = "".into();
        for (k, v) in map {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b, // not type String -> str,  str -> String
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone(); // todo clone
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
}
