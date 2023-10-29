use crate::dtos::{ExampleResponseDto, VocabResponseDto};
use crate::models::Example;
use crate::models::Word;

pub fn map_word_to_response(word: &Word, word_examples: &Vec<Example>) -> VocabResponseDto {
    VocabResponseDto {
        id: word.id,
        word: word.word.to_owned(),
        translation: word.translation.to_owned(),
        source: word.source.to_owned(),
        examples: word_examples.into_iter().map(map_example_to_response).collect(),
        date_added: word.date_added.to_owned(),
    }
}

pub fn map_example_to_response(example: &Example) -> ExampleResponseDto {
    ExampleResponseDto { id: example.id, example: example.example.to_owned() }
}
