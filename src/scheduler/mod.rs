use crate::scheduler::defaults::{pcb_list_default, rcb_list_default};
use crate::scheduler::pcb::PCB;
use crate::scheduler::rcb::RCB;

use self::pcb::PCBState;

mod defaults;
mod pcb;
mod rcb;

pub struct Scheduler {
    current: usize,
    pcb_list: [Option<PCB>; 16],
    rcb_list: [Option<RCB>; 4],
    ready_list: [Vec<usize>; 3],
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

    fn scheduler(&mut self) -> Option<usize> {
        for priority in (0..self.ready_list.len()).rev() {
            if !self.ready_list[priority].is_empty() {
                self.current = self.ready_list[priority][0];
                return Some(self.current);
            }
        }

        return None;
    }

    pub fn create(&mut self, priority: i32) -> Option<usize> {
        // Bounds check
        if priority < 0 || priority > 2 {
            return None;
        }

        // Cast priority to usize
        let priority = priority as usize;

        for pid in 0..self.pcb_list.len() {
            if self.pcb_list[pid].is_none() {
                self.pcb_list[pid] = Some(PCB::new(priority, Some(self.current)));
                self.ready_list[priority].push(pid);
                return self.scheduler();
            }
        }

        return None;
    }

    fn is_parent(&self, parent: usize, child: usize) -> bool {
        let pcb = match &self.pcb_list[child] {
            Some(pcb) => pcb,
            None => return false,
        };

        match pcb.parent {
            Some(p) => {
                if p == parent {
                    return true;
                }

                return self.is_parent(parent, p);
            }
            None => return false,
        }
    }

    pub fn destroy(&mut self, pid: i32) -> Option<usize> {
        // Bounds check
        if pid < 0 || pid > 15 {
            return None;
        }

        // Cast pid to usize
        let pid = pid as usize;

        // Get the PCB of the process to be destroyed
        let pcb = match &self.pcb_list[pid] {
            Some(pcb) => pcb,
            None => return None,
        };

        // Check if the current process is the child of the process to be destroyed
        if self.is_parent(pid, self.current) {
            return None;
        }

        // Remove From The Ready List
        let priority = pcb.priority;
        for i in 0..self.ready_list[priority].len() {
            if self.ready_list[priority][i] == pid {
                self.ready_list[priority].remove(i);
                break;
            }
        }

        // Remove From The Parent's Children List
        let parent = pcb.parent;
        if let Some(parent) = parent {
            let parent = match &mut self.pcb_list[parent] {
                Some(parent) => parent,
                None => return None,
            };

            for i in 0..parent.children.len() {
                if parent.children[i] == pid {
                    parent.children.remove(i);
                    break;
                }
            }
        }

        // Remove From The RCB's Waiting List
        for i in 0..self.rcb_list.len() {
            let rcb = match &mut self.rcb_list[i] {
                Some(rcb) => rcb,
                None => return None,
            };

            for j in 0..rcb.waitlist.len() {
                if rcb.waitlist[j] == pid {
                    rcb.waitlist.remove(j);
                    break;
                }
            }
        }

        // Remove From The PCB List
        self.pcb_list[pid] = None;

        return self.scheduler();
    }

    pub fn request(&mut self, rid: i32, units: i32) -> Option<usize> {
        // Bounds check
        if rid < 0 || rid > 3 {
            return None;
        }

        if units < 0 {
            return None;
        }

        // Cast rid and units to usize
        let rid = rid as usize;
        let units = units as usize;

        let rcb = match &mut self.rcb_list[rid] {
            Some(rcb) => rcb,
            None => return None,
        };

        let pcb = match &mut self.pcb_list[self.current] {
            Some(pcb) => pcb,
            None => return None,
        };

        if rcb.units_available < units {
            // Block the current process

            // Remove the current process from the ready list
            let priority = pcb.priority;
            for i in 0..self.ready_list[priority].len() {
                if self.ready_list[priority][i] == self.current {
                    self.ready_list[priority].remove(i);
                    break;
                }
            }

            pcb.state = PCBState::BLOCKED;
            rcb.waitlist.push(self.current);

            return self.scheduler();
        }

        rcb.units_available -= units;

        // Update the current process
        pcb.resources[rid] += units;

        return None;
    }

    pub fn release(&mut self, rid: i32, units: i32) -> Option<usize> {
        // Bounds check
        if rid < 0 || rid > 3 {
            return None;
        }

        if units < 0 {
            return None;
        }

        // Cast rid and units to usize
        let rid = rid as usize;
        let units = units as usize;

        return None;
    }

    pub fn timeout(&mut self) -> Option<usize> {
        let priority = self.pcb_list[self.current].as_ref().unwrap().priority;

        self.ready_list[priority].remove(0);
        self.ready_list[priority].push(self.current);

        return self.scheduler();
    }
}
