use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: String,
}
