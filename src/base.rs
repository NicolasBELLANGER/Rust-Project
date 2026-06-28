use crate::messages::{RobotMessage, ResourceKind};
use crate::app::App;
use crate::map::Map;

pub struct Base {
    pub pos: (usize, usize),
}

impl Base {
    pub fn new(map: &Map) -> Self {
        Base {
            pos: (map.width / 2, map.height / 2),
        }
    }

    /// Centre de communication : agrège les découvertes et les collectes.
    pub fn handle_message(app: &mut App, message: RobotMessage) {
        match message {
            RobotMessage::ResourceFound { pos, kind } => {
                let already_known = app
                    .known_resources
                    .iter()
                    .any(|(x, y, _)| *x == pos.0 && *y == pos.1);
                if !already_known {
                    app.known_resources.push((pos.0, pos.1, kind));
                }
            }
            RobotMessage::ObstacleFound { pos } => {
                if !app.known_obstacles.contains(&(pos.0, pos.1)) {
                    app.known_obstacles.push(pos);
                }
            }
            RobotMessage::ResourceCollected { amount, kind, .. } => match kind {
                ResourceKind::Energy => app.collected_energy += amount,
                ResourceKind::Crystal => app.collected_crystals += amount,
            },
            RobotMessage::GoCollect { .. } => {}
        }
    }
}
