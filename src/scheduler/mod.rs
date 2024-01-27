use crate::scheduler::defaults::{pcb_list_default, rcb_list_default};
use crate::scheduler::pcb::PCB;
use crate::scheduler::rcb::RCB;

use self::pcb::{PCBResource, PCBState};
use self::rcb::RCBResource;

pub mod defaults;
pub mod pcb;
pub mod rcb;

pub struct Scheduler {
    pub running_pid: usize,
    pub pcb_list: [Option<PCB>; 16],
    pub rcb_list: [RCB; 4],
    pub ready_list: [Vec<usize>; 3],
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            running_pid: 0,
            pcb_list: pcb_list_default(),
            rcb_list: rcb_list_default(),
            ready_list: [vec![0], Vec::new(), Vec::new()],
        }
    }

    pub fn init(&mut self) -> Option<usize> {
        self.running_pid = 0;
        self.pcb_list = pcb_list_default();
        self.rcb_list = rcb_list_default();
        self.ready_list = [vec![0], Vec::new(), Vec::new()];

        Some(self.running_pid)
    }

    fn scheduler(&mut self) -> usize {
        self.running_pid = self
            .ready_list
            .iter()
            .rev()
            .find(|list| !list.is_empty())
            .expect("SCHEDULER: Ready List Shouldn't Be Empty")
            .first()
            .expect("SCHEDULER: Error Accessing First Element")
            .clone();

        self.running_pid
    }

    pub fn create(&mut self, priority: i32) -> Option<usize> {
        let priority = usize::try_from(priority).ok()?;

        // Bounds check
        if priority >= self.ready_list.len() {
            eprintln!("Priority Out Of Bounds");
            return None;
        }

        // Find an empty PCB
        let empty_pid = match self.pcb_list.iter().position(|x| x.is_none()) {
            Some(pos) => pos,
            None => {
                eprintln!("No Empty PCBs");
                return None;
            }
        };

        // Create PCB
        self.pcb_list[empty_pid] = Some(PCB::new(priority, Some(self.running_pid)));

        // Add To Parent's Children List
        self.pcb_list[self.running_pid]
            .as_mut()
            .expect("Running PCB should exist.")
            .children
            .push(empty_pid);

        // Add To Ready List
        self.ready_list[priority].push(empty_pid);

        Some(self.scheduler())
    }

    fn is_child_of_current_process(&self, pid: usize) -> bool {
        // Bounds check
        if pid >= self.pcb_list.len() {
            return false;
        }

        // Check if the current process is the child process
        if pid == self.running_pid {
            return true;
        }

        match &self.pcb_list[pid] {
            Some(pcb) => match pcb.parent {
                Some(parent_id) => {
                    if parent_id == self.running_pid {
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

    pub fn destroy(&mut self, pid: i32) -> Option<usize> {
        let pid = usize::try_from(pid).ok()?;

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
            self.destroy(child.try_into().unwrap());
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
        let pcb_2 = self.pcb_list[pid]
            .as_mut()
            .expect("DESTROY: Current PCB should exist.")
            .resources
            .clone();

        pcb_2.iter().for_each(|resource| {
            self.release_helper(pid, resource.rid, resource.units)
                .unwrap();
        });

        self.rcb_list.iter_mut().for_each(|rcb| {
            rcb.waitlist.retain(|x| x.pid != pid);
        });

        // Remove From The PCB List
        self.pcb_list[pid] = None;

        Some(self.scheduler())
    }

    pub fn request(&mut self, rid: i32, units: i32) -> Option<usize> {
        let rid = usize::try_from(rid).ok()?;
        let units = usize::try_from(units).ok()?;

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
        if self.running_pid == 0 {
            eprintln!("REQUEST: Process 0 Can't Request Resources");
            return None;
        }

        let pcb = match &mut self.pcb_list[self.running_pid] {
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

        // let units_held = pcb
        //     .resources
        //     .iter()
        //     .find(|&x| x.rid == rid)
        //     .map_or(0, |resource| resource.units);

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
                .position(|&x| x == self.running_pid)
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
                pid: self.running_pid,
                units,
            });

            return Some(self.scheduler());
        }

        // ALLOCATE
        pcb.resources.push(PCBResource { rid, units });
        rcb.units_available -= units;

        return Some(self.scheduler());
    }

    fn release_helper(&mut self, pid: usize, rid: usize, units: usize) -> Option<usize> {
        let pcb = self.pcb_list.get_mut(pid)?.as_mut()?;

        let rcb = self.rcb_list.get_mut(rid)?;

        let position = pcb
            .resources
            .iter()
            .position(|x| x.rid == rid && x.units == units)?;
        pcb.resources.remove(position);

        rcb.units_available += units;

        let mut i = 0;
        while i < rcb.waitlist.len() && rcb.units_available > 0 {
            if rcb.waitlist[i].units <= rcb.units_available {
                let temp_pid = rcb.waitlist[i].pid;
                let temp_units = rcb.waitlist[i].units;

                // println!("{:?}, {}", self.pcb_list[temp_pid], temp_pid);

                let temp_pcb = self
                    .pcb_list
                    .get_mut(temp_pid)
                    .expect("PCB should exist")
                    .as_mut()
                    .expect("PCB should exist");

                temp_pcb.state = PCBState::READY;
                temp_pcb.resources.push(PCBResource {
                    rid,
                    units: temp_units,
                });

                rcb.units_available -= temp_units;
                rcb.waitlist.remove(i);

                self.ready_list[temp_pcb.priority].push(temp_pid);
            } else {
                i += 1;
            }
        }

        Some(self.scheduler())
    }

    pub fn release(&mut self, rid: i32, units: i32) -> Option<usize> {
        let rid = usize::try_from(rid).ok()?;
        let units = usize::try_from(units).ok()?;

        // Bounds Check
        if rid >= self.rcb_list.len() {
            eprintln!("RELEASE: RID Out Of Bounds");
            return None;
        }

        if units == 0 {
            eprintln!("RELEASE: Units Cannot Be 0");
            return None;
        }

        self.release_helper(self.running_pid, rid, units)
    }

    /// # Panics
    ///
    /// Will panic if the scheduler data is corrupted from bug
    pub fn timeout(&mut self) -> Option<usize> {
        let priority = self.pcb_list[self.running_pid]
            .as_ref()
            .expect("Running PCB should exist")
            .priority;

        let priority_level_list = self
            .ready_list
            .get_mut(priority)
            .expect("Priority level list should exist");

        assert!(
            !priority_level_list.is_empty(),
            "Priority level list shouldn't be empty. Should contain current process."
        );

        assert_eq!(
            priority_level_list[0], self.running_pid,
            "TIMEOUT: Current process should be at the top of the ready list."
        );

        priority_level_list.remove(0);
        priority_level_list.push(self.running_pid);

        Some(self.scheduler())
    }
}
