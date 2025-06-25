use actix_web::web;
use crate::router::login_router::login;
// use crate::router::register_router::register;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
    // cfg.service(register);
}
