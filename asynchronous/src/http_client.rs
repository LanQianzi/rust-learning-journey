use std::io::Write;
use std::io::{self, Read};
use std::net;
pub fn cheapo_request(host: &str, port: u16, path: &str) -> io::Result<String> {
    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    let mut client = net::TcpStream::connect((host, port))?;
    client.write_all(request.as_bytes())?;
    client.shutdown(net::Shutdown::Write)?;
    let mut response = String::new();
    client.read_to_string(&mut response)?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::cheapo_request;
    #[test]
    fn cheapo_test() {
        match cheapo_request("127.0.0.1", 8080, "/hello/TokeNatu") {
            Ok(ok) => println!("response: {ok}"),
            Err(err) => println!("error: {err}"),
        }
    }
}
