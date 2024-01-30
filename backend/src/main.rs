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

/// The index handler for the root API URL .
///
/// The URL for `index()` is `/``.
#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The index handler for the root of tasks section of the API.
///
/// The URL for `task_index()` is `/tasks`.
#[get("")]
async fn task_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The storage handler for new task entries.
///
/// The URL for `task_store()` is `/tasks`.
#[post("")]
async fn task_store() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The show handler for a task entry with specified ID of `id`.
///
/// The URL for `task_show()` is `/tasks/{id}`.
#[get("/{id}")]
async fn task_show(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The destroy handler for a task entry with specified ID of `id`.
///
/// The URL for `task_destroy()` is `/tasks/{id}`.
#[delete("/{id}")]
async fn task_destroy(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body("{id}")
}

/// The update handler for a task entry with specified ID of `id`.
///
/// The URL for `task_update()` is `/tasks/{id}`.
#[patch("/{id}")]
async fn task_update(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body("{id}")
}

/// The index handler for the root of the categories section of the API.
///
/// The URL for `category_index()` is `/finance/categories`.
#[get("")]
async fn category_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The storage handler for new category entries.
///
/// The URL for `category_store()` is `/finance/categories/{id}`.
#[post("")]
async fn category_store() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The show handler for a category entry with specified ID of `id`.
///
/// The URL for `category_show()` is `/finance/categories/{id}`.
#[get("/{id}")]
async fn category_show(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The destroy handler for a category entry with specified ID of `id`.
///
/// The URL for `category_destroy()` is `/finance/categories/{id}`.
#[delete("/{id}")]
async fn category_destroy(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The update handler for a category entry with specified ID of `id`.
///
/// The URL for `category_update()` is `/finance/categories/{id}`.
#[patch("/{id}")]
async fn category_update(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The index handler for the root of the habits section of the API.
///
/// The URL for `habit_index()` is `/habits`
#[get("")]
async fn habit_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The storage handler for new habits entries.
///
/// The URL for `habit_store()` is `/habits/{id}`.
#[post("")]
async fn habit_store() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The show handler for a habit entry with specified ID of `id`.
///
/// The URL for `habit_show()` is `/habits/{id}`.
#[get("/{id}")]
async fn habit_show(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The destroy handler for a habit entry with specified ID of `id`.
///
/// The URL for `habit_destroy()` is `/habits/{id}`.
#[delete("/{id}")]
async fn habit_destroy(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The update handler for a habit entry with specified ID of `id`.
///
/// The URL for `habit_update()` is `/habits/{id}`.
#[patch("/{id}")]
async fn habit_update(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The index handler for the root of the settings section of the API.
///
/// The URL for `settings_index()` is `/settings`.
#[get("")]
async fn settings_index() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}

/// The update handler for the settings configuration file.
///
/// The URL for `settings_update()` is `/settings`.
#[patch("")]
async fn settings_update() -> impl Responder {
    HttpResponse::Ok().body("This is a test str")
}
