use libc::ucontext_t;



pub(crate) struct thread {
    pub(crate) id: usize,
    pub(crate) state: State,
    pub(crate) scheduler: isize,
    pub(crate) context: ucontext_t,
    pub(crate) tickets: u64,
}

//Hashmap mental
/*hashmap! {
    "1" => RoundRobin(),
    "2" => Lottery(),   *sorteo*
}   
*/
// Define los estados del hilo
pub(crate) enum State {
    On = 1,
    Off = 0,
    Waiting = 2,
    Blocked = 3,
}

// implementación de la estructura de datos de pthread (hilos)
impl thread {
    fn new(id: usize, state: State, scheduler: isize, priority: u64, context: ucontext_t,tickets: u64) -> thread {
        thread {
            id: id,
            state: state,
            scheduler: scheduler,
            context: context,
            tickets: tickets,
        }
    }
}