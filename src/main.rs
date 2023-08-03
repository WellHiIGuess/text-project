use game::Game;

mod game;

fn main() {
    let mut game_data = Game::create();
    
    loop {
        game_data.draw();
    }
}
