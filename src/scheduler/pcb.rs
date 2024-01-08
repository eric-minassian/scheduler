pub enum PCBState {
    READY,
    BLOCKED,
}

pub struct PCB {
    pub state: PCBState,
    pub priority: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub resources: Vec<usize>,
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
