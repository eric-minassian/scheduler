use crate::scheduler::pcb::PCB;
use crate::scheduler::RCB;

pub fn pcb_list_default() -> [Option<PCB>; 16] {
    [
        Some(PCB::new(0, None)),
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

pub fn rcb_list_default() -> [RCB; 4] {
    [RCB::new(1), RCB::new(1), RCB::new(2), RCB::new(3)]
}
