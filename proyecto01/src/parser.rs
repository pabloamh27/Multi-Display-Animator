use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};

pub (crate) fn read_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect("Error al abrir el archivo");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error al leer el archivo");
    let mut lines_vec = Vec::new();
    for line in contents.lines() {
        lines_vec.push(line.to_string());
    }
    return lines_vec;
}

pub (crate) struct animation_args {
    pub (crate) weight: i32,
    pub (crate) height: i32,
    pub (crate) scheduler: i32,
    pub (crate) duration: i32,
    pub (crate) start_pos: (i32, i32),
    pub (crate) end_pos: (i32, i32),
    pub (crate) rotation: i32,
    pub (crate) ascii_object: Vec<String>
}