mod mypthread;

use crate::mypthread::{CURRENT_THREAD, EXIT_CONTEXT, init_handler, my_thread_create};
//use crate::animation::{init_animation, move_figure};
use crate::mycanvas::{init_canvas};
mod mypthread_struct;
mod myschedulers;
use animation::{EXPLOSION_INICIO, EXPLOSION_MITAD, EXPLOSION_FINAL};
use ncurses;
use crate::parser::{print_animation_args, parse_object_args, load_file};

#[path = "parser/parser.rs"] mod parser;


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
    }
}

extern "C" fn f2() {
    println!("INICIO 2");
    println!("FIN 2");
}

extern "C" fn f3() {
    println!("INICIO 3");
    println!("FIN 3");
}


pub fn main() {
    let data = load_file();

    let object: parser::animation_args = parse_object_args(data);

    print_animation_args(&object);
    //init_canvas();
    //init_animation();
    unsafe {
        let mut HANDLER = init_handler(1, 1);
        mypthread::init_context_run();
        //let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(transmute::<fn(*mut ucontext_t), extern "C" fn()>(f1), 1);
        //let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(transmute::<fn(*mut ucontext_t), extern "C" fn()>(f2), 1);
        let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(f1, 1);
        let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(f2, 1);
        let mut new_thread: mypthread_struct::Thread = mypthread::my_thread_create(f3, 1);
        HANDLER.start_threads();
    }
    return;


}
