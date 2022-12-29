use rocket::{Route, routes};

mod user;

pub fn user_routes() -> Vec<Route> {
    routes![user::get_user, user::create_user, user::patch_user]
}