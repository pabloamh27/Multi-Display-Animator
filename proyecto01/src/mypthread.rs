use crate::mypthread_struct::{Thread, State, get_state};
use crate::myschedulers::{my_sched_round_robin, my_sched_sort, my_sched_real_time};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void};
use std::mem;

use rand::Rng;



static mut THREADS:Vec<Thread> = Vec::new();
static mut ROUND_ROBIN_THREADS:Vec<Thread> = Vec::new();
static mut SORT_THREADS:Vec<Thread> = Vec::new();
static mut WAITING_THREADS:Vec<Thread> = Vec::new();

pub static mut CURRENT_THREAD:*mut ucontext_t = std::ptr::null_mut();



//static mut signal_context = ucontext_t::default();
//BRETEAR ESTO
static mut STACK_SIZE: usize = 10000;
pub static mut active_sched: u64 = 0;
static mut PARENT: Option<ucontext_t> = None;


//funcion para crear un hilo
pub (crate) unsafe fn my_thread_create(func: extern "C" fn(), priority_thread: isize) -> Thread{

    // sheduler_type = 0 -> Round Robin
    // sheduler_type = 1 -> Sorteo
    let mut rng = rand::thread_rng();
    let sheduler_type = rng.gen_range(0..2);

    let mut st1: [c_char; 8192] = [mem::zeroed(); 8192];
    let mut child_temp: ucontext_t = mem::uninitialized();

    getcontext(&mut child_temp as *mut ucontext_t);

    child_temp.uc_stack.ss_sp = st1.as_mut_ptr() as *mut c_void;
    child_temp.uc_stack.ss_size = mem::size_of_val(&st1);
    //Ver como importar esta variable del ucontext_t

    child_temp.uc_link = parent_match() as *mut ucontext_t;
   
    makecontext(&mut child_temp as *mut ucontext_t, func, 0);
    //Codigo de Bryan "let thread_t = rb_thread_t {id:ID_THREAD, priority:priority_thread, context:  child_temp };"
    let new_thread = Thread {id:(get_number_of_threads() + 1), state: State::Off, tickets: 1, 
        scheduler: sheduler_type, context: child_temp};
    //Thread creado
    THREADS.push(new_thread);

    if new_thread.scheduler == 0 {
        ROUND_ROBIN_THREADS.push(new_thread);
    } else if new_thread.scheduler == 1 {
        SORT_THREADS.push(new_thread);
    }
    //THREADS_CONTEXT.push(Some(child_temp));
    return new_thread.clone();
}

//Funcion para terminar un hilo
pub (crate) unsafe fn  my_thread_end(context: *mut ucontext_t) {
    swapcontext(context, parent_match() as *mut ucontext_t);
}


// my_thread_yield(hilo1.context, hilo2.context)

pub (crate) unsafe fn my_thread_yield(context_sender: *mut ucontext_t, context_receiver: *const ucontext_t) {
    swapcontext(context_sender as *mut ucontext_t, context_receiver as *const ucontext_t);
}

pub (crate) unsafe fn my_thread_join(mut thread: Thread) {
    if WAITING_THREADS.is_empty(){
        thread.state = State::Ready;
        WAITING_THREADS.push(thread);
    }
    else if get_state(WAITING_THREADS[0]) == State::Ready {
        thread.state = State::Waiting;
        WAITING_THREADS.push(thread);
    }
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
        for thread in THREADS.clone() {
            number_of_threads += 1;
        }
        return number_of_threads;
    }

}



//funcion para mapear schedulers
pub (crate) unsafe fn my_thread_change_sched(scheduler_type: isize, mut thread: Thread){
    if scheduler_type == 0 {
        thread.scheduler = 0;
        for i in 0..SORT_THREADS.len() {
            if SORT_THREADS[i].id == thread.id {
                SORT_THREADS.remove(i);
                break;
            }
        }
        ROUND_ROBIN_THREADS.push(thread);
    } 
    else if scheduler_type == 1 {
        thread.scheduler = 1;
        for i in 0..ROUND_ROBIN_THREADS.len() {
            if ROUND_ROBIN_THREADS[i].id == thread.id {
                ROUND_ROBIN_THREADS.remove(i);
                break;
            }
        }
        SORT_THREADS.push(thread);

    }
}
