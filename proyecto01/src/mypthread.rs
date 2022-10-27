use crate::mypthread_struct::{Thread, State, _get_state};
use crate::myschedulers::{my_scheduler_lottery, my_scheduler_real_time, my_scheduler_round_robin};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void, setcontext};
use std::mem;
use std::process::exit;
use crate::OBJECTS;


pub static mut THREADS:Vec<Thread> = Vec::new();
pub static mut DEAD_THREADS:Vec<Thread> = Vec::new();
pub static mut ACTIVE_THREADS:Vec<Thread> = Vec::new();

pub static mut CURRENT_THREAD: *mut ucontext_t = 0 as *mut ucontext_t;
pub static mut EXIT_CONTEXT: *mut ucontext_t = 0 as *mut ucontext_t;


pub static mut ACTIVE_SCHEDULER: isize = 0;
static mut EXIT_CONTEXT_OPT: Option<ucontext_t> = None;

//Funcion para crear un hilo nuevo
pub (crate) unsafe fn my_thread_create(func: extern "C" fn(), index: usize) -> Thread{

    let mut st1: [c_char; 10000] = [mem::zeroed(); 10000];
    let mut context: ucontext_t = mem::MaybeUninit::zeroed().assume_init();


    //Aqui inicializa el contexto del hilo y llama a su padre directo que es como un root
    getcontext(&mut context as *mut ucontext_t);
    context.uc_stack.ss_sp = st1.as_mut_ptr() as *mut c_void;
    context.uc_stack.ss_size = mem::size_of_val(&st1);
    context.uc_stack.ss_flags = 0;
    context.uc_link = parent_match() as *mut ucontext_t;

    makecontext(&mut context as *mut ucontext_t, func, 1, index);
    let new_thread = Thread {id:(get_number_of_threads() + 1), state: State::On, tickets: OBJECTS[index].tickets, context};
    THREADS.push(new_thread);
    ACTIVE_THREADS.push(new_thread);
    //new_thread.state = On;
    return new_thread;
}

//Funcion para hacerle join a un hilo
pub (crate) unsafe fn _my_thread_join(mut thread: Thread) {
    if ACTIVE_THREADS.is_empty(){
        thread.state = State::_Ready;
        ACTIVE_THREADS.push(thread);
    }
    else if _get_state(ACTIVE_THREADS[0]) == State::_Ready {
        thread.state = State::_Waiting;
        ACTIVE_THREADS.push(thread);
    }
}




//Funcion para saber si un hilo tiene padre o no
pub (crate) unsafe fn parent_match() -> &'static mut ucontext_t{
    match EXIT_CONTEXT_OPT {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

//Funcion para hacer match con el hijo de un hilo
pub (crate) unsafe fn child_match(i:usize) -> &'static mut ucontext_t {
        match THREADS[i].context {
            ref mut x => &mut *x,

    }
}

//Inicializa el contexto principal y el manager que corre todo
pub (crate) unsafe fn start_manager(scheduler:isize) -> ThreadManager {
    EXIT_CONTEXT_OPT = Some(mem::MaybeUninit::zeroed().assume_init());
    let handler = ThreadManager {scheduler_type:scheduler};
    return handler;
}

// Este struct se encarga de manejar los hilos
pub struct ThreadManager {
    pub scheduler_type: isize
}impl ThreadManager {
    // Corre los hilos llamando a un scheduler que entra por parametro
    pub unsafe fn run_threads(&mut self) {
        EXIT_CONTEXT = parent_match() as *mut ucontext_t;
        ACTIVE_SCHEDULER = self.scheduler_type;
        if self.scheduler_type == 1 {
            println!("Se esta usando RoundRobin");
            my_scheduler_round_robin();

        } else if self.scheduler_type == 2 {
            println!("Se esta usando sorteo");
            my_scheduler_lottery();
        }
        else if self.scheduler_type == 3 {
            println!("Se esta usando sorteo");
            my_scheduler_real_time();
        }
        else {
            println!("NO EXISTE ESE TIPO DE SCHEDULER!");
            exit(1);
        }
    }
}


//Funcion que da el numero de hilos en la lista de hilos
pub (crate) fn get_number_of_threads() -> usize {
    let mut number_of_threads = 0;
    unsafe{
        for _thread in THREADS.clone() {
            number_of_threads += 1;
        }
        return number_of_threads;
    }

}

//Funcion que hace que un thread haga detach, cuando un proceso termina se le hace free a la memoria.
pub (crate) unsafe fn _my_thread_detach( thread: &mut ucontext_t) {
    setcontext(thread);
    drop(thread);
}

//Funcion para terminar un hilo
pub (crate) unsafe fn  _my_thread_end(context: *mut ucontext_t) {
    swapcontext(context, parent_match() as *mut ucontext_t);
}


//Funcion para ceder el contexto de un hilo a otro contexto
pub (crate) unsafe fn my_thread_yield(context_sender: *mut ucontext_t, context_receiver: *const ucontext_t) {
    swapcontext(context_sender as *mut ucontext_t, context_receiver as *const ucontext_t);
}

//Funcion para cambiar el scheduler que se esta usando
pub (crate) unsafe fn _my_thread_chsched(scheduler_type: isize){
    ACTIVE_SCHEDULER = scheduler_type;
}






