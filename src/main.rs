use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}", &name)
// }

#[get("/")] // La ruta debe ir siempre arriba del método...
async fn greetings() -> impl Responder {
    HttpResponse::Ok().body("Greetings to everyone !")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_greetings() -> impl Responder {
    HttpResponse::Ok().body("Hey everybody !")
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(greet))
//             .route("/{name}", web::get().to(greet))
//     })
//     .bind(("127.0.0.1", 8085))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greetings)
            .service(echo)
            .route("/hey", web::get().to(manual_greetings))
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
