use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VocabResponseDto {
    pub id: i32,
    pub word: String,
    pub translation: Option<String>,
    pub date_added: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWordDto {
    pub word: String,
    pub translation: Option<String>,
}
