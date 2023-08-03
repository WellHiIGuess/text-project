use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree, entry::Entry};

pub struct Game {
    pub world: Quadtree<i64, char>,
    camera_dimensions: (i64, i64),
}

impl Game {
    pub fn create() -> Self {
        Self {
            world: Quadtree::<i64, char>::new(7),
            camera_dimensions: (20, 10),
        }
    }

    fn add_entry(screen: &mut Vec<Vec<char>>, entry: &Entry<i64, char>) {
        for y in 0..entry.height() {
            for x in 0..entry.width() {
                if entry.anchor().y() + y < screen.len() as i64 && entry.anchor().x() + x < screen[0].len() as i64 {
                    screen[(entry.anchor().y() + y) as usize][(entry.anchor().x() + x) as usize] = *entry.value_ref();
                }
            }
        }
    }

    pub fn draw(&mut self) {
        // Clears terminal
        print!("{}[2J", 27 as char);

        let camera = AreaBuilder::default()
            .anchor(Point { x: 0, y: 0 })
            .dimensions(self.camera_dimensions)
            .build()
            .unwrap();

        let view = self.world.query(camera);

        let mut screen = vec![
            vec![' '; self.camera_dimensions.0 as usize]; self.camera_dimensions.1 as usize
        ];

        for i in view {
            Self::add_entry(&mut screen, i);
        }

        for y in 0..screen.len() {
            for x in 0..screen[y].len() {
                print!("{}", screen[y][x]);
            }
            println!();
        }

        self.world.reset();
    }
}
