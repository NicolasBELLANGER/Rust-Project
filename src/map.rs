use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::Rng;

// Bruit (relief)
const FBM_OCTAVES: usize = 4;
const FBM_FREQUENCY: f64 = 2.0;
const FBM_PERSISTENCE: f64 = 0.5;
const FBM_LACUNARITY: f64 = 2.0;
const MAP_SCALE_FACTOR: f64 = 3.0;
const RESOURCE_NOISE_SCALE: f64 = 6.0;
const RESOURCE_LAYER_SEED_OFFSET: u32 = 1;

// Seuils de placement
const OBSTACLE_THRESHOLD: f64 = 0.2;
const RESOURCE_ZONE_THRESHOLD: f64 = -0.15;
const CRYSTAL_THRESHOLD: f64 = 0.45;
const ENERGY_THRESHOLD: f64 = -0.45;
const RESOURCE_SPAWN_CHANCE: f64 = 0.10;

// Ressources
const RESOURCE_MIN: u32 = 50;
const RESOURCE_MAX: u32 = 200;
const MIN_RESOURCES_PER_KIND_DIVISOR: usize = 60;
const PLACE_RESOURCE_MAX_ATTEMPTS: u32 = 200;

// Base
const BASE_SAFE_RADIUS: i32 = 2;

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
            .set_octaves(FBM_OCTAVES)
            .set_frequency(FBM_FREQUENCY)
            .set_persistence(FBM_PERSISTENCE)
            .set_lacunarity(FBM_LACUNARITY);

        let resource_layer = Perlin::new(seed.wrapping_add(RESOURCE_LAYER_SEED_OFFSET));
        let scale = MAP_SCALE_FACTOR / width.max(height) as f64;

        let mut tiles = vec![vec![MapTile::Empty; width]; height];

        // Placement procédural case par case (Perlin)
        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;
                let field_val = field.get([nx, ny]);
                let resource_val =
                    resource_layer.get([nx * RESOURCE_NOISE_SCALE, ny * RESOURCE_NOISE_SCALE]);

                tiles[y][x] = if field_val > OBSTACLE_THRESHOLD {
                    MapTile::Obstacle
                } else if field_val < RESOURCE_ZONE_THRESHOLD
                    && resource_val > CRYSTAL_THRESHOLD
                    && rng.gen_bool(RESOURCE_SPAWN_CHANCE)
                {
                    MapTile::Crystal(rng.gen_range(RESOURCE_MIN..=RESOURCE_MAX))
                } else if field_val < RESOURCE_ZONE_THRESHOLD
                    && resource_val < ENERGY_THRESHOLD
                    && rng.gen_bool(RESOURCE_SPAWN_CHANCE)
                {
                    MapTile::Energy(rng.gen_range(RESOURCE_MIN..=RESOURCE_MAX))
                } else {
                    MapTile::Empty
                };
            }
        }

        let base_x = width / 2;
        let base_y = height / 2;

        // Zone sûre autour de la base
        for dy in -BASE_SAFE_RADIUS..=BASE_SAFE_RADIUS {
            for dx in -BASE_SAFE_RADIUS..=BASE_SAFE_RADIUS {
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

        let target_per_kind = (width * height) / MIN_RESOURCES_PER_KIND_DIVISOR;

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
        for _ in 0..PLACE_RESOURCE_MAX_ATTEMPTS {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if matches!(tiles[y][x], MapTile::Empty) {
                tiles[y][x] = if is_crystal {
                    MapTile::Crystal(rng.gen_range(RESOURCE_MIN..=RESOURCE_MAX))
                } else {
                    MapTile::Energy(rng.gen_range(RESOURCE_MIN..=RESOURCE_MAX))
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
