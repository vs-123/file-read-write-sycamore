use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::fs;
use actix_cors::Cors;

async fn read(req: HttpRequest) -> impl Responder {
    let contents = fs::read_to_string("data/data.txt")
        .expect("Something went wrong reading the file");
    println!("HIT!");
    format!("{}", contents)
}

async fn write(req: HttpRequest) -> impl Responder {
    let cont = req.match_info().get("cont").unwrap_or("_");
    println!("{:?}", cont);
    fs::write("data/data.txt", format!("{{\"val\":\"{}\"}}", cont)).expect("Unable to write file");
    format!("200")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .route("/", web::get().to(read))
            .route("/write/{cont}", web::get().to(write))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}