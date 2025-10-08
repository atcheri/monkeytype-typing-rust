#[derive(Debug, PartialEq)]
enum InputResult {
    Dry,
    Error,
    Success,
}

struct Game {
    bg_text: String,
    typed_text: String,
}

impl Game {
    fn input(&mut self, typed_letter: char) -> InputResult {
        if typed_letter == '\u{8}' {
            if self.typed_text.is_empty() {
                return InputResult::Dry;
            }
            self.typed_text.pop();
        } else {
            self.typed_text.push(typed_letter);
        }

        let expected_letter = self.bg_text.chars().nth(self.typed_text.len() - 1).unwrap();
        if typed_letter != expected_letter {
            return InputResult::Error;
        }

        InputResult::Success
    }

    fn get_next_word_position(&mut self) -> usize {
        let curren_pos = self.typed_text.len() - 1;
        let target_string = &self.bg_text[curren_pos..];
        let next_space_pos = target_string.find(' ').unwrap();
        curren_pos + next_space_pos + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_correct_input_in_typed_text() {
        // arrange
        let mut game = Game {
            bg_text: String::from("bonjour"),
            typed_text: String::from(""),
        };

        // act
        let result = game.input('b');

        // assert
        assert_eq!(game.typed_text, String::from("b"));
        assert_eq!(result, InputResult::Success)
    }

    #[test]
    fn check_incorrect_input_in_typed_text() {
        // arrange
        let mut game = Game {
            bg_text: String::from("bonjour"),
            typed_text: String::from(""),
        };

        // act
        let result = game.input('x');

        // assert
        assert_eq!(game.typed_text, String::from("x"));
        assert_eq!(result, InputResult::Error)
    }

    #[test]
    fn removes_previous_character_when_backspace_key_stroke() {
        // arrange
        let mut game = Game {
            bg_text: String::from("bonjour"),
            typed_text: String::from("bon"),
        };

        // act
        let result = game.input('\u{8}');

        // assert
        assert_eq!(game.typed_text, String::from("bo"));
        assert_eq!(result, InputResult::Error)
    }

    #[test]
    fn prevent_backspace_on_first_character() {
        // arrange
        let mut game = Game {
            bg_text: String::from("bonjour"),
            typed_text: String::from(""),
        };

        // act
        let result = game.input('\u{8}');

        // assert
        assert_eq!(game.typed_text, String::from(""));
        assert_eq!(result, InputResult::Dry);
    }

    #[test]
    fn verifies_first_next_word_position() {
        // arrange
        let mut game = Game {
            bg_text: String::from("bonjour tout"),
            typed_text: String::from("b"),
        };

        // act
        let position = game.get_next_word_position();

        // assert
        assert_eq!(8, position)
    }

    #[test]
    fn verifies_second_next_word_position() {
        // arrange
        let mut game = Game {
            bg_text: String::from("bonjour tout le monde"),
            typed_text: String::from("bonjour t"),
        };

        // act
        let position = game.get_next_word_position();

        // assert
        assert_eq!(13, position)
    }
}
