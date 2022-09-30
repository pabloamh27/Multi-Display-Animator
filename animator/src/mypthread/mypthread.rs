extern crate libc;

use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void};
use std::mem;

mod mypthread_struct;
use mypthread_struct::create_thread;

static mut THREADS:Vec<threads> = Vec::new();

// todas las funciones de pthread deben hechas aquí


// función para crear un hilo
#[derive(Copy, Clone)]

// create my thread
pub fn create_thread(func: extern "C" fn(), thread_tickets: isize, scheduler_type: isize) -> thread{
    unsafe {
        let mut st1: [c_char; 8192] = [mem::zeroed(); 8192];
        
        let mut child_temp: ucontext_t = mem::uninitialized();
        getcontext(&mut child_temp as *mut ucontext_t);
        child_temp.uc_stack.ss_sp = st1.as_mut_ptr() as *mut c_void;
        child_temp.uc_stack.ss_size = mem::size_of_val(&st1);

        child_temp.uc_link = parent_match() as *mut ucontext_t;
       
        makecontext(&mut child_temp as *mut ucontext_t, func, 0);
        let thread_t = thread {id:(get_number_of_threads + 1), state: false, tickets: thread_tickets, 
            scheduler: scheduler_type, context: child_temp};
        //Thread creado
        THREADS.push(thread_t);
        THREADS_CONTEXT.push(Some(child_temp));
        return thread_t;
    }
}


// Pendiente de implementar de manera correcta
/*
fn assing_scheduler(thread: thread) -> thread{
    if thread.scheduler == 1{
        roundrobin_list.push(thread);
    }else if thread.scheduler == 2{
        sorteo_list.push(thread);
    }
    return thread;
}
*/


// función para iniciar un hilo
fn start_thread(thread: thread) -> thread {
    thread.state = true;
    return thread;
}

// función para terminar un hilo
fn my_thread_end(thread: thread) -> thread {
    thread.state = false;
    return thread;
}

// función para cambiar el estado de un hilo
fn change_state(thread: thread, state: Bool) -> thread {
    thread.state = state;
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

// función para agregar un hilo al thread pool
fn add_thread(thread_pool: Vec<thread>, thread: thread) {
    thread_pool.push(thread);
}

// funcion para ceder un hilo o su posicion en un principio 
// **MODIFICAR PARA CEDER UN HILO QUE ESTÉ EN EJECUCIÓN**
fn my_thread_yield(thread_id: u64, thread_pool: Vec<thread>) -> thread_pool {
    yielded_thread = get_thread_by_id(thread_id, thread_pool);
    thread_pool.retain(|&x| x != thread_id);
    yielded_thread.push(thread_pool);
    return thread;
}

// función para obtener un hilo por su id
fn get_thread_by_id(thread_id: u64, thread_pool: Vec<thread>) -> thread {
    for thread in thread_pool {
        if thread.id == thread_id {
            return thread;
        }
    }
}

// función para poner en espera un hilo hasta que sea acabado o finalizado
// **MODIFICAR EN CASO DE DAR PROBLEMAS EN UN FUTURO**
fn my_thread_join(thread: thread) -> thread {
    while thread.state != Off {
        thread.state = Waiting;
    }
    return thread;
}

// función para quitarle la responsabilidad a un hilo sobre alguna ejecución, le desasigna el hilo a una ejecución
fn my_thread_detach(thread: thread) -> thread {
    // https://www.google.com/search?channel=fs&client=ubuntu&q=detach+threading
    // detach thread function
    return thread;
}

