use crate::parser::{AnimationArgs};


use ncurses::{initscr, noecho, mvwprintw, ll::newwin, WINDOW};
use libc::{c_uint, time, time_t, usleep};
use ncurses::ll::wrefresh;

//Varios tipos de delay que se usan mas adelante
static DELAY:u64 = 700000;
static DELAY_RUNNING:u32 = 800000;
static DELAY_BETWEEN_MOVES:u32 = 900000;

// Explosiones para acabar con el objeto
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

//Funcion que anima todo el movimiento del objeto
pub (crate) unsafe fn animation_fn(animation_struct: AnimationArgs) {
    let mut x_actual = animation_struct.start_pos.1;
    let mut y_actual = animation_struct.start_pos.0;

    let x_objective = animation_struct.end_pos.1;
    let y_objective = animation_struct.end_pos.0;

    let tiempo_fin: time_t = 5 + time(0 as *mut time_t);

    //Inicializa una nueva pantalla
    initscr();
    noecho();
    let canvas_window: WINDOW = newwin(animation_struct.height , animation_struct.weight , 0, 0);

    //Comienza un loop hasta que se llega al limite de tiempo (5 segundos) o se acaba la figura
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

                //Se mueve en uno si no ha llegado al objetivo
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


