use crate::scheduler::pcb::{PCBState, PCB};
use crate::scheduler::RCB;

pub fn pcb_list_default() -> [Option<PCB>; 16] {
    [
        Some(PCB::new(PCBState::READY, 0, -1, Vec::new(), Vec::new())),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ]
}

pub fn rcb_list_default() -> [Option<RCB>; 4] {
    [
        Some(RCB::new(1, 1, Vec::new())),
        Some(RCB::new(1, 1, Vec::new())),
        Some(RCB::new(2, 2, Vec::new())),
        Some(RCB::new(3, 3, Vec::new())),
    ]
}
