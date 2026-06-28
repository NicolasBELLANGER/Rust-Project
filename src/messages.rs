pub enum RobotMessage {
    ResourceFound { pos: (usize, usize), kind: ResourceKind },
    ObstacleFound { pos: (usize, usize) },
    ResourceCollected { pos: (usize, usize), amount: u32, kind: ResourceKind },
    GoCollect { pos: (usize, usize) },
}

#[derive(Clone, Copy, Debug)]
pub enum ResourceKind {
    Energy,
    Crystal,
}