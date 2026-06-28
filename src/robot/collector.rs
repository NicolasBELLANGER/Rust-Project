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
        kind: ResourceKind,
        path: Vec<(usize, usize)>,
    },
    Returning {
        target: (usize, usize),
        kind: ResourceKind,
        path: Vec<(usize, usize)>,
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
                        let assignment = app
                            .known_resources
                            .iter()
                            .find(|(x, y, _)| !app.claimed_resources.contains(&(*x, *y)))
                            .map(|(x, y, kind)| ((*x, *y), *kind));

                        if let Some((target_pos, kind)) = assignment {
                            if let Some(path) = bfs(&app, self.pos, target_pos) {
                                app.claimed_resources.push(target_pos);
                                next_state = Some(State::GoingTo {
                                    target: target_pos,
                                    kind,
                                    path,
                                });
                            }
                        }
                    }

                    State::GoingTo { target, kind: _, path } => {
                        let target = *target;

                        if let Some(&next) = path.first() {
                            if app.map.is_walkable(next.0, next.1) {
                                path.remove(0);
                                self.pos = next;
                                if self.id < app.collectors.len() {
                                    app.collectors[self.id] = self.pos;
                                }
                            }
                        }

                        if self.pos == target {
                            let tile = app.map.tiles[target.1][target.0].clone();
                            if let Some((new_tile, collected_kind)) = take_one_unit(&tile) {
                                app.map.tiles[target.1][target.0] = new_tile.clone();
                                if matches!(new_tile, MapTile::Empty) {
                                    app.known_resources
                                        .retain(|(x, y, _)| !(*x == target.0 && *y == target.1));
                                }

                                let base_pos = app.base.pos;
                                let return_path = bfs(&app, self.pos, base_pos).unwrap_or_default();
                                next_state = Some(State::Returning {
                                    target,
                                    kind: collected_kind,
                                    path: return_path,
                                });
                            } else {
                                app.claimed_resources.retain(|&p| p != target);
                                next_state = Some(State::Idle);
                            }
                        }
                    }

                    State::Returning { target, kind, path } => {
                        let target = *target;
                        let kind = *kind;

                        if let Some(&next) = path.first() {
                            if app.map.is_walkable(next.0, next.1) {
                                path.remove(0);
                                self.pos = next;
                                if self.id < app.collectors.len() {
                                    app.collectors[self.id] = self.pos;
                                }
                            }
                        }

                        if path.is_empty() {
                            tx.send(RobotMessage::ResourceCollected {
                                pos: self.pos,
                                amount: 1,
                                kind,
                            })
                            .ok();

                            if resource_remaining(&app.map.tiles[target.1][target.0]) {
                                if let Some(path) = bfs(&app, self.pos, target) {
                                    next_state = Some(State::GoingTo { target, kind, path });
                                } else {
                                    app.claimed_resources.retain(|&p| p != target);
                                    next_state = Some(State::Idle);
                                }
                            } else {
                                app.known_resources
                                    .retain(|(x, y, _)| !(*x == target.0 && *y == target.1));
                                app.claimed_resources.retain(|&p| p != target);
                                next_state = Some(State::Idle);
                            }
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

fn take_one_unit(tile: &MapTile) -> Option<(MapTile, ResourceKind)> {
    match tile {
        MapTile::Energy(n) if *n > 0 => {
            let remaining = n - 1;
            let new_tile = if remaining == 0 {
                MapTile::Empty
            } else {
                MapTile::Energy(remaining)
            };
            Some((new_tile, ResourceKind::Energy))
        }
        MapTile::Crystal(n) if *n > 0 => {
            let remaining = n - 1;
            let new_tile = if remaining == 0 {
                MapTile::Empty
            } else {
                MapTile::Crystal(remaining)
            };
            Some((new_tile, ResourceKind::Crystal))
        }
        _ => None,
    }
}

fn resource_remaining(tile: &MapTile) -> bool {
    matches!(tile, MapTile::Energy(n) if *n > 0)
        || matches!(tile, MapTile::Crystal(n) if *n > 0)
}

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
