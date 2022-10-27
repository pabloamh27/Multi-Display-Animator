use crate::parser::{AnimationArgs};


use ncurses::{initscr, noecho, mvwprintw, ll::newwin, WINDOW};
use libc::{c_uint, time, time_t, usleep};
use ncurses::ll::wrefresh;


static DELAY:u64 = 700000;
static DELAY_RUNNING:u32 = 800000;
static DELAY_BETWEEN_MOVES:u32 = 900000;

/*
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
*/


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
    pub(crate)ascii_object: Vec<String>,
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


pub (crate) fn init_animation() {
    //init_mutex();
}*/
pub static mut TOP: &str = "                     *                       ";
pub static mut TOPMID: &str = "               *******                    ";
pub static mut MIDTOP: &str = "           ***************                ";
pub static mut MID: &str = "            *******************              ";
pub static mut MIDBOT: &str = "           ***************                ";
pub static mut BOTMID: &str = "               *******                    ";
pub static mut BOT: &str = "                     *                      ";

pub static mut TOPEXP: &str = "        ***       *********       **          ";
pub static mut TOPMIDEXP: &str = "   *        *********************       *     ";
pub static mut MIDTOPEXP: &str = "     *********************************       ";
pub static mut MIDEXP: &str = "  ***************************************    ";
pub static mut MIDBOTEXP: &str = " *    *********************************  **   ";
pub static mut BOTMIDEXP: &str = "      *     *********************             ";
pub static mut BOTEXP: &str = "  *               *********              **   ";

pub static mut TOPEXPFIN: &str = "                                              ";
pub static mut TOPMIDEXPFIN: &str = "                  ***                      ";
pub static mut MIDTOPEXPFIN: &str = "                *******                    ";
pub static mut MIDEXPFIN: &str = "             *************                 ";
pub static mut MIDBOTEXPFIN: &str = "                *******                    ";
pub static mut BOTMIDEXPFIN: &str = "                  ***                      ";
pub static mut BOTEXPFIN: &str = "                                          ";

