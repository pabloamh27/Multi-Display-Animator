mod mypthread_struct;
use mypthread_struct::create_thread;

// todas las funciones de pthread deben hechas aquí


// función para crear un hilo
fn create_thread(id: u64, state: Bool, scheduler: Scheduler) -> thread {
    return thread {
        id: id,
        state: state,
        scheduler: scheduler,
        priority: 1,
        tickets: 1,
    }
}

// función para iniciar un hilo
fn start_thread(thread: thread) -> thread {
    thread.state = true;
    return thread;
}

// función para terminar un hilo
fn end_thread(thread: thread) -> thread {
    thread.state = false;
    return thread;
}

// función para cambiar el estado de un hilo
fn change_state(thread: thread, state: Bool) -> thread {
    thread.state = state;
    return thread;
}

// función para cambiar el scheduler de un hilo
fn change_scheduler(thread: thread, scheduler: Scheduler) -> thread {
    thread.scheduler = scheduler;
    return thread;
}

// función para cambiar la prioridad de un hilo
fn change_priority(thread: thread, priority: u64) -> thread {
    thread.priority = priority;
    return thread;
}

// función para cambiar el número de tickets de un hilo
fn change_tickets(thread: thread, tickets: u64) -> thread {
    thread.tickets = tickets;
    return thread;
}

// función para obtener el id de un hilo
fn get_id(thread: thread) -> u64 {
    return thread.id;
}

// función para obtener el estado de un hilo
fn get_state(thread: thread) -> Bool {
    return thread.state;
}

// función para obtener el scheduler de un hilo
fn get_scheduler(thread: thread) -> Scheduler {
    return thread.scheduler;
}

// función para obtener la prioridad de un hilo
fn get_priority(thread: thread) -> u64 {
    return thread.priority;
}

// función para obtener el número de tickets de un hilo
fn get_tickets(thread: thread) -> u64 {
    return thread.tickets;
}

// función para obtener el número de hilos
fn get_number_of_threads(thread_pool: Vec<thread>) -> u64 {
    number_of_threads = 0;
    for thread in thread_pool {
        number_of_threads += 1;
    }
    return number_of_threads;
}

// función para obtener el número de hilos activos
fn get_number_of_active_threads(thread_pool: Vec<thread>) -> u64 {
    number_of_active_threads = 0;
    for thread in thread_pool {
        if thread.state == true {
            number_of_active_threads += 1;
        }
    }
    return number_of_active_threads;
}



