use actix_web::{
    delete, get, middleware::Logger, patch, post, web, App, HttpResponse, HttpServer, Responder,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(settings_index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("/tasks")]
async fn task_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[post("/tasks")]
async fn task_store() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("/tasks/{id}")]
async fn task_show(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[delete("/tasks/{id}")]
async fn task_destroy(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("{id}")
}

#[patch("/tasks/{id}")]
async fn task_update(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("{id}")
}

#[get("/finance/categories")]
async fn category_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[post("/finance/categories")]
async fn category_store() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("/finance/categories/{id}")]
async fn category_show(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[delete("/finance/categories/{id}")]
async fn category_delete(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[patch("/finance/categories/{id}")]
async fn category_update(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("/habits")]
async fn habit_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[post("/habits")]
async fn habit_store() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("/habits/{id}")]
async fn habit_show(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[delete("/habits/{id}")]
async fn habit_destory(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[patch("/habits/{id}")]
async fn habit_update(path: web::Path<(u32t)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("/settings")]
async fn settings_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[patch("/settings")]
async fn settings_update() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}
