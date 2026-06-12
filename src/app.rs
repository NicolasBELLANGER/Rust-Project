use crate::map::Map;
use crate::messages::ResourceKind;

pub struct App {
    pub map: Map,
    pub base: Base,
    pub collected_energy: u32,
    pub collected_crystals: u32,
    pub known_resources: Vec<(usize, usize, ResourceKind)>,
    pub known_obstacles: Vec<(usize, usize)>,
    pub active: bool,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        App {
            map: Map::new(width, height),
            collected_energy: 0,
            collected_crystals: 0,
            known_resources: Vec::new(),
            known_obstacles: Vec::new(),
            active: true,
        }
    }
}
