pub enum RobotMessage {
    ResourceFound { pos: (usize, usize), kind: ResourceKind },
    ObstacleFound { pos: (usize, usize) },
    ResourceCollected { pos: (usize, usize), amount: u32 },
    GoCollect { pos: (usize, usize) },
}

pub enum ResourceKind {
    Energy,
    Crystal,
}