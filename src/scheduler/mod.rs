use crate::scheduler::defaults::{pcb_list_default, rcb_list_default};
use crate::scheduler::pcb::PCB;
use crate::scheduler::rcb::RCB;

use self::pcb::{PCBResource, PCBState};
use self::rcb::RCBResource;

pub mod defaults;
pub mod pcb;
pub mod rcb;

pub struct Scheduler {
    pub current: usize,
    pub pcb_list: [Option<PCB>; 16],
    pub rcb_list: [RCB; 4],
    pub ready_list: [Vec<usize>; 3],
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

        Some(self.current)
    }

    fn scheduler(&mut self) -> usize {
        self.current = self
            .ready_list
            .iter()
            .rev()
            .find(|list| !list.is_empty())
            .expect("SCHEDULER: Ready List Shouldn't Be Empty")
            .first()
            .expect("SCHEDULER: Error Accessing First Element")
            .clone();

        self.current
    }

    pub fn create(&mut self, priority: usize) -> Option<usize> {
        // Bounds check
        if priority >= self.ready_list.len() {
            eprintln!("CREATE: Priority Out Of Bounds");
            return None;
        }

        // Find an empty PCB
        let empty_pid = match self.pcb_list.iter().position(|x| x.is_none()) {
            Some(pos) => pos,
            None => {
                eprintln!("CREATE: No Empty PCBs");
                return None;
            }
        };

        // Add To Current Process's Children List
        let current_pcb = self
            .pcb_list
            .get_mut(self.current)
            .unwrap()
            .as_mut()
            .unwrap();

        current_pcb.children.push(empty_pid);

        // Create PCB
        self.pcb_list[empty_pid] = Some(PCB::new(priority, Some(self.current)));

        // Add To Ready List
        self.ready_list[priority].push(empty_pid);

        println!("process {} created", empty_pid);

        Some(self.scheduler())
    }

    fn is_child_of_current_process(&self, pid: usize) -> bool {
        // Bounds check
        if pid >= self.pcb_list.len() {
            return false;
        }

        // Check if the current process is the child process
        if pid == self.current {
            return true;
        }

        match &self.pcb_list[pid] {
            Some(pcb) => match pcb.parent {
                Some(parent_id) => {
                    if parent_id == self.current {
                        return true;
                    } else {
                        return self.is_child_of_current_process(parent_id);
                    }
                }
                None => return false,
            },
            None => return false,
        }
    }

    pub fn destroy(&mut self, pid: usize) -> Option<usize> {
        // Bounds Check
        if pid >= self.pcb_list.len() {
            eprintln!("DESTROY: PID Out Of Bounds");
            return None;
        }

        // Don't Destroy Process 0
        if pid == 0 {
            eprintln!("DESTROY: Cannot Destroy Process 0");
            return None;
        }

        // Only Destroy Child Processes
        if !self.is_child_of_current_process(pid) {
            eprintln!("DESTROY: PID Is Not A Child Of The Current Process");
            return None;
        }

        // Recursively Destroy Children
        let children = match &self.pcb_list[pid] {
            Some(pcb) => pcb.children.clone(),
            None => {
                eprintln!("DESTROY: PID Does Not Exist");
                return None;
            }
        };

        children.iter().for_each(|&child| {
            self.destroy(child);
        });

        // Get the PCB of the process to be destroyed
        let pcb = match &self.pcb_list[pid] {
            Some(pcb) => pcb,
            None => {
                eprintln!("DESTROY: PID Does Not Exist");
                return None;
            }
        };

        // Remove From The Ready List
        if let Some(pos) = self.ready_list[pcb.priority].iter().position(|&x| x == pid) {
            self.ready_list[pcb.priority].remove(pos);
        }

        // Remove From The Parent's Children List
        match pcb.parent {
            Some(parent) => {
                let parent_pcb = self.pcb_list[parent]
                    .as_mut()
                    .expect("DESTROY: Parent PCB should exist.");

                match parent_pcb.children.iter().position(|&x| x == pid) {
                    Some(pos) => {
                        parent_pcb.children.remove(pos);
                    }
                    None => {
                        panic!("DESTROY: Child should be in parent's children list.");
                    }
                }
            }
            None => {
                panic!("DESTROY: All processes should have a parent except process 0.");
            }
        }

        // Release Resources
        let current_pcb_resources = self.pcb_list[self.current]
            .as_mut()
            .expect("DESTROY: Current PCB should exist.")
            .resources
            .clone();

        current_pcb_resources.iter().for_each(|resource| {
            self.release(resource.rid, resource.units);
        });

        // Remove From The PCB List
        self.pcb_list[pid] = None;

        // TODO: Replace with "{n} processes destroyed"
        println!("process {} destroyed", pid);

        Some(self.scheduler())
    }

    pub fn request(&mut self, rid: usize, units: usize) -> Option<usize> {
        // Bounds Check
        if rid >= self.rcb_list.len() {
            eprintln!("REQUEST: RID Out Of Bounds");
            return None;
        }

        if units == 0 {
            eprintln!("REQUEST: Units Cannot Be 0");
            return None;
        }

        // Process 0 Can't Request
        if self.current == 0 {
            eprintln!("REQUEST: Process 0 Can't Request Resources");
            return None;
        }

        let pcb = match &mut self.pcb_list[self.current] {
            Some(pcb) => pcb,
            None => {
                panic!("REQUEST: Current PCB should exist.");
            }
        };
        let rcb = match self.rcb_list.get_mut(rid) {
            Some(rcb) => rcb,
            None => {
                eprintln!("REQUEST: RCB Does Not Exist");
                return None;
            }
        };

        if rcb.inventory < units {
            eprintln!("REQUEST: Units Exceeds Max Inventory");
            return None;
        }

        if rcb.units_available < units {
            // BLOCK

            // Update PCB State To Blocked
            pcb.state = PCBState::BLOCKED;

            // Remove From Ready List
            match self.ready_list[pcb.priority]
                .iter()
                .position(|&x| x == self.current)
            {
                Some(pos) => {
                    self.ready_list[pcb.priority].remove(pos);
                }
                None => {
                    panic!("REQUEST: Current process should be in the ready list.");
                }
            }

            // Add To RCB Waitlist
            rcb.waitlist.push(RCBResource {
                pid: self.current,
                units,
            });

            println!("process {} blocked", self.current);

            return Some(self.scheduler());
        }

        // ALLOCATE
        pcb.resources.push(PCBResource { rid, units });
        rcb.units_available -= units;

        println!("process {} allocated", self.current);

        return Some(self.scheduler());
    }

    pub fn release(&mut self, rid: usize, units: usize) -> Option<usize> {
        // Bounds Check
        if rid >= self.rcb_list.len() {
            eprintln!("RELEASE: RID Out Of Bounds");
            return None;
        }

        if units == 0 {
            eprintln!("RELEASE: Units Cannot Be 0");
            return None;
        }

        // Get RCB
        let rcb = match self.rcb_list.get_mut(rid) {
            Some(rcb) => rcb,
            None => {
                eprintln!("RELEASE: RCB Does Not Exist");
                return None;
            }
        };

        let pcb = match &mut self.pcb_list[self.current] {
            Some(pcb) => pcb,
            None => {
                panic!("RELEASE: Current PCB should exist.");
            }
        };

        // Remove Resource From PCB
        match pcb
            .resources
            .iter()
            .position(|x| x.rid == rid && x.units == units)
        {
            Some(pos) => pcb.resources.remove(pos),
            None => {
                eprintln!("RELEASE: Resource Does Not Exist");
                return None;
            }
        };

        // Add Units To RCB
        rcb.units_available += units;

        // Unblock Processes
        let mut i = 0;
        while i < rcb.waitlist.len() && rcb.units_available > 0 {
            if rcb.waitlist[i].units <= rcb.units_available {
                let temp_pid = rcb.waitlist[i].pid;
                let temp_units = rcb.waitlist[i].units;

                let temp_pcb = match &mut self.pcb_list[temp_pid] {
                    Some(pcb) => pcb,
                    None => {
                        panic!("RELEASE: PCB should exist.");
                    }
                };

                // Add Resource To PCB
                temp_pcb.resources.push(PCBResource {
                    rid,
                    units: temp_units,
                });

                // Remove Resource From RCB
                rcb.units_available -= temp_units;
                rcb.waitlist.remove(i);

                // Add To Ready List
                self.ready_list[temp_pcb.priority].push(temp_pid);
                temp_pcb.state = PCBState::READY;

                println!("process {} unblocked", temp_pid);
            } else {
                i += 1;
            }
        }

        println!("process {} released", self.current);

        Some(self.scheduler())
    }

    pub fn timeout(&mut self) -> Option<usize> {
        let priority = self.pcb_list[self.current].as_ref()?.priority;

        let priority_level_list = self.ready_list.get_mut(priority)?;

        if priority_level_list.is_empty() {
            panic!(
                "TIMEOUT: Priority level list shouldn't be empty. Should contain current process."
            )
        }

        if priority_level_list[0] != self.current {
            panic!("TIMEOUT: Current process should be at the top of the ready list.")
        }

        priority_level_list.remove(0);
        priority_level_list.push(self.current);

        println!("process {} timed out", self.current);

        Some(self.scheduler())
    }
}
