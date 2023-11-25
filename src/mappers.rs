use crate::dtos::{ExampleResponseDto, VocabResponseDto};
use crate::models::Example;
use crate::models::Word;

pub fn map_word_to_response(word: &Word, word_examples: &[Example]) -> VocabResponseDto {
    VocabResponseDto {
        id: word.id,
        word: word.word.to_owned(),
        translation: word.translation.to_owned(),
        source: word.source.to_owned(),
        examples: word_examples
            .iter()
            .map(map_example_to_response)
            .collect(),
        date_added: word.date_added.and_utc().to_rfc3339(),
    }
}

pub fn map_example_to_response(example: &Example) -> ExampleResponseDto {
    ExampleResponseDto {
        id: example.id,
        example: example.example.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDateTime, Utc};
    use uuid::Uuid;

    #[test]
    fn should_map_example() {
        // given
        let example = Example {
            id: Uuid::new_v4(),
            word_id: Uuid::new_v4(),
            example: "test example".to_owned(),
        };

        // when
        let example_response_dto = map_example_to_response(&example);

        // then
        assert_eq!(example_response_dto.id, example.id);
        assert_eq!(example_response_dto.example, example.example);
    }

    #[test]
    fn should_map_word() {
        // given
        let example_string = "test example";
        let word_id = Uuid::new_v4();
        let example = Example {
            id: Uuid::new_v4(),
            word_id,
            example: example_string.to_owned(),
        };
        let word = Word {
            id: word_id,
            word: "test word".to_owned(),
            translation: Some("test translation".to_owned()),
            source: None,
            date_added: Utc::now().naive_utc(),
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
    }

    #[test]
    fn should_add_utc_timezone_to_date_added() {
        // given
        let date_added =
            NaiveDateTime::parse_from_str("2023-01-30 23:52:04", "%Y-%m-%d %H:%M:%S").unwrap();
        let word = Word {
            id: Uuid::new_v4(),
            word: "test word".to_owned(),
            translation: Some("test translation".to_owned()),
            source: None,
            date_added,
        };
        let expected_date_iso_string = "2023-01-30T23:52:04+00:00";

        // when
        let vocab_response_dto = map_word_to_response(&word, &vec![]);

        // then
        assert_eq!(vocab_response_dto.date_added, expected_date_iso_string);
    }
}
