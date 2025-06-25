use actix_web::{post, web, HttpResponse, Responder};
use crate::model::{login_model::Login};
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    success: bool,
    message: String,
}

#[post("/login")]
pub async fn login(info: web::Json<Login>) -> impl Responder {
    let mock_email = "admin@example.com";
    let mock_phone = "0123456789";
    let mock_password = "123456";

    let valid_user = 
    (info.email.as_deref() == Some(mock_email)|| info.phone.as_deref() == Some(mock_phone))
        && info.password == mock_password;

    if valid_user {
        HttpResponse::Ok().json(LoginResponse {
            success: true,
            message: "Đăng nhập thành công!".to_string(),
        })
    } else {
        HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "Sai thông tin đăng nhập!".to_string(),
        })
    }
}