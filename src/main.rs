use game::Game;

mod game;

fn main() {
    let mut game_data = Game::create();
    game_data.draw();
}
