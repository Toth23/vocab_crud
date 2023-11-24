use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct VocabResponseDto {
    pub id: Uuid,
    pub word: String,
    pub translation: Option<String>,
    pub source: Option<String>,
    pub examples: Vec<ExampleResponseDto>,
    pub date_added: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExampleResponseDto {
    pub id: Uuid,
    pub example: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateWordDto {
    pub word: String,
    pub translation: Option<String>,
    pub source: Option<String>,
    #[serde(default)]
    pub examples: Vec<String>,
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
