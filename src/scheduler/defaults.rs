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

pub fn rcb_list_default() -> [Option<RCB>; 4] {
    [
        Some(RCB::new(1)),
        Some(RCB::new(1)),
        Some(RCB::new(2)),
        Some(RCB::new(3)),
    ]
}
