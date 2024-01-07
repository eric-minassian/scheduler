use crate::scheduler::defaults::{pcb_list_default, rcb_list_default};
use crate::scheduler::pcb::PCB;
use crate::scheduler::rcb::RCB;

mod defaults;
mod pcb;
mod rcb;

pub struct Scheduler {
    current: i32,
    pcb_list: [Option<PCB>; 16],
    rcb_list: [Option<RCB>; 4],
    ready_list: [Vec<i32>; 3],
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            current: 0,
            pcb_list: pcb_list_default(),
            rcb_list: rcb_list_default(),
            ready_list: [vec![0], Vec::new(), Vec::new()],
        }
    }

    pub fn init(&mut self) {
        self.current = 0;
        self.pcb_list = pcb_list_default();
        self.rcb_list = rcb_list_default();
        self.ready_list = [vec![0], Vec::new(), Vec::new()];
    }

    fn scheduler(&mut self) -> i32 {
        let mut i = 0;
        while i < 3 {
            if self.ready_list[i].len() > 0 {
                self.current = self.ready_list[i][0];
                return self.current;
            }
            i += 1;
        }

        return -1;
    }

    // pub fn create(&mut self, priority:
}
