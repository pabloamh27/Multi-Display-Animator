mod mypthread;

use crate::mypthread::{child_match, CURRENT_THREAD, EXIT_CONTEXT, start_manager, my_thread_create, my_thread_yield};
//use crate::animation::{init_animation, move_figure};
use crate::mycanvas::{init_canvas};
mod mypthread_struct;
mod myschedulers;
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
extern "C" fn thread(input: &mut String) {
    println!("{}", input);
    unsafe  {
        my_thread_yield(CURRENT_THREAD,EXIT_CONTEXT);
    }
}


pub fn main() {
    let data = load_file();

    let object: parser::animation_args = parse_object_args(data);

    print_animation_args(&object);
    //init_canvas();
    //init_animation();
    unsafe {
        //Se ingresa el tipo de scheduler a correr y el quantum a seleccionar
        /*
        1: Round Robin
        2: Sort
        3: Real Time
        Recomendamos un Quantum de 200
        */
        //Prueba con Round Robin
        let mut scheduler_type: isize = 1;
        //Prueba con Sorteo
        //let mut scheduler_type: isize = 2;
        //Prueba con Real Time
        //let mut scheduler_type: isize = 3;
        let mut thread_manager = start_manager(scheduler_type, 200);
        let mut text_vec : Vec<&mut String> = Vec::new();
        let mut text: &mut String = &mut "Primer hilo".to_string();
        text_vec.push(text);
        let mut text:&mut String = &mut "Segundo Hilo".to_string();
        text_vec.push(text);
        let mut text:&mut String = &mut "Tercer hilo".to_string();
        text_vec.push(text);
        let mut text: &mut String = &mut "Cuarto hilo".to_string();
        text_vec.push(text);
        for i in text_vec{
            my_thread_create(transmute::<extern "C" fn(&mut String),extern "C" fn()>(thread), 1, i);
        }
        thread_manager.run_threads();
    }
    return;


}
