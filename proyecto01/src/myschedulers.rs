use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void};
use std::mem;

static mut STACK_SIZE: usize = 10000;

//funcion para mapear schedulers
pub (crate) unsafe fn my_thread_change_sched(scheduler_type: isize, mut active_sched: isize) -> isize{
    if scheduler_type == 0 {active_sched = 0;}
    if scheduler_type == 1 {active_sched = 1;}
    if scheduler_type == 2 {active_sched = 2;}
    return active_sched;
}

/*
// función para alternar el scheduler
pub (crate) unsafe fn sched_alternator() {
        getcontext(&signal_context);

        signal_context.uc_stack.ss_sp = st1.as_mut_ptr() as *mut c_void;
        signal_context.uc_stack.ss_size = STACK_SIZE;
        signal_context.uc_stack.ss_flags = 0;

        sigemptyset(&mut signal_context.uc_sigmask);
        

        let alternator : u64 = 0;

        alternator = alternator^active_sched;

        my_thread_change_sched(alternator);

        if active_sched == 0 {
            makecontext(&mut child_temp as *mut ucontext_t, my_sched_round_robin, 1);
        }

        if active_sched == 1 {
            makecontext(&mut child_temp as *mut ucontext_t, my_sched_sort, 1);
        }

        if active_sched == 2 {
            makecontext(&mut child_temp as *mut ucontext_t, my_sched_real_time, 1);
        }

        swapcontext(&mut child_temp as *mut ucontext_t, parent_match() as *mut ucontext_t);
}
*/

pub fn my_sched_round_robin() {}

pub fn my_sched_sort() {}

pub fn my_sched_real_time() {}