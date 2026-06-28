use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use rand::Rng;

use crate::app::SharedApp;
use crate::map::MapTile;
use crate::messages::{ResourceKind, RobotMessage};

pub struct Scout {
    id: usize,
    pos: (usize, usize),
}

impl Scout {
    pub fn new(id: usize, started_pos: (usize, usize)) -> Self {
        Scout { id, pos: started_pos }
    }

    pub fn run(mut self, shared: SharedApp, tx: Sender<RobotMessage>) {
        let mut rng = rand::thread_rng();
        loop {
            {
                let app = shared.lock().unwrap();
                if !app.active {
                    break;
                }
            }

            // Mélanger les 4 directions pour explorer aléatoirement
            let mut directions = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
            for i in (1..directions.len()).rev() {
                let j = rng.gen_range(0..=i);
                directions.swap(i, j);
            }

            for (dx, dy) in directions {
                let new_x = self.pos.0 as i32 + dx;
                let new_y = self.pos.1 as i32 + dy;

                let mut app = shared.lock().unwrap();

                if new_x < 0
                    || new_x >= app.map.width as i32
                    || new_y < 0
                    || new_y >= app.map.height as i32
                {
                    continue;
                }

                let new_pos = (new_x as usize, new_y as usize);

                // Obstacle physique rencontré → le partager aux autres robots
                if matches!(app.map.tiles[new_pos.1][new_pos.0], MapTile::Obstacle) {
                    if !app.known_obstacles.contains(&new_pos) {
                        tx.send(RobotMessage::ObstacleFound { pos: new_pos }).ok();
                    }
                    continue;
                }

                // Éviter les obstacles déjà connus (partagés via messages)
                if app.known_obstacles.contains(&new_pos) {
                    continue;
                }

                if !app.map.is_walkable(new_pos.0, new_pos.1) {
                    continue;
                }

                self.pos = new_pos;
                if self.id < app.scouts.len() {
                    app.scouts[self.id] = self.pos;
                }

                match &app.map.tiles[new_pos.1][new_pos.0] {
                    MapTile::Energy(_) => {
                        tx.send(RobotMessage::ResourceFound {
                            pos: new_pos,
                            kind: ResourceKind::Energy,
                        })
                        .ok();
                    }
                    MapTile::Crystal(_) => {
                        tx.send(RobotMessage::ResourceFound {
                            pos: new_pos,
                            kind: ResourceKind::Crystal,
                        })
                        .ok();
                    }
                    _ => {}
                }
                break;
            }

            thread::sleep(Duration::from_millis(200));
        }
    }
}
