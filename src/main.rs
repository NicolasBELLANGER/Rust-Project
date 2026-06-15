mod map;
mod messages;
mod base;
mod app;

use app::new_shared;
use messages::{RobotMessage, ResourceKind};

fn main() {
    let shared = new_shared(20, 20);
    {
        let app = shared.lock().unwrap();
        println!("Carte créée : {}x{}", app.map.width, app.map.height);
        println!("Base position : {:?}", app.base.pos);
        println!("Énergie collectée : {}", app.collected_energy);
        println!("Cristaux collectés : {}", app.collected_crystals);
    }
}
