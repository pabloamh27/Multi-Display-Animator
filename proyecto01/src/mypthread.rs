use crate::mypthread_struct::{Thread, State, get_state};
use crate::myschedulers::{my_sched_sort, my_sched_real_time, my_sched_round_robin_aux};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void, setcontext, timer_settime, timer_create};
use std::mem;
use std::process::exit;

use rand::Rng;

static mut THREADS:Vec<Thread> = Vec::new();
static mut ROUND_ROBIN_THREADS:Vec<Thread> = Vec::new();
static mut SORT_THREADS:Vec<Thread> = Vec::new();
static mut ACTIVE_THREADS:Vec<Thread> = Vec::new();

pub static mut CURRENT_THREAD: *mut ucontext_t = 0 as *mut ucontext_t;



//static mut signal_context = ucontext_t::default();
pub static mut active_sched: u64 = 0;
static mut PARENT: Option<ucontext_t> = None;


//funcion para crear un hilo
pub (crate) unsafe fn my_thread_create(func: extern "C" fn(), priority_thread: isize) -> Thread{

    // sheduler_type = 0 -> Round Robin
    // sheduler_type = 1 -> Sorteo
    let mut rng = rand::thread_rng();
    let sheduler_type = rng.gen_range(0..2);

    let mut st1: [c_char; 10000] = [mem::zeroed(); 10000];
    let mut context: ucontext_t = mem::uninitialized();

    getcontext(&mut context as *mut ucontext_t);
    context.uc_stack.ss_sp = st1.as_mut_ptr() as *mut c_void;
    context.uc_stack.ss_size = mem::size_of_val(&st1);
    context.uc_stack.ss_flags = 0;
    context.uc_link = 0 as *mut ucontext_t;

    //Ver como importar esta variable del ucontext_t

    makecontext(&mut context as *mut ucontext_t, func, 0);
    let mut new_thread = Thread {id:(get_number_of_threads() + 1), state: State::On, tickets: 1,
        scheduler: 0, context
    };
    //Thread creado
    THREADS.push(new_thread);

    if new_thread.scheduler == 0 {
        ROUND_ROBIN_THREADS.push(new_thread);
    } else if new_thread.scheduler == 1 {
        SORT_THREADS.push(new_thread);
    }
    return new_thread;

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
    if ACTIVE_THREADS.is_empty(){
        thread.state = State::Ready;
        ACTIVE_THREADS.push(thread);
    }
    else if get_state(ACTIVE_THREADS[0]) == State::Ready {
        thread.state = State::Waiting;
        ACTIVE_THREADS.push(thread);
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

//Funcion que hace que un thread haga detach, cuando un proceso termina se le hace free a la memoria.
pub (crate) unsafe fn my_thread_detach( thread: &mut ucontext_t) {
    setcontext(thread);
    drop(thread);
}




//funcion para mapear schedulers
pub (crate) unsafe fn my_thread_chsched(scheduler_type: isize, mut thread: Thread){
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


pub (crate) unsafe fn run_threads() {
    /*
    Funcion que se encarga de correr los threads creados
    Entradas: Ninguna
    Restricciones: Ninguna
    Salidas: Hilos ejecutados
    */
    let thread:&'static mut ucontext_t  = &mut THREADS[0].context;

    CURRENT_THREAD = thread;

//    setcontext(thread);

    for i in THREADS.clone(){
        if i.state != State::Off && i.state != State::Blocked {
            ACTIVE_THREADS.push(i);
        }
    }

    for i in ACTIVE_THREADS.clone() {
        println!("Thread: {}", i.id);
    }

    for i in 0..ROUND_ROBIN_THREADS.len() {
        if ROUND_ROBIN_THREADS[i].state == State::On {
            ROUND_ROBIN_THREADS[i].state == State::Waiting;
        }
    }

    // while todavía *hayan* hilos activos
    while !ROUND_ROBIN_THREADS.is_empty() {

        // Random para ver cuál sched se usa
        //let mut rng = rand::thread_rng();
        //let lil_coin = rng.gen_range(0..2);
        let lil_coin = 0;
        println!("lil_coin: {}", lil_coin);

        if lil_coin == 0 {
            // Round Robin
            if !ROUND_ROBIN_THREADS.is_empty() {
                ROUND_ROBIN_THREADS = my_sched_round_robin_aux(ROUND_ROBIN_THREADS.clone());
                // no está haciendo este print why?
                println!("ThreadRR: {}", ROUND_ROBIN_THREADS[0].id);
                let thread:&'static mut ucontext_t  = &mut ROUND_ROBIN_THREADS[0].context;

                CURRENT_THREAD = thread;
                //Considerar el *swap_context*

                let context = getcontext(thread as *mut ucontext_t);

                if context == -1 {
                    let thread_to_remove = ROUND_ROBIN_THREADS[0];
                    for i in 0..ACTIVE_THREADS.len() {
                        if ACTIVE_THREADS[i].id == thread_to_remove.id {
                            ACTIVE_THREADS.remove(i);
                            break;
                        }
                    }
                }
                else {
                    setcontext(&mut ROUND_ROBIN_THREADS[0].context);
                    //swapcontext(thread, &ROUND_ROBIN_THREADS[0].context); // lo probé y da el mismo error
                }

                //swapcontext(thread, &ROUND_ROBIN_THREADS[0].context);
                //timer_create(0, 0 as *mut timer_settime, );
            }


        }
        else if lil_coin == 1{
            // Sorteo
            if !SORT_THREADS.is_empty(){
                SORT_THREADS = my_sched_sort(SORT_THREADS.clone());
                SORT_THREADS[0].state = State::Ready;

                // creo que aquí está el segmenation fault
                let thread:&'static mut ucontext_t  = &mut SORT_THREADS[0].context;
                CURRENT_THREAD = thread;
                //Considerar el *swap_context*

                setcontext(&SORT_THREADS[0].context);
                //swapcontext(parent_match(), &SORT_THREADS[0].context);
            }
        }

    }

}



