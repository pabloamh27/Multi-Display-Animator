use std::io::BufRead;
use crate::parser::{parse_object_args, animation_args};


use ncurses::TRUE;
use ncurses::stdscr;
use ncurses::curs_set;
use ncurses::initscr;
use ncurses::noecho;
use ncurses::FALSE;
use ncurses::getmaxyx;
use ncurses::mvprintw;
use ncurses::refresh;
use ncurses::clear;
use ncurses::endwin;
use libc::{c_uint, usleep};
use ncurses::ll::newwin;
use crate::mypthread::{CURRENT_THREAD, EXIT_CONTEXT, my_thread_yield};

static DELAY:u64 = 10000;
static DELAY_running:u32 = 100000;
static LOOP:bool = true;


pub(crate) unsafe fn animation_fn(animation_struct: animation_args) {

    let mut x = animation_struct.start_pos.1;
    let mut y = animation_struct.start_pos.0;

    let mut x_objective = animation_struct.end_pos.1;
    let mut y_objective = animation_struct.end_pos.0;

    print!("{} {} {} {}", x, y, x_objective, y_objective);

    let mut canva_X = animation_struct.weight;
    let mut canva_Y = animation_struct.height;

    initscr();
    noecho();

    let mut window = stdscr();
    getmaxyx(window, &mut canva_Y, &mut canva_X);
    refresh();
    //x = max_X / 2;
    //y = max_Y / 2;

    let mut ascii = animation_struct.ascii_object.clone();

    /*while x != x_objective || y != y_objective {
        getmaxyx(window, &mut canva_Y, &mut canva_X);

        y = canva_Y / 2;
        clear();
        //ascii = rotate_90(&mut ascii).to_vec();
        for i in ascii.iter() {
            //unsafe{usleep(DELAY_running as c_uint)};
            mvprintw(y, x, &*i);
            if x != x_objective {
                if y != y_objective {
                    y += 1;
                }
                else {
                    x += 1;
                }
            }
            else {
                if y <= y_objective {
                    y += 1;
                }
                else {
                    continue;
                }
            }
            //unsafe{usleep(DELAY_running as c_uint)};

            /*mvprintw(y, x, &*i);
            y += 1;*/
        }

        refresh();

        unsafe{usleep(DELAY as c_uint)};

        if next_x == canva_X - 1 {
            refresh();
            //my_thread_yield(CURRENT_THREAD,EXIT_CONTEXT);
            //return;
        } else {
            next_x = x + direction;
        }
        if next_x >= canva_X - 10 || next_x < 0 {
            direction= -1;
        }
        else
        {
            x += direction;
        }
    }*/

    y = canva_Y / 2;

    while x <= x_objective && y <= y_objective{
        for i in ascii.iter() {
            mvprintw(y, x, &*i);
            y += 1;
        }
    }
    
    endwin();
    refresh();
    //my_thread_yield(CURRENT_THREAD,EXIT_CONTEXT);
}



/*def rotate90Clockwise(A):
N = len(A[0])
for i in range(N // 2):
    for j in range(i, N - i - 1):
        temp = A[i][j]
    A[i][j] = A[N - 1 - j][i]
A[N - 1 - j][i] = A[N - 1 - i][N - 1 - j]
A[N - 1 - i][N - 1 - j] = A[j][N - 1 - i]
A[j][N - 1 - i] = temp
*/



//use crate::mymutex::{init_mutex, lock_mutex, destroy_mutex};
/*use ncurses::{mvwprintw, WINDOW, wrefresh};
use crate::mypthread::{my_thread_yield};
use std::thread::sleep;
use std::time;
use libc::{time, time_t};
use crate::{CURRENT_THREAD, XIT_CONTEXT};

pub (crate) struct monitor_info {
    pub(crate)id: i32,
    pub(crate)width: i32,
    pub(crate)height: i32,
    pub(crate)canvas_window: *mut WINDOW,
    pub(crate)previo : *mut monitor_info,
    pub(crate)siguiente : *mut monitor_info,
}

pub (crate) struct monitor_queue {
    pub(crate)head: *mut monitor_info,
    pub(crate)tail: *mut monitor_info,
    pub(crate)size: i32,
}

pub (crate) struct datos_objeto {
    pub(crate)x_actual: i32,
    pub(crate)y_actual: i32,
    pub(crate)x_final: i32,
    pub(crate)y_final: i32,
    pub(crate)x_inicial: i32,
    pub(crate)y_inicial: i32,
    pub(crate)angulo: i32,
    pub(crate)tiempo_fin: i32,
    pub(crate)tiempo_inicio: i32,
    pub(crate)scheduler: i32,
    pub(crate)monitor_id: i32,
    pub(crate)ascii_item: Vec<String>,
}

pub (crate) struct config {
    pub(crate)protocolo: *mut char,
    pub(crate)numero_monitores: i32,
    pub(crate)monitor_list: *mut monitor_queue,
    pub(crate)item_list: *mut datos_objeto,
    pub(crate)espacio_entre_objetos: i32,
}
*/

/*
pub static mut top: &str = "                     *                       ";
pub static mut topSec: &str = "               *******                    ";
pub static mut midTop: &str = "           ***************                ";
pub static mut mid: &str = "            *******************              ";
pub static mut midBot: &str = "           ***************                ";
pub static mut botSec: &str = "               *******                    ";
pub static mut bot: &str = "                     *                      ";

pub static mut topExp: &str = "        ***       *********       **          ";
pub static mut topSecExp: &str = "   *        *********************       *     ";
pub static mut midTopExp: &str = "     *********************************       ";
pub static mut midExp: &str = "  ***************************************    ";
pub static mut midBotExp: &str = " *    *********************************  **   ";
pub static mut botSecExp: &str = "      *     *********************             ";
pub static mut botExp: &str = "  *               *********              **   ";

pub static mut topExpFin: &str = "                                              ";
pub static mut topSecExpFin: &str = "                  ***                      ";
pub static mut midTopExpFin: &str = "                *******                    ";
pub static mut midExpFin: &str = "             *************                 ";
pub static mut midBotExpFin: &str = "                *******                    ";
pub static mut botSecExpFin: &str = "                  ***                      ";
pub static mut botExpFin: &str = "                                          ";

pub (crate) fn init_animation() {
    //init_mutex();
}*/

