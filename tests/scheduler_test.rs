use scheduler::scheduler::{
    defaults::{pcb_list_default, rcb_list_default},
    pcb::{PCBResource, PCBState, PCB},
    rcb::{RCBResource, RCB},
    Scheduler,
};

#[test]
fn test_scheduler_new() {
    let scheduler = Scheduler::new();
    assert_eq!(scheduler.running_pid, 0);
    assert_eq!(scheduler.ready_list, [vec![0], Vec::new(), Vec::new()]);
    assert_eq!(scheduler.pcb_list, pcb_list_default());
    assert_eq!(scheduler.rcb_list, rcb_list_default());
}

#[test]
fn test_scheduler_init() {
    let mut scheduler = Scheduler::new();

    scheduler.create(1);
    scheduler.create(2);
    scheduler.request(1, 1);
    scheduler.request(2, 1);
    scheduler.release(1, 1);
    scheduler.destroy(1);
    scheduler.destroy(2);

    let result = scheduler.init();
    assert_eq!(result, Some(0));
    assert_eq!(scheduler.running_pid, 0);
    assert_eq!(scheduler.ready_list, [vec![0], Vec::new(), Vec::new()]);
}

#[test]
fn test_scheduler_create() {
    let mut scheduler = Scheduler::new();

    assert_eq!(scheduler.pcb_list[1], None);
    assert_eq!(scheduler.create(1), Some(1));
    assert_eq!(scheduler.ready_list, [vec![0], vec![1], Vec::new()]);
    assert_eq!(
        scheduler.pcb_list[1].as_ref().unwrap(),
        &PCB {
            children: Vec::new(),
            parent: Some(0),
            priority: 1,
            state: PCBState::READY,
            resources: Vec::new()
        }
    );

    assert_eq!(scheduler.create(2), Some(2));
    assert_eq!(scheduler.ready_list, [vec![0], vec![1], vec![2]]);

    assert_eq!(scheduler.create(1), Some(2));
    assert_eq!(scheduler.ready_list, [vec![0], vec![1, 3], vec![2]]);

    assert_eq!(scheduler.create(2), Some(2));
    assert_eq!(scheduler.ready_list, [vec![0], vec![1, 3], vec![2, 4]]);

    assert_eq!(scheduler.create(0), Some(2));
    assert_eq!(scheduler.ready_list, [vec![0, 5], vec![1, 3], vec![2, 4]]);
}

#[test]
fn test_scheduler_destroy() {
    let mut scheduler = Scheduler::new();

    scheduler.create(1);
    let result = scheduler.destroy(1);
    assert_eq!(result, Some(0));
    assert_eq!(scheduler.ready_list, [vec![0], Vec::new(), Vec::new()]);
}

#[test]
fn general_request() {
    let mut scheduler = Scheduler::new();

    scheduler.create(1);

    assert_eq!(
        scheduler.rcb_list[1],
        RCB {
            inventory: 1,
            units_available: 1,
            waitlist: Vec::new()
        }
    );
    assert_eq!(scheduler.request(1, 1), Some(1));
    assert_eq!(
        scheduler.rcb_list[1],
        RCB {
            inventory: 1,
            units_available: 0,
            waitlist: Vec::new()
        }
    );
    assert_eq!(
        scheduler.pcb_list[1].as_ref().unwrap(),
        &PCB {
            children: Vec::new(),
            parent: Some(0),
            priority: 1,
            state: PCBState::READY,
            resources: vec![PCBResource { rid: 1, units: 1 }]
        }
    );

    assert_eq!(scheduler.request(3, 2), Some(1));
    assert_eq!(scheduler.request(1, 1), None);

    assert_eq!(
        scheduler.rcb_list[1],
        RCB {
            inventory: 1,
            units_available: 0,
            waitlist: Vec::new()
        }
    );
    assert_eq!(
        scheduler.pcb_list[1].as_ref().unwrap(),
        &PCB {
            children: Vec::new(),
            parent: Some(0),
            priority: 1,
            state: PCBState::READY,
            resources: vec![
                PCBResource { rid: 1, units: 1 },
                PCBResource { rid: 3, units: 2 }
            ]
        }
    );
}

#[test]
fn general_release() {
    let mut scheduler = Scheduler::new();

    scheduler.create(1);
    scheduler.request(1, 1);

    assert_eq!(
        scheduler.rcb_list[1],
        RCB {
            inventory: 1,
            units_available: 0,
            waitlist: Vec::new()
        }
    );

    assert_eq!(scheduler.release(1, 1), Some(1));

    assert_eq!(
        scheduler.rcb_list[1],
        RCB {
            inventory: 1,
            units_available: 1,
            waitlist: Vec::new()
        }
    );
}

// Common Errors

#[test]
fn more_than_n_processes() {
    let mut scheduler = Scheduler::new();
    let mut result;

    for i in 0..15 {
        result = scheduler.create(i % 3);
        assert_ne!(result, None);
    }

    result = scheduler.create(2);
    assert_eq!(result, None);

    result = scheduler.destroy(3);
    assert_ne!(result, None);

    result = scheduler.create(2);
    assert_ne!(result, None);
}

