mod mypthread;

use crate::mypthread::{my_thread_create};
mod mypthread_struct;
mod myschedulers;
//mod mymutex;


/*fn main() {
    print!("Hello, world!");
}*/

use libc::ucontext_t;
use libc::c_char;
use std::mem;

// Función de ejemplo
extern "C" fn f1() {
    println!("INICIO 1");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(0), rb_thread::child_match(1));}
    println!("FIN 1");
}
extern "C" fn f2() {
    println!("INICIO 2");
    //unsafe{rb_thread::thread_yield(rb_thread::child_match(1), rb_thread::child_match(0));}
    println!("FIN 2");
}

extern "C" fn f3() {
    println!("Hilo 3");
}
pub fn main() {
    unsafe {

        let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(f1, 1);
        let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(f2, 2);
        mypthread::run_threads(0);
        mypthread::run_threads(1);
    }



}
