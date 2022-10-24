use ncurses::{initscr, noecho, curs_set, FALSE, CURSOR_VISIBILITY, WINDOW};
use crate::animation;
use crate::animation::{monitor_info, monitor_queue, datos_objeto, config};

//static mut configuracion : config = config {x: 0, y: 0, tiempo: 0, scheduler: 0};

pub (crate) fn init_canvas() {
    let mut x = 0;
    let mut y = 0;
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /*let mut temp_monitor = *monitor_info {
        id: 0,
        width: 100,
        height: 100,
        canvas_window: 0 as *mut WINDOW,
        previo : *monitor_info,
       siguiente : *monitor_info,
    };

    while(temp_monitor!=0){
        temp.monitor.canvas_window = newwin(temp_monitor.height, temp_monitor.width, y, x);
        x += temp_monitor.width;
        box(temp_monitor.canvas_window, 0, 0);
        wrefresh(temp_monitor.canvas_window);
        temp_monitor = temp_monitor.siguiente;

     */

}