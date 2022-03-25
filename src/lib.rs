use actix_web::{web, App, HttpRequest, HttpResponse,HttpServer, Responder};
use actix_web::dev::Server;

async fn greet(req: HttpRequest)-> impl Responder{
    let name = req.match_info().get("name").unwrap_or("Hello");
    format!("Hi {}!", &name)
}
async fn health_check(req: HttpRequest) -> impl Responder{
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error>{
    let server = HttpServer::new(||{
            App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}",web::get().to(greet))})
    .bind("127.0.0.1:9999")?
    .run();
    Ok(server)
}