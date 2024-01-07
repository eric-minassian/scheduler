pub(crate) struct RCB {
    inventory: u8,
    state: u8,
    waitlist: Vec<i32>,
}

impl RCB {
    pub(crate) fn new(inventory: u8, state: u8, waitlist: Vec<i32>) -> Self {
        Self {
            inventory,
            state,
            waitlist,
        }
    }
}
