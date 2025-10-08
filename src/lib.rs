pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug, PartialEq)]
enum InputResult {
    Error,
    Success,
}

struct Game {
    bg_test: String,
    typed_text: String,
}

impl Game {
    fn input(&mut self, typed_letter: char) -> InputResult {
        self.typed_text = String::from(typed_letter);
        let expected_letter = self.bg_test.chars().nth(self.typed_text.len() - 1).unwrap();

        if typed_letter != expected_letter {
            return InputResult::Error;
        }

        InputResult::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_left_2_and_right_2_it_returns_4() {
        // arrange
        let expected: usize = 4;

        // act
        let result = add(2, 2);

        // assert
        assert_eq!(result, expected);
    }

    #[test]
    fn check_correct_input_in_typed_text() {
        // arrange
        let mut game = Game {
            bg_test: String::from("bonjour"),
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
            bg_test: String::from("bonjour"),
            typed_text: String::from(""),
        };

        // act
        let result = game.input('x');

        // assert
        assert_eq!(game.typed_text, String::from("x"));
        assert_eq!(result, InputResult::Error)
    }
}
