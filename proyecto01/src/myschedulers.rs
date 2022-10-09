use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void, sigemptyset};
use std::mem;
use crate::mypthread;

static mut STACK_SIZE: usize = 10000;

//funcion para mapear schedulers
pub (crate) unsafe fn my_thread_change_sched(scheduler_type: u64){
    if scheduler_type == 0 {mypthread::active_sched = 0;}
    if scheduler_type == 1 {mypthread::active_sched = 1;}
    if scheduler_type == 2 {mypthread::active_sched = 2;}
}

// función para alternar el scheduler
pub (crate) unsafe fn sched_alternator() {
        //Creacion del Signal Context***
        let mut signal_context: ucontext_t = mem::uninitialized(); 
        //Creacion del Signal Context***

        getcontext(&mut signal_context);
    
        let mut st1: [c_char; 8192] = [mem::zeroed(); 8192];
        signal_context.uc_stack.ss_sp = st1.as_mut_ptr() as *mut c_void;
        signal_context.uc_stack.ss_size = STACK_SIZE;
        signal_context.uc_stack.ss_flags = 0;

        sigemptyset(&mut signal_context.uc_sigmask);
        

        let mut alternator : u64 = 0;

        alternator = alternator^mypthread::active_sched;

        my_thread_change_sched(alternator);

        if mypthread::active_sched == 0 {
            makecontext(&mut signal_context as *mut ucontext_t, my_sched_round_robin, 1);
        }

        if mypthread::active_sched == 1 {
            makecontext(&mut signal_context as *mut ucontext_t, my_sched_sort, 1);
        }

        if mypthread::active_sched == 2 {
            makecontext(&mut signal_context as *mut ucontext_t, my_sched_real_time, 1);
        }

        swapcontext(mypthread::CURRENT_THREAD ,&mut signal_context as *mut ucontext_t);
}


pub extern "C" fn my_sched_round_robin() {}

pub extern "C" fn my_sched_sort() {}

pub extern "C" fn my_sched_real_time() {}