pub (crate) unsafe fn animation_fn(animation_struct: AnimationArgs) {
    let mut x_actual = animation_struct.start_pos.1;
    let mut y_actual = animation_struct.start_pos.0;

    let x_objective = animation_struct.end_pos.1;
    let y_objective = animation_struct.end_pos.0;

    let tiempo_fin: time_t = 5 + time(0 as *mut time_t);

    initscr();
    noecho();


    let canvas_window: WINDOW = newwin(animation_struct.height , animation_struct.weight , 0, 0);

    loop {
            while x_actual != x_objective || animation_struct.start_pos.0 != y_objective {
                if y_actual <= y_objective {
                    mvwprintw(canvas_window, y_actual - 3, x_actual - 1, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual + 1, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual + 2, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual + 3, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual + 4, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual + 5, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual + 6, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual + 7, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual + 8, " ");
                    mvwprintw(canvas_window, y_actual - 3, x_actual + 9, " ");
                }
                if x_actual <= x_objective {
                    mvwprintw(canvas_window, y_actual - 2, x_actual - 1, " ");
                    mvwprintw(canvas_window, y_actual - 1, x_actual - 1, " ");
                    mvwprintw(canvas_window, y_actual, x_actual - 1, " ");
                    mvwprintw(canvas_window, y_actual + 1, x_actual - 1, " ");
                    mvwprintw(canvas_window, y_actual + 2, x_actual - 1, " ");
                }
                if x_actual >= x_objective {
                    mvwprintw(canvas_window, y_actual - 2, x_actual + 11, " ");
                    mvwprintw(canvas_window, y_actual - 1, x_actual + 11, " ");
                    mvwprintw(canvas_window, y_actual, x_actual + 11, " ");
                    mvwprintw(canvas_window, y_actual + 1, x_actual + 11, " ");
                    mvwprintw(canvas_window, y_actual + 2, x_actual + 11, " ");
                }
                if y_actual >= y_objective {
                    mvwprintw(canvas_window, y_actual + 3, x_actual - 1, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual + 1, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual + 2, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual + 3, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual + 4, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual + 5, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual + 6, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual + 7, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual + 8, " ");
                    mvwprintw(canvas_window, y_actual + 3, x_actual + 9, " ");
                }
                mvwprintw(canvas_window, y_actual - 2, x_actual, &animation_struct.ascii_object[0].clone() as &str);
                mvwprintw(canvas_window, y_actual - 1, x_actual, &animation_struct.ascii_object[1].clone() as &str);
                mvwprintw(canvas_window, y_actual , x_actual, &animation_struct.ascii_object[2].clone() as &str);
                mvwprintw(canvas_window, y_actual + 1, x_actual, &animation_struct.ascii_object[3].clone() as &str);
                mvwprintw(canvas_window, y_actual + 2, x_actual, &animation_struct.ascii_object[4].clone() as &str);
                if time(0 as *mut time_t) > tiempo_fin {
                    mvwprintw(canvas_window, y_actual - 3, x_actual, TOPEXPFIN);
                    mvwprintw(canvas_window, y_actual - 2, x_actual, TOPMIDEXPFIN);
                    mvwprintw(canvas_window, y_actual - 1, x_actual, MIDTOPEXPFIN);
                    mvwprintw(canvas_window, y_actual, x_actual, MIDEXPFIN);
                    mvwprintw(canvas_window, y_actual + 1, x_actual, MIDBOTEXPFIN);
                    mvwprintw(canvas_window, y_actual + 2, x_actual, BOTMIDEXPFIN);
                    mvwprintw(canvas_window, y_actual + 3, x_actual, BOTEXPFIN);
                    wrefresh(canvas_window);
                    usleep(DELAY_RUNNING as c_uint);
                    mvwprintw(canvas_window, y_actual - 3, x_actual, TOP);
                    mvwprintw(canvas_window, y_actual - 2, x_actual, TOPMID);
                    mvwprintw(canvas_window, y_actual - 1, x_actual, MIDTOP);
                    mvwprintw(canvas_window, y_actual, x_actual, MID);
                    mvwprintw(canvas_window, y_actual + 1, x_actual, MIDBOT);
                    mvwprintw(canvas_window, y_actual + 2, x_actual, BOTMID);
                    mvwprintw(canvas_window, y_actual + 3, x_actual, BOT);
                    wrefresh(canvas_window);
                    usleep(DELAY_RUNNING as c_uint);
                    mvwprintw(canvas_window, y_actual - 3, x_actual, TOPEXP);
                    mvwprintw(canvas_window, y_actual - 2, x_actual, TOPMIDEXP);
                    mvwprintw(canvas_window, y_actual - 1, x_actual, MIDTOPEXP);
                    mvwprintw(canvas_window, y_actual, x_actual, MIDEXP);
                    mvwprintw(canvas_window, y_actual + 1, x_actual, MIDBOTEXP);
                    mvwprintw(canvas_window, y_actual + 2, x_actual, BOTMIDEXP);
                    mvwprintw(canvas_window, y_actual + 3, x_actual, BOTEXP);
                    wrefresh(canvas_window);
                    usleep(DELAY as c_uint);
                    mvwprintw(canvas_window, y_actual - 3, x_actual, TOPEXPFIN);
                    mvwprintw(canvas_window, y_actual - 2, x_actual, TOPMIDEXPFIN);
                    mvwprintw(canvas_window, y_actual - 1, x_actual, MIDTOPEXPFIN);
                    mvwprintw(canvas_window, y_actual, x_actual, MIDEXPFIN);
                    mvwprintw(canvas_window, y_actual + 1, x_actual, MIDBOTEXPFIN);
                    mvwprintw(canvas_window, y_actual + 2, x_actual, BOTMIDEXPFIN);
                    mvwprintw(canvas_window, y_actual + 3, x_actual, BOTEXPFIN);
                    wrefresh(canvas_window);
                    break;
                }

                if y_actual < y_objective {
                    y_actual += 1;
                }
                if y_actual > y_objective {
                    y_actual -= 1;
                }
                if x_actual < x_objective {
                    x_actual += 1;
                }
                if x_actual > x_objective {
                    x_actual -= 1;
                }

                wrefresh(canvas_window);
                usleep(DELAY_BETWEEN_MOVES as c_uint);
            }
            break;
        }
    }


