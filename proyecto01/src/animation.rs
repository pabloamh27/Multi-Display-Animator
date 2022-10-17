use crate::mymutex::{init_mutex, lock_mutex, destroy_mutex};


struct monitor_info {
    id: i32,
    width: i32,
    height: i32,
    canvas_window: *WINDOW,
    previo : *monitor_info,
    siguiente : *monitor_info,
}

struct monitor_queue {
    head: *monitor_info,
    tail: *monitor_info,
    tamano: i32,
}

struct datos_objeto {
    x_actual: i32,
    y_actual: i32,
    x_final: i32,
    y_final: i32,
    x_inicial: i32,
    y_inicial: i32,
    angulo: i32,
    tiempo_fin: i32,
    tiempo_inicio: i32,
    scheduler: i32,
}

struct config {
    x: i32,
    y: i32,
    tiempo: i32,
    scheduler: i32,
}

// Char para explosion de figura
static mut Top: char = {' '; ' '; ' '; ' '; ' '; ' ';' ';' ';' ';' ';' '};
static mut secTop: char ={' '; ' '; ' '; ' '; ' '; ' ';' ';' ';' ';' ';' '};
static mut Mid: char = {' '; ' '; ' '; ' '; ' '; ' ';' ';' ';' ';' ';' '};
static mut secBot: char ={' '; ' '; ' '; ' '; ' '; ' ';' ';' ';' ';' ';' '};
static mut Bot: char = {' '; ' '; ' '; ' '; ' '; ' ';' ';' ';' ';' ';' '};

// Char para explosion de figura
static mut TopExp: char =  {' '; ' '; ' '; ' '; ' '; ' ';' ';' ';' ';' '};
static mut sexTopExp: char ={' '; ' '; '*'; '*'; '*'; ' ';' ';' ';' ';' '};
static mut MidExp: char = {' '; ' '; '*'; '*'; '*'; '*';'*';' ';' ';' '};
static mut secBotExp: char = {' '; ' '; '*'; '*'; '*'; ' ';' ';' ';' ';' '};
static mut BotExp: char ={' '; ' '; ' '; ' '; ' '; ' ';' ';' ';' ';' '};

// Char para explosion de figura
static mut TopExpFinal: char ={' '; ' '; ' '; '*'; '*'; '*';' ';' ';' ';' '};
static mut secTopExpFinal: char ={' '; ' '; '*'; '*'; '*'; '*';'*';' ';' ';' '};
static mut MidExpFinal: char = {' '; '*'; '*'; '*'; '*'; '*';'*';'*';' ';' '};
static mut secBotExpFinal: char = {' '; ' '; '*'; '*'; '*'; '*';'*';' ';' ';' '};
static mut BotFinal: char = {' '; ' '; ' '; '*'; '*'; '*';' ';' ';' ';' '};

pub (crate) fn init_animation() {
    let mut mutex: i32 = 0;
    my_mutex::init_mutex(&mut mutex);
}

pub (crate) fn move_figure() {
    let mut mutex: i32 = 0;
    lock_mutex(&mut mutex);


}