#[test]
fn destroy_non_child() {
    let mut scheduler = Scheduler::new();
    let mut result;

    scheduler.create(1);
    scheduler.create(2);
    scheduler.create(2);

    result = scheduler.destroy(1);
    assert_eq!(result, None);

    result = scheduler.destroy(3);
    assert_ne!(result, None);
}

#[test]
fn request_nonexistent_resource() {
    let mut scheduler = Scheduler::new();
    let mut result;

    scheduler.create(2);

    result = scheduler.request(0, 1);
    assert_ne!(result, None);

    result = scheduler.request(4, 1);
    assert_eq!(result, None);
}

#[test]
fn request_more_resources_then_available() {
    let mut scheduler = Scheduler::new();

    scheduler.create(2);

    let result = scheduler.request(0, 5);
    assert_eq!(result, None);
}

#[test]
fn release_process_not_holding() {
    let mut scheduler = Scheduler::new();

    assert_eq!(scheduler.create(2), Some(1));
    assert_eq!(scheduler.request(2, 2), Some(1));
    assert_eq!(scheduler.request(3, 3), Some(1));
    assert_eq!(scheduler.create(2), Some(1));
    assert_eq!(scheduler.timeout(), Some(2));
    assert_eq!(scheduler.request(2, 1), Some(1));
    assert_eq!(scheduler.release(2, 3), None);
    assert_eq!(scheduler.release(2, 2), Some(1));
    assert_eq!(scheduler.timeout(), Some(2));
    assert_eq!(scheduler.release(3, 3), None);
    assert_eq!(scheduler.release(2, 1), Some(2));
}

#[test]
fn process_0_cant_request() {
    let mut scheduler = Scheduler::new();

    assert_eq!(scheduler.request(1, 1), None);
}

#[test]
fn process_0_cant_destroy() {
    let mut scheduler = Scheduler::new();

    assert_eq!(scheduler.destroy(0), None);
    assert_eq!(scheduler.running_pid, 0);
    assert_eq!(scheduler.ready_list, [vec![0], Vec::new(), Vec::new()]);
}

#[test]
fn bounds_check() {
    let mut scheduler = Scheduler::new();

    assert_eq!(scheduler.create(-1), None);
    assert_eq!(scheduler.create(3), None);

    assert_eq!(scheduler.destroy(-1), None);
    assert_eq!(scheduler.destroy(16), None);

    assert_eq!(scheduler.request(-1, 1), None);
    assert_eq!(scheduler.request(1, -1), None);
    assert_eq!(scheduler.request(-1, -1), None);
    assert_eq!(scheduler.request(4, 1), None);
    assert_eq!(scheduler.request(1, 0), None);

    assert_eq!(scheduler.release(-1, 1), None);
    assert_eq!(scheduler.release(1, -1), None);
    assert_eq!(scheduler.release(-1, -1), None);
    assert_eq!(scheduler.release(4, 1), None);
    assert_eq!(scheduler.release(3, 0), None);
}

#[test]
fn multiple_releases() {
    let mut scheduler = Scheduler::new();

    scheduler.create(2); // Process 1
    scheduler.create(2); // Process 2
    scheduler.create(2); // Process 3

    assert_eq!(scheduler.running_pid, 1);
    assert_eq!(scheduler.request(3, 3), Some(1));

    scheduler.timeout();

    assert_eq!(scheduler.running_pid, 2);
    assert_eq!(scheduler.request(3, 2), Some(3));

    assert_eq!(scheduler.running_pid, 3);
    assert_eq!(scheduler.request(3, 1), Some(1));

    assert_eq!(scheduler.ready_list, [vec![0], Vec::new(), vec![1]]);

    scheduler.release(3, 3);

    assert_eq!(scheduler.ready_list, [vec![0], Vec::new(), vec![1, 2, 3]]);
}

#[test]
fn multiple_releases_with_skipping() {
    let mut scheduler = Scheduler::new();

    scheduler.create(2); // Process 1
    scheduler.create(2); // Process 2
    scheduler.create(2); // Process 3
    scheduler.create(2); // Process 4

    assert_eq!(scheduler.running_pid, 1);
    assert_eq!(scheduler.request(3, 2), Some(1));
    assert_eq!(scheduler.request(3, 1), Some(1));

    scheduler.timeout();

    assert_eq!(scheduler.running_pid, 2);
    assert_eq!(scheduler.request(3, 3), Some(3));

    assert_eq!(scheduler.running_pid, 3);
    assert_eq!(scheduler.request(3, 3), Some(4));

    assert_eq!(scheduler.running_pid, 4);
    assert_eq!(scheduler.request(3, 2), Some(1));

    assert_eq!(scheduler.ready_list, [vec![0], Vec::new(), vec![1]]);

    scheduler.release(3, 2);

    assert_eq!(scheduler.ready_list, [vec![0], Vec::new(), vec![1, 4]]);
    assert_eq!(
        scheduler.rcb_list[3].waitlist,
        vec![
            RCBResource { pid: 2, units: 3 },
            RCBResource { pid: 3, units: 3 }
        ]
    );
}

