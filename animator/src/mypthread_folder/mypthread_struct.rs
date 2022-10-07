
// Este archivo contiene la estructura de datos de pthread (hilos)

use libc::ucontext_t;



pub(crate) struct thread {
    pub(crate) id: u64,
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
#[derive(PartialEq)]
pub(crate) enum State {
    On,
    Off,
    Waiting,
    Blocked,
}

// implementación de la estructura de datos de pthread (hilos)

impl thread {
    fn new(id: u64, state: State, scheduler: isize, priority: u64, context: ucontext_t,tickets: u64) -> thread {
        thread {
            id: id,
            state: state,
            scheduler: scheduler,
            context: context,
            tickets: tickets,
        }
    }
}



