mod mypthread;

use crate::mypthread::{my_thread_create};
//use crate::animation::{init_animation, move_figure};
//use crate::mycanvas::{init_canvas};
mod mypthread_struct;
mod myschedulers;
/*
mod animation;
mod parser;
mod mymutex;
mod mycanvas;
*/

use libc::{swapcontext, ucontext_t};
use libc::c_char;
use std::mem;
use std::mem::transmute;

// Función de ejemplo
pub fn f1(context_save: *mut ucontext_t, context_load: *mut ucontext_t) {
    println!("INICIO 1");
    println!("FIN 1");
    unsafe {
        swapcontext(context_save, context_load);
    }
}
extern "C" fn f2() {
    println!("INICIO 2");
    println!("FIN 2");
}
/*
extern "C" fn mover_figura() {
    move_figure();
}*/

extern "C" fn f3() {
    println!("Hilo 3");
}

pub fn main() {
    //let mut datos = parser::parse("input.txt");
    //let i = 0;
    //init_canvas();

    unsafe {
        let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(transmute(f1), 1);
        let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(f2, 1);
        mypthread::run_threads();
    }
    return;
}
