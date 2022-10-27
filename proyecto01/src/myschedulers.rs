use libc::{c_char, swapcontext, makecontext, getcontext, ucontext_t, c_void, sigemptyset, timer_settime, setcontext};
use std::mem;
use rand::Rng;
use crate::mypthread::{CURRENT_THREAD, THREADS, my_thread_create, my_thread_yield, child_match, EXIT_CONTEXT, active_sched, DEAD_THREADS, ACTIVE_THREADS};
use crate::mypthread_struct::{Thread, State, get_state};


static mut STACK_SIZE: usize = 10000;
static mut LOTTERY_TICKETS: Vec<i32> =  Vec::new();

//funcion para mapear schedulers
pub (crate) unsafe fn my_thread_change_sched(scheduler_type: u64){
    if scheduler_type == 0 {active_sched = 0;}
    if scheduler_type == 1 {active_sched = 1;}
    if scheduler_type == 2 {active_sched = 2;}
}
/*
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
*/

pub (crate) unsafe fn my_scheduler_round_robin(){
    let mut i = 0;
    while THREADS.len() != DEAD_THREADS.len() && active_sched == 1 {
        println!("{}", THREADS.len());
        println!("{}", DEAD_THREADS.len());
        CURRENT_THREAD = &mut THREADS[i].context as *mut ucontext_t;
        println!("Hilo: {}", THREADS[i].id);
        my_thread_yield(EXIT_CONTEXT, child_match(i) as *const ucontext_t);
        DEAD_THREADS.push(THREADS[i]);
        i += 1;
    }
}


pub (crate) unsafe fn my_scheduler_sort() {
    while  THREADS.len() != 0 && active_sched == 2{
        println!("{}", THREADS.len());
        println!("{}", DEAD_THREADS.len());
        for i in THREADS.clone() {
            let mut tickets = i.tickets;
            for j in 0..tickets {
                LOTTERY_TICKETS.push(i.id as i32);
            }
        }
        let mut random = rand::thread_rng().gen_range(0..LOTTERY_TICKETS.len());
        let mut winner = LOTTERY_TICKETS[random] as usize;
        CURRENT_THREAD = &mut THREADS[winner].context as *mut ucontext_t;
        println!("Hilo: {}", THREADS[winner].id);
        my_thread_yield(EXIT_CONTEXT, child_match(winner) as *const ucontext_t);
        DEAD_THREADS.push(THREADS[winner]);
        THREADS.remove(winner);

    }
}


pub (crate) unsafe fn my_scheduler_real_time() {}