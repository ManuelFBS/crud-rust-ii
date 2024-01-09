use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}", &name)
// }

/// La función `saludos` devuelve una respuesta HTTP con el cuerpo "¡Saludos a todos!" cuando se accede
/// a la ruta raíz.
#[get("/")] // La ruta debe ir siempre arriba del método...
async fn greetings() -> impl Responder {
    HttpResponse::Ok().body("Greetings to everyone !")
}

/// La función `echo` es una función asincrónica de Rust que toma el cuerpo de una solicitud como
/// parámetro y devuelve una respuesta HTTP con el mismo cuerpo.
///
/// Arguments:
///
/// * `req_body`: El parámetro `req_body` es de tipo `String` y representa el cuerpo de la solicitud
/// HTTP.
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// La función `manual_greetings` devuelve una respuesta HTTP con el cuerpo "¡Hola a todos!"
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

/// La función anterior configura un servidor web Actix que escucha en el puerto localhost 8085 y maneja
/// solicitudes de saludos y ecos.
///
/// Returns:
///
/// La función `main` devuelve un `std::io::Result<()>`.
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
