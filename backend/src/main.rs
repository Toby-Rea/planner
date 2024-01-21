use actix_web::{
    delete, get, middleware::Logger, patch, post, web, App, HttpResponse, HttpServer, Responder,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        let task_scope = web::scope("/tasks")
            .service(task_index)
            .service(task_store)
            .service(task_show)
            .service(task_destroy)
            .service(task_update);
        let category_scope = web::scope("/finance/categories")
            .service(category_index)
            .service(category_store)
            .service(category_show)
            .service(category_destroy)
            .service(category_update);
        let habit_scope = web::scope("/habits")
            .service(habit_index)
            .service(habit_store)
            .service(habit_show)
            .service(habit_destroy)
            .service(habit_update);
        let settings_scope = web::scope("/settings")
            .service(settings_index)
            .service(settings_update);

        App::new()
            .wrap(Logger::default())
            .service(task_scope)
            .service(category_scope)
            .service(habit_scope)
            .service(settings_scope)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("")]
async fn task_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[post("")]
async fn task_store() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("/{id}")]
async fn task_show(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[delete("/{id}")]
async fn task_destroy(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("{id}")
}

#[patch("/{id}")]
async fn task_update(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("{id}")
}

#[get("")]
async fn category_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[post("")]
async fn category_store() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("/{id}")]
async fn category_show(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[delete("/{id}")]
async fn category_destroy(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[patch("/{id}")]
async fn category_update(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("")]
async fn habit_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[post("")]
async fn habit_store() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("/{id}")]
async fn habit_show(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[delete("/{id}")]
async fn habit_destroy(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[patch("/{id}")]
async fn habit_update(path: web::Path<(u32)>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[get("")]
async fn settings_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

#[patch("")]
async fn settings_update() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}
