use crate::mypthread_struct::{thread, State};
use crate::myschedulers::{my_sched_round_robin, my_sched_sort, my_sched_real_time};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void};
use std::mem;


static mut THREADS:Vec<thread> = Vec::new();
//static mut signal_context = ucontext_t::default();
//BRETEAR ESTO
static mut STACK_SIZE: usize = 10000;
static mut active_sched: i64 = 0;
static mut PARENT: Option<ucontext_t> = None;

//funcion para crear un hilo
pub (crate) unsafe fn my_thread_create(func: extern "C" fn(), priority_thread: isize) -> thread{

    let mut st1: [c_char; 8192] = [mem::zeroed(); 8192];
    let mut child_temp: ucontext_t = mem::uninitialized();

    getcontext(&mut child_temp as *mut ucontext_t);

    child_temp.uc_stack.ss_sp = st1.as_mut_ptr() as *mut c_void;
    child_temp.uc_stack.ss_size = mem::size_of_val(&st1);
    //Ver como importar esta variable del ucontext_t

    child_temp.uc_link = parent_match() as *mut ucontext_t;
   
    makecontext(&mut child_temp as *mut ucontext_t, func, 0);
    //Codigo de Bryan "let thread_t = rb_thread_t {id:ID_THREAD, priority:priority_thread, context:  child_temp };"
    let mut new_thread = thread {id:(get_number_of_threads() + 1), state: State::Off, tickets: 1, 
        scheduler: 0, context: child_temp};
    //Thread creado
    THREADS.push(new_thread);
    //Revisar este push, es necesario?
    //THREADS_CONTEXT.push(Some(child_temp));
    return thread;
}

//funcion para saber si un hilo tiene padre
pub (crate) unsafe fn parent_match() -> &'static mut ucontext_t{
    match PARENT {
        Some(ref mut x) => &mut *x,
        None => panic!(),
    }
}

//Funcion que da el numero de hilos en la lista de hilos
pub (crate) fn get_number_of_threads() -> usize {
    let mut number_of_threads = 0;
    unsafe{
        for thread in THREADS {
            number_of_threads += 1;
        }
        return number_of_threads;
    }

}

//Funcion para terminar un hilo
pub (crate) unsafe fn  my_thread_end(context: *mut ucontext_t) {
    swapcontext(context, parent_match() as *mut ucontext_t);
}