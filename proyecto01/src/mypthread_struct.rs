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
    _Ready = 2,
    _Waiting = 3,
}


pub(crate) fn _get_state(thread: Thread) -> State {
    return thread.state;
}