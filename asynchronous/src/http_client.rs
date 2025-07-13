use async_std::io::{ReadExt, WriteExt};
use async_std::net as asy_net;
use async_std::task;
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

pub async fn async_cheapo_request(host: &str, port: u16, path: &str) -> io::Result<String> {
    let request = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    let mut client = asy_net::TcpStream::connect((host, port)).await?;
    client.write_all(request.as_bytes()).await?;
    client.shutdown(net::Shutdown::Write)?;
    let mut response = String::new();
    client.read_to_string(&mut response).await?;
    Ok(response)
}

pub async fn own_cheapo_request(host: String, port: u16, path: String) -> io::Result<String> {
    async_cheapo_request(&host, port, &path).await
}

pub async fn many_results(requests: Vec<(String, u16, String)>) -> Vec<io::Result<String>> {
    let mut handlers = vec![];
    for (host, port, path) in requests {
        handlers.push(task::spawn_local(own_cheapo_request(host, port, path)));
    }

    let mut resluts = vec![];
    for handler in handlers {
        resluts.push(handler.await);
    }
    resluts
}

#[cfg(test)]
mod tests {
    use async_std::task;

    use super::async_cheapo_request;
    use super::cheapo_request;
    use super::many_results;

    #[test]
    fn cheapo_test() {
        match cheapo_request("127.0.0.1", 8080, "/hello/TokeNatu") {
            Ok(ok) => println!("response: {ok}"),
            Err(err) => println!("error: {err}"),
        }
    }

    #[test]
    fn async_cheapo_test() {
        match task::block_on(async_cheapo_request(
            "127.0.0.1",
            8080,
            "/hello/asyncTokeNatu",
        )) {
            Ok(ok) => println!("response: {ok}"),
            Err(err) => println!("error: {err}"),
        }
    }

    #[test]
    fn many_results_test() {
        let requelsts = vec![
            (
                "127.0.0.1".to_string(),
                8080,
                "/hello/asyncTokeNatu".to_string(),
            ),
            ("127.0.0.1".to_string(), 8080, "/hello/afdasfa".to_string()),
            ("127.0.0.1".to_string(), 8080, "/hello/阿叔".to_string()),
        ];
        for reslut in task::block_on(many_results(requelsts)) {
            match reslut {
                Ok(ok) => println!("result: {ok}"),
                Err(err) => println!("error: {err:?}"),
            }
        }
    }
}
