use std::io::{Read, Write};
use std::net::TcpStream;

type IOError = std::io::Error;

/// Perform a limited http get request
fn http_get_request(addr: &str) -> Result<String, IOError> {
    let mut stream = TcpStream::connect(format!("{}:80", addr))?;

    let mut hdr = String::new();
    hdr.push_str("GET / HTTP/1.1");
    hdr.push_str("\r\n");
    hdr.push_str(format!("Host: {}", addr).as_str());
    hdr.push_str("\r\n");
    hdr.push_str("\r\n");

    stream.write_all(hdr.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    Ok(response)
}

/// To be able to communicate with other nodes
/// we need to know the public ip
pub fn get_external_ip() -> Result<String, IOError> {
    let url = "checkip.dyndns.org";
    let keyword = "Address: ";

    let response = http_get_request(url)?;
    for line in response.lines() {
        if let Some(start) = line.find(keyword) {
            if let Some(end) = line.find("</body>") {
                let ip = line[start + keyword.len()..end].to_string();
                return Ok(ip);
            }
        }
    }

    Err(IOError::from(std::io::ErrorKind::NotFound))
}
