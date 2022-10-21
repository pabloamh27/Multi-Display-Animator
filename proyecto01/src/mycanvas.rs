use ncurses::{initscr, noecho, curs_set, FALSE, CURSOR_VISIBILITY};
use crate::animation;
//use crate::animation::{monitor_info, monitor_queue, datos_objeto, config};

//static mut configuracion : config = config {x: 0, y: 0, tiempo: 0, scheduler: 0};

pub (crate) fn init_canvas() {
    let mut x = 0;
    let mut y = 0;
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut temp_monitor = *monitor_info {
        id: i32,
        width: i32,
        height: i32,
        canvas_window: *WINDOW,
        previo : *monitor_info,
       siguiente : *monitor_info,
    };

}