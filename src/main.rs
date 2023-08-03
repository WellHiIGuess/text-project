use game::Game;
use quadtree_rs::{area::AreaBuilder, point::Point};
use device_query::{DeviceQuery, DeviceState, Keycode};

mod game;

fn main() {
    let mut game_data = Game::create();

    let mut x: f32 = 0.0;
    
    use std::process::Command;
    use std::time::SystemTime;
    loop {
        let now = SystemTime::now();

        // Controls frame rate
        let mut child = Command::new("sleep").arg("0.1").spawn().unwrap();
        let _result = child.wait().unwrap();

        let player = AreaBuilder::default()
            .anchor(Point{ x: x as u64, y: 0 })
            .dimensions((2, 2))
            .build()
            .unwrap();
        
        game_data.world.insert(player, 'p');

        game_data.draw();

        let device_state = DeviceState::new();
        let keys = device_state.get_keys();

        for key in keys.iter() {
            if key == &Keycode::D {
                x += 1.0 * now.elapsed().unwrap().as_secs_f32();
            }

            if key == &Keycode::A {
                x -= 1.0 * now.elapsed().unwrap().as_secs_f32();
            }
        }

        println!("{}", now.elapsed().unwrap().as_secs_f32());

    }
}
