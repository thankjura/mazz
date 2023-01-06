use rocket::{Route, routes};

mod user;
mod customfield;
mod project;

pub fn user_routes() -> Vec<Route> {
    routes![
        user::get_user,
        user::create_user,
        user::patch_user,
        user::delete_user,
    ]
}

pub fn customfield_routes() -> Vec<Route> {
    routes![
        customfield::get_customfield,
        customfield::create_customfield,
        customfield::patch_customfield,
        customfield::delete_customfield,
    ]
}

pub fn project_routes() -> Vec<Route> {
    routes![
        project::get_project,
        project::create_project,
        project::patch_project,
        project::delete_project,
    ]
}