#[test]
fn deadlock() {
    let mut scheduler = Scheduler::new();

    scheduler.create(2);

    assert_eq!(scheduler.request(3, 3), Some(1));
    assert_eq!(scheduler.request(3, 3), None);
}

#[test]
fn destroy_self() {
    let mut scheduler = Scheduler::new();

    scheduler.create(2);
    scheduler.create(2);
    assert_eq!(scheduler.destroy(2), Some(1));
    assert_eq!(scheduler.destroy(1), Some(0));
}

#[test]
fn destroy_self_with_children() {
    let mut scheduler = Scheduler::new();

    scheduler.create(2);
    scheduler.create(2);
    scheduler.create(2);
    assert_eq!(scheduler.destroy(1), Some(0));
}

#[test]
fn destroy_with_resources() {
    let mut scheduler = Scheduler::new();

    // Process 1 is parent of 2 and 3
    // Process 2 has 3 units of resource 3
    // Process 3 is waiting for 3 units of resource 3
    scheduler.create(2);
    scheduler.create(2);
    scheduler.create(2);
    scheduler.timeout();
    scheduler.request(3, 3);
    scheduler.timeout();
    scheduler.request(3, 3);

    assert_eq!(
        scheduler.pcb_list[3].as_ref().unwrap(),
        &PCB {
            children: Vec::new(),
            parent: Some(1),
            priority: 2,
            state: PCBState::BLOCKED,
            resources: Vec::new()
        }
    );
    assert_eq!(
        scheduler.rcb_list[3],
        RCB {
            inventory: 3,
            units_available: 0,
            waitlist: vec![RCBResource { pid: 3, units: 3 }]
        }
    );

    assert_eq!(scheduler.destroy(3), Some(1));
    assert_eq!(scheduler.pcb_list[3], None);
    assert_eq!(
        scheduler.rcb_list[3],
        RCB {
            inventory: 3,
            units_available: 0,
            waitlist: Vec::new()
        }
    );

    assert_eq!(scheduler.destroy(2), Some(1));
    assert_eq!(scheduler.pcb_list[2], None);
    assert_eq!(
        scheduler.rcb_list[3],
        RCB {
            inventory: 3,
            units_available: 3,
            waitlist: Vec::new()
        }
    );
}

#[test]
fn add_requests() {
    let mut scheduler = Scheduler::new();

    scheduler.create(2);
    scheduler.request(3, 1);
    scheduler.request(3, 1);
    scheduler.request(3, 1);

    assert_eq!(
        scheduler.pcb_list[1].as_ref().unwrap().resources,
        vec![PCBResource { rid: 3, units: 3 }]
    );

    assert_eq!(scheduler.request(3, 1), None);

    assert_eq!(
        scheduler.pcb_list[1].as_ref().unwrap().resources,
        vec![PCBResource { rid: 3, units: 3 }]
    );

    assert_eq!(scheduler.release(3, 3), Some(1));
    assert_eq!(
        scheduler.pcb_list[1].as_ref().unwrap().resources,
        Vec::new()
    );
    assert_eq!(scheduler.rcb_list[3].units_available, 3);
}

#[test]
fn multiple_requests() {
    let mut scheduler = Scheduler::new();

    scheduler.create(1);
    scheduler.request(3, 1);
    scheduler.create(2);
    assert_eq!(scheduler.request(3, 2), Some(2));

    assert_eq!(scheduler.request(3, 1), Some(1));
    assert_eq!(
        scheduler.pcb_list[2].as_ref().unwrap(),
        &PCB {
            children: Vec::new(),
            parent: Some(1),
            priority: 2,
            state: PCBState::BLOCKED,
            resources: vec![PCBResource { rid: 3, units: 2 }]
        }
    );
    assert_eq!(
        scheduler.rcb_list[3],
        RCB {
            inventory: 3,
            units_available: 0,
            waitlist: vec![RCBResource { pid: 2, units: 1 }]
        }
    );

    assert_eq!(scheduler.release(3, 1), Some(2));
    assert_eq!(
        scheduler.pcb_list[2].as_ref().unwrap(),
        &PCB {
            children: Vec::new(),
            parent: Some(1),
            priority: 2,
            state: PCBState::READY,
            resources: vec![PCBResource { rid: 3, units: 3 }]
        }
    );
    assert_eq!(
        scheduler.rcb_list[3],
        RCB {
            inventory: 3,
            units_available: 0,
            waitlist: Vec::new()
        }
    );
}

#[test]
fn release_not_holding() {
    let mut scheduler = Scheduler::new();

    scheduler.create(1);
    assert_eq!(scheduler.release(1, 1), None);
    scheduler.request(1, 1);
    scheduler.create(2);
    assert_eq!(scheduler.release(1, 1), None);
}

#[test]
fn lowest_pid_value() {
    let mut scheduler = Scheduler::new();

    scheduler.create(1);
    scheduler.create(2);
    scheduler.create(2);
    scheduler.create(2);
    scheduler.destroy(1);
    assert_eq!(scheduler.create(2), Some(2));
}
