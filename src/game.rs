use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree, entry::Entry};

pub struct Game {
    world: Quadtree<u64, char>,
}

impl Game {
    pub fn create() -> Self {
        Self {
            world: Quadtree::<u64, char>::new(7),
        }
    }

    fn add_entry(screen: &mut Vec<Vec<char>>, entry: &Entry<u64, char>) {
        for y in 0..entry.height() {
            for x in 0..entry.width() {
                screen[(entry.anchor().y() + y) as usize][(entry.anchor().x() + x) as usize] = *entry.value_ref();
            }
        }
    }

    pub fn draw(&mut self) {
        // self.clear();
        let region = AreaBuilder::default()
            .anchor(Point { x: 0, y: 0 })
            .dimensions((3, 3))
            .build()
            .unwrap();
        self.world.insert(region, 'r');

        let region_2 = AreaBuilder::default()
            .anchor(Point { x: 1, y: 1 })
            .dimensions((3, 3))
            .build()
            .unwrap();
        self.world.insert(region_2, 'o');

        let camera = AreaBuilder::default()
            .anchor(Point { x: 0, y: 0 })
            .dimensions((10, 10))
            .build()
            .unwrap();

        let view = self.world.query(camera);

        let mut screen = vec![vec![' '; 10]; 10];

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
