pub enum PCBState {
    READY,
    BLOCKED,
}

pub struct PCBResource {
    pub rid: usize,
    pub units: usize,
}

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
