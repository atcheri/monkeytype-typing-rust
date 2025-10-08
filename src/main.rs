use game::Game;

fn main() {
    let mut game = Game {
        bg_text: String::from("Lorem ipsum dolor sit amet"),
        typed_text: String::from(""),
    };

    game.input('L');
    game.input('o');
    game.input('r');
    game.input('e');
    game.input(' ');
    game.input('i');
    game.input(' ');
    game.input('d');

    game.show();
}
