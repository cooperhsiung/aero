use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub version: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

// use From, lifetime parameter
impl From<String> for HttpRequest {
    fn from(req: String) -> HttpRequest {
        let mut parsed_method = "".to_string();
        let mut parsed_version = "1,0".to_string();
        let mut parsed_resource = "".to_string();
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";
        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, path, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = path;
            // If the line read is header line, call function process_header_line()
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            //  If it is blank line, do nothing
            } else if line.len() == 0 {
                // If none of these, treat it as message body
            } else {
                parsed_msg_body = line;
            }
        }

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            path: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (String, String, String) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resouce = words.next().unwrap();
    let version = words.next().unwrap();

    (method.to_string(), resouce.to_string(), version.to_string())
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        value = v.to_string();
    }

    (key, value)
}
