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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_example() {
        // given
        let example = Example { id: 1, word_id: 2, example: "test example".to_owned() };

        // when
        let example_response_dto = map_example_to_response(&example);

        // then
        assert_eq!(example_response_dto.id, example.id);
        assert_eq!(example_response_dto.example, example.example);
    }

    #[test]
    fn test_map_word() {
        // given
        let example_string = "test example";
        let example = Example { id: 1, word_id: 2, example: example_string.to_owned() };
        let word = Word {
            id: 2,
            word: "test word".to_owned(),
            translation: Some("test translation".to_owned()),
            source: None,
            date_added: "01.01.2023".to_string(),
        };

        // when
        let vocab_response_dto = map_word_to_response(&word, &vec![example]);

        // then
        assert_eq!(vocab_response_dto.id, word.id);
        assert_eq!(vocab_response_dto.word, word.word);
        assert_eq!(vocab_response_dto.translation, word.translation);
        assert_eq!(vocab_response_dto.source, word.source);
        assert_eq!(vocab_response_dto.examples.len(), 1);
        assert_eq!(vocab_response_dto.examples[0].example, example_string);
        assert_eq!(vocab_response_dto.date_added, word.date_added);
    }
}