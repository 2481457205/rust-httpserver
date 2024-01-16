use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s { 
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V1_2,
    Uninitialized
}
impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/1.2" => Version::V1_2,
            _ => Version::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub smg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version ) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
                
            } else if line.contains(":") {
                let (key,value) = process_handle_line(line);
                parsed_headers.insert(key,value);

            } else if line.len() == 0 {

            }else {
                parsed_msg_body = line;
            }
        }
        HttpRequest {
            method : parsed_method,
            version : parsed_version,
            resource : parsed_resource,
            headers : parsed_headers,
            smg_body : parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s:  &str) ->(Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_handle_line(s: &str) -> (String, String) {
    let mut headler_item = s.split(":");
    let mut key = String::new();
    let mut value = String::new();
    if let Some(k) = headler_item.next() {
        key = k.to_string();

    }
    if let Some(v) = headler_item.next() {
        value = v.to_string();
    }
    (key, value)
}

#[cfg(test)]
mod tsets {


    //use std::ptr::eq;
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "Get".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1)
    }
    
    # [test]
    fn test_read_http() {
        let s: String = String::from("Get /qwb HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.3\r\nAccept: */*\r\n\r\n");
        let mut headers_exected: HashMap<String, String>= HashMap::new();
        headers_exected.insert("Host".into(), " localhost".into());
        headers_exected.insert("Accept".into(), " */*".into());
        headers_exected.insert("User-Agent".into()," curl/7.71.3".into());
        
        let  req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/qwb".to_string()), req.resource);
        assert_eq!(headers_exected, req.headers);
    }
}