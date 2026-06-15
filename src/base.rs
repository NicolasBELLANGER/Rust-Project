use crate::messages::{RobotMessage, ResourceKind};
use crate::app::App;
use crate::map::Map;

pub struct Base {
    pub pos: (usize, usize),
}

impl Base {
    pub fn new(map: Map) -> Self {
        Base {
            pos: (map.width / 2, map.height / 2),
        }
    }

    pub fn messages(&self, app: &mut App, messages: RobotMessage) {
        match messages {
            RobotMessage::ResourceFound { pos, kind } => {
                app.known_resources.push((pos.0, pos.1, kind));
            },
            RobotMessage::ObstacleFound { pos } => {
                app.known_obstacles.push((pos.0, pos.1));
            },
            RobotMessage::BasePosition { pos } => {
                app.base_position = pos;
            },
            RobotMessage::ResourceCollected { pos: _, amount, kind } => {
                match kind {
                    ResourceKind::Energy  => app.collected_energy += amount,
                    ResourceKind::Crystal => app.collected_crystals += amount,
                }
            }
            RobotMessage::GoCollect { pos: _ } => {}
        }
    }
}