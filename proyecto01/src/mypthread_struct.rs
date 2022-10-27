use libc::ucontext_t;


#[derive(Clone, Copy)]
pub struct Thread {
    pub(crate) id: usize,
    pub(crate) state: State,
    pub(crate) context: ucontext_t,
    pub(crate) tickets: i32,
}


// Define los estados del hilo
#[derive(Clone, Copy, PartialEq)]
pub enum State {
    On = 1,
    Off = 0,
    Ready = 2,
    Waiting = 3,
    Blocked = 4,
}

// implementación de la estructura de datos de pthread (hilos)
impl Thread {
    fn new(id: usize, state: State, scheduler: isize, priority: u64, context: ucontext_t,tickets: i32) -> Thread {
        Thread {
            id: id,
            state: state,
            context: context,
            tickets: tickets,
        }
    }
}


pub(crate) fn get_state(thread: Thread) -> State {
    return thread.state;
}