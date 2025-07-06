use actix_web::{Responder, get, web};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}
#[cfg(test)]
mod tests {
    use actix_web::{App, HttpServer};
    use std::io;

    use crate::web::greet;

    #[actix_web::test]
    async fn test_server() -> io::Result<()> {
        HttpServer::new(|| App::new().service(greet))
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
    }
}
