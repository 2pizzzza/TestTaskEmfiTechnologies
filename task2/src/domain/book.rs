use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: Option<u32>,
    pub title: String,
    pub author: String,
    pub year: u32,
}
