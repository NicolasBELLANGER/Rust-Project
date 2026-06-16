use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use crate::map::MapTile;
use crate::app::SharedApp;
use crate::messages::{RobotMessage, ResourceKind};
use rand::Rng;

pub struct Scout {
    pos: (usize, usize),
}

impl Scout {
    pub fn new(started_pos: (usize, usize)) -> Self {
        Scout {
            pos: started_pos,
        }
    }

    pub fn run(mut self, shared: SharedApp, tx: Sender<RobotMessage>) {
        // Crée un générateur de nombres aléatoires
        let mut rng = rand::thread_rng();
        loop{
            // Vérifie si l'application est active
            {
                //lock() permet de prendre l'état partagé
                let app = shared.lock().unwrap();
                // Vérifie si l'application est active
                if !app.active {
                    break;
                }
            }

            // 4 directions possibles : bas, haut, droite, gauche
            let directions = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
            // Déplacement entre x et y
            // Choisis une direction aléatoire entre les 4 directions
            let (dx, dy) = directions[rng.gen_range(0..4)];
            let new_x = self.pos.0 as i32 + dx;
            let new_y = self.pos.1 as i32 + dy;

            {
                let app = shared.lock().unwrap();
                // Vérifie si la nouvelle position est dans les limites de la carte
                if new_x >= 0 && new_x < app.map.width as i32 && new_y >= 0 && new_y < app.map.height as i32 {
                    let new_pos = (new_x as usize, new_y as usize);
                    // Vérifie si la nouvelle position est marchable
                    if app.map.is_walkable(new_pos.0, new_pos.1) {
                        self.pos = new_pos;
                        match &app.map.tiles[new_pos.1][new_pos.0] {
                            MapTile::Energy(_) => {
                                tx.send(RobotMessage::ResourceFound { pos: new_pos, kind: ResourceKind::Energy }).ok();
                            },
                            MapTile::Crystal(_) => {
                                tx.send(RobotMessage::ResourceFound { pos: new_pos, kind: ResourceKind::Crystal }).ok();
                            },
                            MapTile::Obstacle => {
                                tx.send(RobotMessage::ObstacleFound { pos: new_pos }).ok();
                            },
                            _ => {}
                        }
                    }
                }
            }
            thread::sleep(Duration::from_millis(200));
        }

    }
}


