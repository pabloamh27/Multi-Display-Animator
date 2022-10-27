mod mypthread;

use crate::mypthread::{CURRENT_THREAD, EXIT_CONTEXT, start_manager, my_thread_create, my_thread_yield};
//use crate::animation::{init_animation, move_figure};
mod mypthread_struct;
mod myschedulers;
use crate::parser::{parse_object_args, load_file, AnimationArgs};
use crate::animation::{animation_fn};

#[path = "parser/parser.rs"] mod parser;


mod animation;

mod mymutex;


use std::{mem::transmute};
use libc::exit;


pub (crate) static mut OBJECTS: Vec<AnimationArgs> =  Vec::new();


extern "C" fn thread(index : usize) {
    unsafe  {
    animation_fn(OBJECTS[index].clone());
        my_thread_yield( CURRENT_THREAD, EXIT_CONTEXT);
    }
}

pub fn main() {

    unsafe {
        let args = std::env::args().collect::<Vec<String>>();
        if args.len() < 4 {
            println!("El programa debe recibir un minimo de 3 parametros de la forma:\
            ./proyecto01 animar -c <ruta COMPLETA Y ABSOLUTA del archivo>");
            exit(69);
        }
        if args[1] != String::from("animar") {
            println!("La primer entrada debe ser la palabra 'animar'");
            exit(69);
        }
        if args[2] != String::from("-c") {
            println!("El segundo argumento debe ser -c para indicarle al programa que debe leer configuracion.");
            exit(69);
        }

        let file = &args[3];
        //Pofesor tiene que ponele el path completo, desde lo mas basico, no entiende relativos este carajo
        let data = load_file(file);

        OBJECTS = parse_object_args(data);
        //animation_fn(objects[1].clone());


        //Se ingresa el tipo de scheduler a correr y el quantum a seleccionar
        /*
        1: Round Robin
        2: Sort
        3: Real Time
        Recomendamos un Quantum de 200
        */
        //Prueba con Round Robin
        let scheduler_type: isize = 1;
        //Prueba con Sorteo
        //let scheduler_type: isize = 2;
        //Prueba con Real Time
        //let scheduler_type: isize = 3;
        let mut thread_manager = start_manager(scheduler_type) ;
        for i in 0..OBJECTS.len() {
            my_thread_create(transmute::<extern "C" fn(usize), extern "C" fn()>(thread),  i);
        }

        thread_manager.run_threads();
    }






}
