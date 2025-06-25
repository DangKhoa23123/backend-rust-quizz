use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Register {
    pub username: String,
    pub password: String,
    pub email: String,
    pub phone_number: String,
    pub class: String,
    pub age: u32,
}
