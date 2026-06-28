use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::app::{App, SharedApp};
use crate::map::MapTile;
use crate::messages::{ResourceKind, RobotMessage};

enum State {
    Idle,
    GoingTo {
        target: (usize, usize),
        path: Vec<(usize, usize)>,
    },
    Returning {
        path: Vec<(usize, usize)>,
        amount: u32,
        kind: ResourceKind,
    },
}

pub struct Collector {
    id: usize,
    pos: (usize, usize),
}

impl Collector {
    pub fn new(id: usize, started_pos: (usize, usize)) -> Self {
        Collector { id, pos: started_pos }
    }

    pub fn run(mut self, shared: SharedApp, tx: Sender<RobotMessage>) {
        let mut state = State::Idle;

        loop {
            thread::sleep(Duration::from_millis(300));

            let mut next_state: Option<State> = None;

            {
                let mut app = shared.lock().unwrap();

                if !app.active {
                    break;
                }

                match &mut state {
                    State::Idle => {
                        // Trouver la première ressource connue et non revendiquée
                        let target = app
                            .known_resources
                            .iter()
                            .find(|(x, y, _)| !app.claimed_resources.contains(&(*x, *y)))
                            .map(|(x, y, _)| (*x, *y));

                        if let Some(target_pos) = target {
                            if let Some(path) = bfs(&app, self.pos, target_pos) {
                                app.claimed_resources.push(target_pos);
                                next_state = Some(State::GoingTo { target: target_pos, path });
                            }
                        }
                    }

                    State::GoingTo { target, path } => {
                        let target = *target;

                        // Avancer d'un pas
                        if let Some(&next) = path.first() {
                            if app.map.is_walkable(next.0, next.1) {
                                path.remove(0);
                                self.pos = next;
                                if self.id < app.collectors.len() {
                                    app.collectors[self.id] = self.pos;
                                }
                            }
                        }

                        // Arrivé à la ressource ?
                        if self.pos == target {
                            let collected = match app.map.tiles[target.1][target.0] {
                                MapTile::Energy(a) => Some((a, ResourceKind::Energy)),
                                MapTile::Crystal(a) => Some((a, ResourceKind::Crystal)),
                                _ => None,
                            };

                            if let Some((amount, kind)) = collected {
                                app.map.tiles[target.1][target.0] = MapTile::Empty;
                                app.known_resources
                                    .retain(|(x, y, _)| !(*x == target.0 && *y == target.1));
                                app.claimed_resources.retain(|&p| p != target);

                                let base_pos = app.base.pos;
                                let return_path = bfs(&app, self.pos, base_pos).unwrap_or_default();
                                next_state = Some(State::Returning { path: return_path, amount, kind });
                            } else {
                                // Ressource déjà ramassée par un autre
                                app.claimed_resources.retain(|&p| p != target);
                                next_state = Some(State::Idle);
                            }
                        }
                    }

                    State::Returning { path, amount, kind } => {
                        let amount = *amount;
                        let kind = *kind;

                        // Avancer d'un pas vers la base
                        if let Some(&next) = path.first() {
                            if app.map.is_walkable(next.0, next.1) {
                                path.remove(0);
                                self.pos = next;
                                if self.id < app.collectors.len() {
                                    app.collectors[self.id] = self.pos;
                                }
                            }
                        }

                        // Chemin terminé = arrivée à la base
                        if path.is_empty() {
                            tx.send(RobotMessage::ResourceCollected {
                                pos: self.pos,
                                amount,
                                kind,
                            })
                            .ok();
                            next_state = Some(State::Idle);
                        }
                    }
                }
            }

            if let Some(ns) = next_state {
                state = ns;
            }
        }
    }
}

// BFS simple sur la grille pour trouver le plus court chemin
fn bfs(app: &App, start: (usize, usize), goal: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    if start == goal {
        return Some(vec![]);
    }

    let mut queue = VecDeque::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    queue.push_back(start);
    came_from.insert(start, start);

    while let Some(pos) = queue.pop_front() {
        if pos == goal {
            let mut path = vec![];
            let mut cur = pos;
            while cur != start {
                path.push(cur);
                cur = came_from[&cur];
            }
            path.reverse();
            return Some(path);
        }

        for (dx, dy) in [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
            let nx = pos.0 as i32 + dx;
            let ny = pos.1 as i32 + dy;
            if nx >= 0 && nx < app.map.width as i32 && ny >= 0 && ny < app.map.height as i32 {
                let next = (nx as usize, ny as usize);
                if (app.map.is_walkable(next.0, next.1) || next == goal)
                    && !came_from.contains_key(&next)
                {
                    came_from.insert(next, pos);
                    queue.push_back(next);
                }
            }
        }
    }

    None
}
