pub struct RCB {
    pub inventory: usize,
    pub units_available: usize,
    pub waitlist: Vec<usize>,
}

impl RCB {
    pub fn new(inventory: usize) -> Self {
        Self {
            inventory,
            units_available: inventory,
            waitlist: Vec::new(),
        }
    }
}
