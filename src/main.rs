use game::Game;
use quadtree_rs::{area::AreaBuilder, point::Point};
use device_query::{DeviceQuery, DeviceState, Keycode};

mod game;

fn control_frame_rate(fps: f32) {
    use std::process::Command;

    let wait_time = 1.0 / fps;
    let mut child = Command::new("sleep").arg(wait_time.to_string().as_str()).spawn().unwrap();
    let _result = child.wait().unwrap();
}

fn main() {
    let mut game_data = Game::create();

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    
    use std::time::SystemTime;
    loop {
        let now = SystemTime::now();
        control_frame_rate(25.0);

        let player = AreaBuilder::default()
            .anchor(Point{ x: x as i64, y: y as i64 })
            .dimensions((2, 2))
            .build()
            .unwrap();
        
        game_data.world.insert(player, 'p');

        game_data.draw();

        let device_state = DeviceState::new();
        let keys = device_state.get_keys();

        for key in keys.iter() {
            if key == &Keycode::S {
                y += 10.0 * now.elapsed().unwrap().as_secs_f32();
            }

            if key == &Keycode::W {
                y -= 10.0 * now.elapsed().unwrap().as_secs_f32();
            }

            if key == &Keycode::D {
                x += 10.0 * now.elapsed().unwrap().as_secs_f32();
            }

            if key == &Keycode::A {
                x -= 10.0 * now.elapsed().unwrap().as_secs_f32();
            }
        }
    }
}
