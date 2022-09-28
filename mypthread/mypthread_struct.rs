
// Este archivo contiene la estructura de datos de pthread (hilos)

struct thread {
    id: u64,
    state: State,
    scheduler: Scheduler,
    priority: u64,
    tickets: u64,
}

// Define los tipos de scheduler que hay
enum Scheduler {
    RoundRobin,
    Sorteo,
    TiempoReal,
}

// Define los estados del hilo
enum State {
    On,
    Off,
    Waiting,
    Blocked,
}

// implementación de la estructura de datos de pthread (hilos)

impl thread {
    fn new(id: u64, state: State, scheduler: Scheduler, priority: u64, tickets: u64) -> thread {
        thread {
            id: id,
            state: state,
            scheduler: scheduler,
            priority: priority,
            tickets: tickets,
        }
    }
}