/*
pub (crate) unsafe fn move_figure() {
    //mymutex::lock_mutex();
    let mut figure = datos_objeto {
        x_actual: 0,
        y_actual: 0,
        x_final: 0,
        y_final: 0,
        x_inicial: 0,
        y_inicial: 0,
        angulo: 0,
        tiempo_fin: 0,
        tiempo_inicio: 0,
        scheduler: 0,
        monitor_id: 0,
        ascii_item: Vec::new(),
    };
    // IMPLEMENTAR ESTO! datos_objeto *figure = configuracion.item_list;
    //mymutex::unlock_mutex();
    //mymutex::lock_mutex();
    // IMPLEMENTAR ESTO! monitor_info *temp_monitor = configuracion.monitor_list.head;
    let mut temp_monitor = monitor_info {
        id: 0,
        width: 0,
        height: 0,
        canvas_window: std::ptr::null_mut(),
        previo: std::ptr::null_mut(),
        siguiente: std::ptr::null_mut(),
    };
    while (temp_monitor.id != figure.monitor_id) {
        temp_monitor = temp_monitor.siguiente;
    }
    //mymutex::unlock_mutex();
    //mymutex::lock_mutex();
    figure.x_actual = figure.x_inicio;
    figure.y_actual = figure.y_inicio;
    //mymutex::unlock_mutex();

    while (1) {
        if (time(0 as *mut time_t) < figure.tiempo_inicio) {
            my_thread_yield(EXIT_CONTEXT, CURRENT_THREAD);
        } else {
            while (figure.x_actual != figure.x_final || figure.y_inicial != figure.y_final) {
                if (figure.y_actual <= figure.y_final) {
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual - 1, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 1, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 2, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 3, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 4, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 5, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 6, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 7, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 8, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 9, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual + 10, " ");
                }
                if (figure.x_actual <= figure.x_final) {
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 2, figure.x_actual - 1, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 1, figure.x_actual - 1, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual, figure.x_actual - 1, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 1, figure.x_actual - 1, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 2, figure.x_actual - 1, " ");
                }
                if (figure.x_actual >= figure.x_final) {
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 2, figure.x_actual + 11, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 1, figure.x_actual + 11, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual, figure.x_actual + 11, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 1, figure.x_actual + 11, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 2, figure.x_actual + 11, " ");
                }
                if (figure.y_actual >= figure.y_final) {
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual - 1, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 1, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 2, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 3, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 4, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 5, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 6, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 7, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 8, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 9, " ");
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual + 10, " ");
                }
                mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual, figure.ascii_item[0]);
                mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 2, figure.x_actual, figure.ascii_item[1]);
                mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 1, figure.x_actual, figure.ascii_item[2]);
                mvwprintw(*temp_monitor.canvas_window, figure.y_actual, figure.x_actual, figure.ascii_item[3]);
                mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 1, figure.x_actual, figure.ascii_item[4]);
                mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 2, figure.x_actual, figure.ascii_item[5]);
                mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual, figure.ascii_item[6]);
                if (time(0 as *mut time_t) > figure.tiempo_fin as time_t) {
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual, topExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 2, figure.x_actual, topSecExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 1, figure.x_actual, midTopExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual, figure.x_actual, midExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 1, figure.x_actual, midBotExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 2, figure.x_actual, botSecExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual, botExpFin);
                    wrefresh(*temp_monitor.canvas_window);
                    sleep(time::Duration::from_millis(800000));
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual, top);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 2, figure.x_actual, topSec);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 1, figure.x_actual, midTop);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual, figure.x_actual, mid);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 1, figure.x_actual, midBot);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 2, figure.x_actual, botSec);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual, bot);
                    wrefresh(*temp_monitor.canvas_window);
                    sleep(time::Duration::from_millis(800000));
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual, topExp);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 2, figure.x_actual, topSecExp);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 1, figure.x_actual, midTopExp);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual, figure.x_actual, midExp);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 1, figure.x_actual, midBotExp);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 2, figure.x_actual, botSecExp);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual, botExp);
                    wrefresh(*temp_monitor.canvas_window);
                    sleep(time::Duration::from_millis(700000));
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 3, figure.x_actual, topExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 2, figure.x_actual, topSecExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual - 1, figure.x_actual, midTopExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual, figure.x_actual, midExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 1, figure.x_actual, midBotExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 2, figure.x_actual, botSecExpFin);
                    mvwprintw(*temp_monitor.canvas_window, figure.y_actual + 3, figure.x_actual, botExpFin);
                    wrefresh(*temp_monitor.canvas_window);
                    break;
                }

                //mymutex::lock_mutex();

                if (figure.y_actual < figure.y_final) {
                    figure.y_actual += 1;
                }
                if (figure.y_actual > figure.y_final) {
                    figure.y_actual -= 1;
                }
                if (figure.x_actual < figure.x_final) {
                    figure.x_actual += 1;
                }
                if (figure.x_actual > figure.x_final) {
                    figure.x_actual -= 1;
                }
                //mymutex::unlock_mutex();

                wrefresh(*temp_monitor.canvas_window);
                sleep(time::Duration::from_millis(900000));
            }
            break;
        }
    }
}
*/
