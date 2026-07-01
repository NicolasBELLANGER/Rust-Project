use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::Rng;

#[derive(Debug, Clone)]
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
        let seed: u32 = rand::random();
        let mut rng = rand::thread_rng();

        let field = Fbm::<Perlin>::new(seed)
            .set_octaves(4)
            .set_frequency(2.0)
            .set_persistence(0.5)
            .set_lacunarity(2.0);

        let resource_layer = Perlin::new(seed.wrapping_add(1));
        let scale = 3.0 / width.max(height) as f64;

        let mut tiles = vec![vec![MapTile::Empty; width]; height];

        // Ressources via Perlin ; obstacles ajoutés ensuite en îlots compacts
        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;
                let field_val = field.get([nx, ny]);
                let resource_val = resource_layer.get([nx * 6.0, ny * 6.0]);

                tiles[y][x] = if field_val > 0.2 {
                    MapTile::Obstacle
                } else if field_val < -0.15 && resource_val > 0.45 && rng.gen_bool(0.10) {
                    MapTile::Crystal(rng.gen_range(50..=200))
                } else if field_val < -0.15 && resource_val < -0.45 && rng.gen_bool(0.10) {
                    MapTile::Energy(rng.gen_range(50..=200))
                } else {
                    MapTile::Empty
                };
            }
        }

        let base_x = width / 2;
        let base_y = height / 2;

        // Zone sûre autour de la base
        for dy in -2i32..=2 {
            for dx in -2i32..=2 {
                let cx = (base_x as i32 + dx).clamp(0, width as i32 - 1) as usize;
                let cy = (base_y as i32 + dy).clamp(0, height as i32 - 1) as usize;
                tiles[cy][cx] = MapTile::Empty;
            }
        }
        tiles[base_y][base_x] = MapTile::Base;

        let energy_count = tiles
            .iter()
            .flatten()
            .filter(|t| matches!(t, MapTile::Energy(_)))
            .count();
        let crystal_count = tiles
            .iter()
            .flatten()
            .filter(|t| matches!(t, MapTile::Crystal(_)))
            .count();

        let target_per_kind = (width * height) / 60;

        for _ in energy_count..target_per_kind {
            Self::place_resource_randomly(&mut tiles, width, height, &mut rng, false);
        }
        for _ in crystal_count..target_per_kind {
            Self::place_resource_randomly(&mut tiles, width, height, &mut rng, true);
        }

        Map {
            tiles,
            width,
            height,
        }
    }

    fn place_resource_randomly(
        tiles: &mut Vec<Vec<MapTile>>,
        width: usize,
        height: usize,
        rng: &mut impl Rng,
        is_crystal: bool,
    ) {
        for _ in 0..200 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if matches!(tiles[y][x], MapTile::Empty) {
                tiles[y][x] = if is_crystal {
                    MapTile::Crystal(rng.gen_range(50..=200))
                } else {
                    MapTile::Energy(rng.gen_range(50..=200))
                };
                return;
            }
        }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        !matches!(self.tiles[y][x], MapTile::Obstacle)
    }
}
