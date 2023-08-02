use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};

pub struct Game {
    world: Quadtree<u64, char>,
}

impl Game {
    pub fn create() -> Self {
        Self {
            world: Quadtree::<u64, char>::new(7),
        }
    }

    pub fn clear(&mut self) {
        let region = AreaBuilder::default()
            .anchor(Point { x: 0, y: 0 })
            .dimensions((2, 1))
            .build()
            .unwrap();
        self.world.insert(region, ' ');
    }

    pub fn draw(&mut self) {
        let region = AreaBuilder::default()
            .anchor(Point { x: 0, y: 0 })
            .dimensions((3, 3))
            .build()
            .unwrap();
        self.world.insert(region, 'o');

        let region_2 = AreaBuilder::default()
            .anchor(Point { x: 5, y: 0 })
            .dimensions((3, 3))
            .build()
            .unwrap();
        self.world.insert(region_2, 'o');

        let camera = AreaBuilder::default()
            .anchor(Point { x: 0, y: 0 })
            .dimensions((10, 10))
            .build()
            .unwrap();

        let view = &self
            .world
            .query(camera)
            .map(|x| x.anchor().x)
            .collect::<Vec<_>>();

        for x in 0..10 {
            
        }
    }
}
