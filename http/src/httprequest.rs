use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Resource {
    Path(String),
}

#[derive(PartialEq, Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        //create empty resources
        let mut parsed_method = Method::UNINITIALIZED;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers: HashMap<String, String> = HashMap::new();
        let mut parsed_msg_body = "";

        //read each line
        for line in req.lines() {
            // process_req_line()
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
                continue;
            } else {
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

// impl HttpRequest {
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();

    //"GET /path HTTP/1.1" => Some("GET"), Some("/path"), Some("HTTP/1.1")
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    // Extract the key part of the header
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }
    // Extract the value part of the header
    if let Some(v) = header_items.next() {
        value = v.to_string();
    }
    (key, value)
}

#[derive(PartialEq, Debug)]
pub enum Method {
    GET,
    POST,
    UNINITIALIZED,
}
impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::UNINITIALIZED,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Version {
    V1_1,
    V2_0,
    UNINITIALIZED,
}
impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::UNINITIALIZED,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from() {
        let new_method: Method = "GET".into();
        assert_eq!(new_method, Method::GET);
    }

    #[test]
    fn test_version_from() {
        let version: Version = "HTTP/1.1".into();
        assert_eq!(version, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent:curl/7.64.1\r\nAccept:*/*\r\n\r\n");

        let mut headers_expected: HashMap<String, String> = HashMap::new();
        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
        headers_expected.insert("Accept".into(), "*/*".into());

        let req: HttpRequest = s.into();

        assert_eq!(headers_expected, req.headers);
        assert_eq!(Method::GET, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
    }
}
