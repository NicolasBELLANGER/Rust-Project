use crate::map::Map;
use crate::messages::ResourceKind;
use crate::base::Base;

use std::sync::{Arc, Mutex};

pub type SharedApp = Arc<Mutex<App>>;

pub fn new_shared(width: usize, height: usize) -> SharedApp {
    Arc::new(Mutex::new(App::new(width, height)))
}

pub struct App {
    pub map: Map,
    pub base: Base,
    pub collected_energy: u32,
    pub collected_crystals: u32,
    pub known_resources: Vec<(usize, usize, ResourceKind)>,
    pub known_obstacles: Vec<(usize, usize)>,
    pub active: bool,
    pub scouts: Vec<(usize, usize)>,
    pub collectors: Vec<(usize, usize)>,
    pub claimed_resources: Vec<(usize, usize)>,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        let map = Map::new(width, height);
        let base = Base::new(&map);
        App {
            map,
            base,
            collected_energy: 0,
            collected_crystals: 0,
            known_resources: Vec::new(),
            known_obstacles: Vec::new(),
            active: true,
            scouts: Vec::new(),
            collectors: Vec::new(),
            claimed_resources: Vec::new(),
        }
    }
}
