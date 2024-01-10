#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "This is a test str"
}

#[get("/tasks")]
fn task_index() {}

#[post("/tasks")]
fn task_store() {}

#[get("/tasks/<id>")]
fn task_show(id: usize) {}

#[delete("/tasks/<id>")]
fn task_destroy(id: usize) {}

#[patch("/tasks/<id>")]
fn task_update(id: usize) {}

#[get("/finance/categories")]
fn category_index() {}

#[post("/finance/categories")]
fn category_store() {}

#[get("/finance/categories/<id>")]
fn category_show(id: usize) {}

#[delete("/finance/categories/<id>")]
fn category_delete(id: usize) {}

#[patch("/finance/categories/<id>")]
fn category_update(id: usize) {}

#[get("/habits")]
fn habit_index() {}

#[post("/habits")]
fn habit_store() {}

#[get("/habits/<id>")]
fn habit_show(id: usize) {}

#[delete("/habits/<id>")]
fn habit_destory(id: usize) {}

#[patch("/habits/<id>")]
fn habit_update(id: usize) {}

#[get("/settings")]
fn settings_index() {}

#[patch("/settings")]
fn settings_update() {}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
