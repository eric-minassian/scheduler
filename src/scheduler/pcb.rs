pub(crate) enum PCBState {
    READY,
    BLOCKED,
}

pub(crate) struct PCB {
    state: PCBState,
    priority: u8,
    parent: i32,
    children: Vec<i32>,
    resources: Vec<i32>,
}

impl PCB {
    pub(crate) fn new(
        state: PCBState,
        priority: u8,
        parent: i32,
        children: Vec<i32>,
        resources: Vec<i32>,
    ) -> Self {
        Self {
            state,
            priority,
            parent,
            children,
            resources,
        }
    }
}
