mod mypthread;

use crate::mypthread::{CURRENT_THREAD, EXIT_CONTEXT, my_thread_create};
//use crate::animation::{init_animation, move_figure};
use crate::mycanvas::{init_canvas};
mod mypthread_struct;
mod myschedulers;
use animation::{EXPLOSION_INICIO, EXPLOSION_MITAD, EXPLOSION_FINAL};
use ncurses;


#[path = "parser.rs"] mod parser;


mod animation;

mod mymutex;

mod mycanvas;

use std::time;
use std::thread::sleep;
use libc::{getcontext, setcontext, swapcontext, ucontext_t};
use libc::c_char;
use std::mem;
use std::mem::transmute;

// Función de ejemplo
extern "C" fn f1() {
    unsafe {
    println!("{}", EXPLOSION_INICIO);
        sleep(time::Duration::from_millis(350));
        std::process::Command::new("clear").status().unwrap();
    println!("{}", EXPLOSION_MITAD);
        sleep(time::Duration::from_millis(350));
        std::process::Command::new("clear").status().unwrap();
    println!("{}", EXPLOSION_FINAL);
        setcontext(EXIT_CONTEXT);
    }
}
extern "C" fn f2() {
    println!("INICIO 2");
    println!("FIN 2");
    unsafe {
        let thread : &'static mut ucontext_t  = &mut *EXIT_CONTEXT;

        setcontext(thread);
    }
}
/*
extern "C" fn mover_figura() {
    move_figure();
}*/

pub fn main() {
    //let mut datos = parser::parse("input.txt");
    //let i = 0;
    //init_canvas();

    unsafe {
        mypthread::init_context_run();
        //let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(transmute::<fn(*mut ucontext_t), extern "C" fn()>(f1), 1);
        //let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(transmute::<fn(*mut ucontext_t), extern "C" fn()>(f2), 1);
        let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(f1, 1);
        let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(f2, 1);
        mypthread::run_threads();
    }
    return;


}
