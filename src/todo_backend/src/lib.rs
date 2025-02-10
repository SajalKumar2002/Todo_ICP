use candid::CandidType;
use ic_cdk::{query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::cell::RefMut;

thread_local! {
    static TASKS: RefCell<Vec<Task>> = RefCell::new(Vec::new());
    static NEXT_ID: RefCell<u64> = RefCell::new(0);
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, CandidType)]
struct Task {
    id: u64,
    title: String,
    is_completed: bool,
}

#[update]
fn add_task(input: String) -> Task {
    let id: u64 = NEXT_ID.with(|next_id: &RefCell<u64>| {
        let mut next_id: RefMut<u64> = next_id.borrow_mut();
        let id: u64 = *next_id;
        *next_id += 1;
        id
    });

    let task: Task = Task {
        id,
        title: input,
        is_completed: false,
    };

    TASKS.with(|tasks| {
        tasks.borrow_mut().push(task.clone());
    });

    task
}

#[update]
fn update_task(id: u64, input: String) -> Result<Task, String> {
    TASKS.with(|tasks: &RefCell<Vec<Task>>| {
        let mut tasks: RefMut<'_, Vec<Task>> = tasks.borrow_mut();
        let task = tasks.get_mut(id as usize).ok_or("Task not found")?;
        task.title = input;
        Ok(task.clone())
    })
}

#[query]
fn get_all_tasks() -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .iter()
            .filter(|task| !task.is_completed)
            .cloned()
            .collect()
    })
}

#[query]
fn count_tasks() -> u64 {
    TASKS.with(|tasks: &RefCell<Vec<Task>>| tasks.borrow().iter().count() as u64)
}

#[query]
fn delete_task(id: u64) -> Result<(), String> {
    TASKS.with(|tasks: &RefCell<Vec<Task>>| {
        let mut tasks: std::cell::RefMut<'_, Vec<Task>> = tasks.borrow_mut();
        tasks.retain(|task: &Task| task.id != id);
        Ok(())
    })
}
