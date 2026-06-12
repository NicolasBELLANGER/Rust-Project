use noise::{NoiseFn, Perlin};
use rand::Rng;

pub enum MapTile {
    Empty,
    Obstacle,
    Energy(u32),
    Crystal(u32),
    Base,
}

pub struct Map {
    pub tiles: Vec<Vec<MapTile>>,
    pub width: usize,
    pub height: usize,
}


impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let perlin = Perlin::new(rand::random());
        let mut rng = rand::thread_rng();

        let mut tiles = vec![vec![MapTile::Empty; width]; height];

        for y in 0..height {
            for x in 0..width {
                let noise_val = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
                if noise_val > 0.3 {
                    tiles[y][x] = MapTile::Obstacle;
                }
            }
        }

        let base_x = width / 2;
        let base_y = height / 2;
        tiles[base_y][base_x] = MapTile::Base;

        let mut placed = 0;
        while placed < 10 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if matches!(tiles[y][x], MapTile::Empty) {
                if placed < 5 {
                    tiles[y][x] = MapTile::Energy(rng.gen_range(50..=200));
                } else {
                    tiles[y][x] = MapTile::Crystal(rng.gen_range(50..=200));
                }
                placed += 1;
            }
        }
        
        Map { tiles, width, height }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        !matches!(self.tiles[y][x], MapTile::Obstacle)
    }
}

