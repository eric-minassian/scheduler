use scheduler::scheduler::{
    defaults::{pcb_list_default, rcb_list_default},
    Scheduler,
};

#[test]
fn test_scheduler_new() {
    let scheduler = Scheduler::new();
    assert_eq!(scheduler.current, 0);
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
    assert_eq!(scheduler.current, 0);
    assert_eq!(scheduler.ready_list, [vec![0], Vec::new(), Vec::new()]);
}

#[test]
fn test_scheduler_create() {
    let mut scheduler = Scheduler::new();

    let result = scheduler.create(1);
    assert_eq!(result, Some(1));
    assert_eq!(scheduler.ready_list, [vec![0], vec![1], Vec::new()]);

    let result = scheduler.create(2);
    assert_eq!(result, Some(2));
    assert_eq!(scheduler.ready_list, [vec![0], vec![1], vec![2]]);

    let result = scheduler.create(1);
    assert_eq!(result, Some(2));
    assert_eq!(scheduler.ready_list, [vec![0], vec![1, 3], vec![2]]);

    let result = scheduler.create(2);
    assert_eq!(result, Some(2));
    assert_eq!(scheduler.ready_list, [vec![0], vec![1, 3], vec![2, 4]]);

    let result = scheduler.create(0);
    assert_eq!(result, Some(2));
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
