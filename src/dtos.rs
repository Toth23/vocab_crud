use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VocabResponseDto {
    pub id: i32,
    pub word: String,
    pub translation: Option<String>,
    pub source: Option<String>,
    pub examples: Vec<ExampleResponseDto>,
    pub date_added: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExampleResponseDto {
    pub id: i32,
    pub example: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWordDto {
    pub word: String,
    pub translation: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateExampleDto {
    pub example: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWordDto {
    pub word: String,
    pub translation: Option<String>,
    pub source: Option<String>,
}
