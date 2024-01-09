#[derive(Debug, PartialEq)]
pub enum PCBState {
    READY,
    BLOCKED,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PCBResource {
    pub rid: usize,
    pub units: usize,
}

#[derive(Debug, PartialEq)]
pub struct PCB {
    pub state: PCBState,
    pub priority: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub resources: Vec<PCBResource>,
}

impl PCB {
    pub fn new(priority: usize, parent: Option<usize>) -> Self {
        Self {
            priority,
            parent,
            state: PCBState::READY,
            children: Vec::new(),
            resources: Vec::new(),
        }
    }
}
