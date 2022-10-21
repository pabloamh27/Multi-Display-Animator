use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};

pub(crate) fn load_file() -> Vec<String> {
    let mut file = File::open("/home/estudiante/Escritorio/S.O/proyecto1/sistemasoperativos-proyecto01/proyecto01/src/parser/animation.txt").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    return lines;
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

pub(crate) fn parse_object_args(data: Vec<String>) -> animation_args {

    let size: Vec<&str> = data[1].split(":").collect();

    let mut args = animation_args {
        weight: size[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
        height: size[1].split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
        scheduler: data[2].split(":").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
        duration: data[3].split(":").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
        start_pos: (data[4].split(":").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
                    data[4].split(":").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()),
        end_pos: (data[5].split(":").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(),
                    data[5].split(":").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()),
        rotation: data[6].split(":").collect::<Vec<&str>>()[1].parse::<i32>().unwrap(),
        ascii_object: get_ascii_object(data)
    };

    return args;
}


pub (crate) fn get_ascii_object(data: Vec<String>) -> Vec<String> {
    let mut ascii_object: Vec<String> = Vec::new();
    for i in 7..data.len() {
        if data[i].to_string() == "end" {continue; }
        else { ascii_object.push(data[i].to_string()); }
    }
    return ascii_object;
}

pub(crate) fn print_animation_args(args: &animation_args) {
    println!("weight: {}", args.weight);
    println!("height: {}", args.height);
    println!("scheduler: {}", args.scheduler);
    println!("duration: {}", args.duration);
    println!("start_pos: ({}, {})", args.start_pos.0, args.start_pos.1);
    println!("end_pos: ({}, {})", args.end_pos.0, args.end_pos.1);
    println!("rotation: {}", args.rotation);
    println!("----------------------------------------------\n");
    for line in &args.ascii_object {
        println!("{}", line);
    }
}



pub (crate) fn print_animation_args_vec(args: &Vec<String>) {
    for arg in args.iter() {
        println!("{}", arg);
    }
}