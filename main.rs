struct thread {
    id: u64,
    state: State,
    scheduler: Scheduler,
    priority: u64,
    tickets: u64,
}

fn create_thread(id: u64, state: Bool, scheduler: Scheduler) -> thread {
    return thread {
        id: id,
        state: state,
        scheduler: scheduler,
        priority: 1,
        tickets: 1,
    }
}



