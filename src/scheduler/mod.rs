use crate::scheduler::defaults::{pcb_list_default, rcb_list_default};
use crate::scheduler::pcb::PCB;
use crate::scheduler::rcb::RCB;

use self::pcb::{PCBResource, PCBState};
use self::rcb::RCBResource;

mod defaults;
mod pcb;
mod rcb;

pub struct Scheduler {
    current: usize,
    pcb_list: [Option<PCB>; 16],
    rcb_list: [RCB; 4],
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

    pub fn init(&mut self) -> Option<usize> {
        self.current = 0;
        self.pcb_list = pcb_list_default();
        self.rcb_list = rcb_list_default();
        self.ready_list = [vec![0], Vec::new(), Vec::new()];

        return Some(self.current);
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
                // Add To Current Process's Children List
                let current_pcb = match &mut self.pcb_list[self.current] {
                    Some(pcb) => pcb,
                    None => return None,
                };
                current_pcb.children.push(pid);

                // Create PCB
                self.pcb_list[pid] = Some(PCB::new(priority, Some(self.current)));

                // Add To Ready List
                self.ready_list[priority].push(pid);

                return self.scheduler();
            }
        }

        return None;
    }

    fn is_child(&self, pid: usize) -> bool {
        let parent_pid = match &self.pcb_list[pid] {
            Some(pcb) => match pcb.parent {
                Some(parent_id) => parent_id,
                None => return false,
            },
            None => return false,
        };

        if pid == self.current || parent_pid == self.current {
            return true;
        } else {
            return self.is_child(parent_pid);
        }
    }

    pub fn destroy(&mut self, pid: i32) -> Option<usize> {
        // Bounds check
        if pid < 0 || pid > 15 {
            return None;
        }

        // Cast pid to usize
        let pid = pid as usize;

        // Check if the current process is the child of the process to be destroyed
        if !self.is_child(pid) {
            return None;
        }

        // Destroy the children of the process to be destroyed
        let pcb = match &self.pcb_list[pid] {
            Some(pcb) => pcb,
            None => return None,
        };
        let children = pcb.children.clone();
        for child in children {
            self.destroy(child as i32);
        }

        // Get the PCB of the process to be destroyed
        let pcb = match &self.pcb_list[pid] {
            Some(pcb) => pcb,
            None => return None,
        };

        // 1. Remove From The Ready List
        if let Some(pos) = self.ready_list[pcb.priority].iter().position(|&x| x == pid) {
            self.ready_list[pcb.priority].remove(pos);
        }

        // 2. Remove From The Parent's Children List
        if let Some(parent) = pcb.parent {
            let parent_pcb = match &mut self.pcb_list[parent] {
                Some(parent_pcb) => parent_pcb,
                None => return None,
            };

            if let Some(pos) = parent_pcb.children.iter().position(|&x| x == pid) {
                parent_pcb.children.remove(pos);
            }
        }

        // 3. Remove From The RCB's Waiting List
        self.rcb_list.iter_mut().for_each(|rcb| {
            if let Some(pos) = rcb.waitlist.iter().position(|x| x.pid == pid) {
                rcb.waitlist.remove(pos);
            }
        });

        // 4. Remove From The PCB List
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

        let pcb = match &mut self.pcb_list[self.current] {
            Some(pcb) => pcb,
            None => return None,
        };
        let rcb = match self.rcb_list.get_mut(rid) {
            Some(rcb) => rcb,
            None => return None,
        };

        if rcb.units_available < units {
            // Block
            pcb.state = PCBState::BLOCKED;

            match self.ready_list[pcb.priority]
                .iter_mut()
                .position(|x| *x == self.current)
            {
                Some(pos) => {
                    self.ready_list[pcb.priority].remove(pos);
                }
                None => return None,
            }

            rcb.waitlist.push(RCBResource {
                pid: self.current,
                units,
            });

            return self.scheduler();
        }

        // Allocate
        pcb.resources.push(PCBResource { rid, units });
        rcb.units_available -= units;

        return self.scheduler();
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

        let rcb = match self.rcb_list.get_mut(rid) {
            Some(rcb) => rcb,
            None => return None,
        };

        {
            let pcb = match &mut self.pcb_list[self.current] {
                Some(pcb) => pcb,
                None => return None,
            };

            // Remove (r, k) from i.resources
            if let Some(pos) = pcb
                .resources
                .iter()
                .position(|x| x.rid == rid && x.units == units)
            {
                pcb.resources.remove(pos);
            } else {
                return None;
            }

            // Add k to r.units_available
            rcb.units_available += units;
        }
        // Unblock processes
        let mut i = 0;
        while i < rcb.waitlist.len() && rcb.units_available > 0 {
            if rcb.waitlist[i].units <= rcb.units_available {
                // Allocate
                let pid = rcb.waitlist[i].pid;
                let pcb = match &mut self.pcb_list[pid] {
                    Some(pcb) => pcb,
                    None => return None,
                };
                pcb.resources.push(PCBResource {
                    rid,
                    units: rcb.waitlist[i].units,
                });
                rcb.units_available -= rcb.waitlist[i].units;

                // Unblock
                pcb.state = PCBState::READY;
                self.ready_list[pcb.priority].push(pid);
                rcb.waitlist.remove(i);
            } else {
                i += 1;
            }
        }

        return self.scheduler();
    }

    pub fn timeout(&mut self) -> Option<usize> {
        let priority = self.pcb_list[self.current].as_ref().unwrap().priority;

        self.ready_list[priority].remove(0);
        self.ready_list[priority].push(self.current);

        return self.scheduler();
    }
}
