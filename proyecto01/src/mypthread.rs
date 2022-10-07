use crate::mypthread_struct::{thread, State};
use crate::myschedulers::{my_sched_round_robin, my_sched_sort, my_sched_real_time};
use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void};
use std::mem;


static mut THREADS:Vec<thread> = Vec::new();
//static mut signal_context = ucontext_t::default();
//BRETEAR ESTO
static mut STACK_SIZE: usize = 10000;
static mut active_sched: i64 = 0;

pub (crate) fn my_thread_create(){
    let mut context: ucontext_t = unsafe { mem::zeroed() };
}

pub (crate) fn get_number_of_threads() -> u64 {
    let mut number_of_threads = 0;
    unsafe{
        for thread in THREADS {
            number_of_threads += 1;
        }
        return number_of_threads;
    }

}