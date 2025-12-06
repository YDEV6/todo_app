//todo_app
use rocket::*;

#[get("/")]
fn hello_user() ->&'static str{
    "Hello, User"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/",routes![hello_user])
}