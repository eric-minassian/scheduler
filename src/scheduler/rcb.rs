#[derive(Debug, PartialEq, Eq)]
pub struct RCBResource {
    pub pid: usize,
    pub units: usize,
}

#[derive(Debug, PartialEq, Eq)]

pub struct RCB {
    pub inventory: usize,
    pub units_available: usize,
    pub waitlist: Vec<RCBResource>,
}

impl RCB {
    #[must_use] pub fn new(inventory: usize) -> Self {
        Self {
            inventory,
            units_available: inventory,
            waitlist: Vec::new(),
        }
    }
}
