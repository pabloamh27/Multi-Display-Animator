use libc::{ucontext_t};
use rand::Rng;
use crate::mypthread::{CURRENT_THREAD, THREADS, my_thread_yield, child_match, EXIT_CONTEXT, ACTIVE_SCHEDULER, DEAD_THREADS};
use crate::mypthread_struct::{Thread};
use crate::mypthread_struct::State::{Off};


//funcion para mapear schedulers
pub (crate) unsafe fn _my_thread_change_sched(scheduler_type: u64){
    if scheduler_type == 0 { ACTIVE_SCHEDULER = 0;}
    if scheduler_type == 1 { ACTIVE_SCHEDULER = 1;}
    if scheduler_type == 2 { ACTIVE_SCHEDULER = 2;}
}

pub (crate) unsafe fn my_scheduler_round_robin(){
    let mut i = 0;
    while THREADS.len() != DEAD_THREADS.len() && ACTIVE_SCHEDULER == 1 {
        println!("{}", THREADS.len());
        println!("{}", DEAD_THREADS.len());
        CURRENT_THREAD = &mut THREADS[i].context as *mut ucontext_t;
        println!("Hilo: {}", THREADS[i].id);
        my_thread_yield(EXIT_CONTEXT, child_match(i) as *const ucontext_t);
        THREADS[i].state = Off;
        DEAD_THREADS.push(THREADS[i]);
        i += 1;
    }
}

pub (crate) unsafe fn my_scheduler_lottery() {
    while  THREADS.len() != 0 && ACTIVE_SCHEDULER == 2 {
        let mut lottery_tickets : Vec<usize> = Vec::new();
        println!("{}", THREADS.len());
        for i in THREADS.clone() {
            let tickets = i.tickets;
            for _j in 0..tickets {
                lottery_tickets.push(i.id);
            }
        }
        let random = rand::thread_rng().gen_range(0..lottery_tickets.len());
        let winner = lottery_tickets[random]-1;
        if lottery_tickets.len() > 1 {
            CURRENT_THREAD = &mut THREADS[winner].context as *mut ucontext_t;
            println!("Hilo: {}", THREADS[winner].id);
            my_thread_yield(EXIT_CONTEXT, child_match(winner) as *const ucontext_t);
            THREADS[winner].state = Off;
            THREADS.remove(winner);
        }
        else {
            let winner = 0;
            CURRENT_THREAD = &mut THREADS[winner].context as *mut ucontext_t;
            println!("Hilo: {}", THREADS[winner].id);
            my_thread_yield(EXIT_CONTEXT, child_match(winner) as *const ucontext_t);
            THREADS[winner].state = Off;
            THREADS.remove(winner);
            break;
        }
        }
    }



pub (crate) unsafe fn get_min_execute_value() -> usize {
    let mut min: Thread;
    min = THREADS[0];

    let mut final_pos = 0;

    for i in 0..THREADS.len() {
        if THREADS[i].context.uc_stack.ss_size < min.context.uc_stack.ss_size {
            min = THREADS[i];
            final_pos = THREADS[i].id;
        }
    }
    return final_pos;
}

pub (crate) unsafe fn my_scheduler_real_time() {
    while THREADS.len() != 0 && ACTIVE_SCHEDULER == 3 {
        CURRENT_THREAD = &mut THREADS[get_min_execute_value()].context as *mut ucontext_t;
        println!("Hilo: {}", THREADS[get_min_execute_value()].id);
        my_thread_yield(EXIT_CONTEXT, child_match(get_min_execute_value()) as *const ucontext_t);
        THREADS.remove(get_min_execute_value());
    }
}