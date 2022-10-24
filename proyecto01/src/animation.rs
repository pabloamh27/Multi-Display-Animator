//use crate::mymutex::{init_mutex, lock_mutex, destroy_mutex};
use ncurses::{WINDOW};
use crate::mypthread::{my_thread_yield};
use std::thread::sleep;

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


pub static mut EXPLOSION_INICIO: &str = "                      *                       \n
                                                        *******                    \n
                                                    ***************                \n
                                                  *******************              \n
                                                    ***************                \n
                                                        *******                    \n
                                                           *                      ";

pub static mut EXPLOSION_MITAD: &str =  "        ***       *********       **          \n
                                        *        *********************       *     \n
                                           *********************************       \n
                                        ***************************************    \n
                                      *    *********************************  **   \n
                                           *     *********************             \n
                                       *               *********              **   ";

pub static mut EXPLOSION_FINAL: &str =  "                                              \n
                                                          ***                      \n
                                                        *******                    \n
                                                     *************                 \n
                                                        *******                    \n
                                                          ***                      \n
                                                                                  ";

pub (crate) fn init_animation() {
    //init_mutex();
}

pub (crate) fn move_figure() {
    /*
    mymutex::lock_mutex();
    datos_objeto *figure = configuracion.item_list;
    mymutex::unlock_mutex();
    mymutex::lock_mutex();
    monitor_info *temp_monitor = configuracion.monitor_list->head;
    while (temp_monitor.id != figure.monitor_id) {
        temp_monitor = temp_monitor.siguiente;
    }
    mymutex::unlock_mutex();
    mymutex::lock_mutex();
    figure.x_actual = figure.x_inicio;
    figure.y_actual = figure.y_inicio;
    mymutex::unlock_mutex();

    while (1){
    if (time(0) < figure.tiempo_inicio) {
        my_thread_yield();
    }
    else{
        while (figure.x_actual != figure.x_final || figure.y_inicial != figure.y_final) {
            if (figure.y_actual <= figure.y_final) {
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual-1, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+1, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+2, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+3, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+4, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+5, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+6, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+7, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+8, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+9, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-3, figure.x_actual+10, " ");
            }
            if (figure.x_actual <= figure.x_final) {
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-2, figure.x_actual-1, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-1, figure.x_actual-1, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual, figure.x_actual-1, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+1, figure.x_actual-1, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+2, figure.x_actual-1, " ");
            }
            if (figure.x_actual >= figure.x_final) {
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-2, figure.x_actual+11, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-1, figure.x_actual+11, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual, figure.x_actual+11, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+1, figure.x_actual+11, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+2, figure.x_actual+11, " ");
            }
            if (figure.y_actual >= figure.y_final) {
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual-1, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+1, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+2, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+3, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+4, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+5, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+6, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+7, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+8, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+9, " ");
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+3, figure.x_actual+10, " ");
            }
            mvwprintw(temp_monitor.canvas_window, figure.y_actual-2, figure.x_actual, figure.ascii_item[0]);
            mvwprintw(temp_monitor.canvas_window, figure.y_actual-1, figure.x_actual, figure.ascii_item[1]);
            mvwprintw(temp_monitor.canvas_window, figure.y_actual, figure.x_actual, figure.ascii_item[2]);
            mvwprintw(temp_monitor.canvas_window, figure.y_actual+1, figure.x_actual, figure.ascii_item[3]);
            mvwprintw(temp_monitor.canvas_window, figure.y_actual+2, figure.x_actual, figure.ascii_item[4]);
            if (time(0) > figure.tiempo_fin) {
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-2, figure.x_actual, Top);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-1, figure.x_actual, SecTop);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual, figure.x_actual, mid);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+1, figure.x_actual, SecBot);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+2, figure.x_actual, Bot);
                wrefresh(temp_monitor.canvas_window);
                sleep(time::Duration::from_millis(800000));
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-2, figure.x_actual, TopExp);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-1, figure.x_actual, SecTopExp);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual, figure.x_actual, midExp);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+1, figure.x_actual, SecBotExp);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+2, figure.x_actual, BotExp);
                wrefresh(temp_monitor.canvas_window);
                sleep(time::Duration::from_millis(800000));
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-2, figure.x_actual, TopExpFin);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-1, figure.x_actual, SecTopExpFin);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual, figure.x_actual, midExpFin);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+1, figure.x_actual, SecBotExpFin);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+2, figure.x_actual, BotExpFin);
                wrefresh(temp_monitor.canvas_window);
                sleep(time::Duration::from_millis(700000));
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-2, figure.x_actual, Top);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual-1, figure.x_actual, SecTop);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual, figure.x_actual, mid);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+1, figure.x_actual, SecBot);
                mvwprintw(temp_monitor.canvas_window, figure.y_actual+2, figure.x_actual, Bot);
                wrefresh(temp_monitor.canvas_window);
                break;
            }

            mymutex::lock_mutex();

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
            mymutex::unlock_mutex();

            wrefresh(temp_monitor.canvas_window);
            sleep(time::Duration::from_millis(900000));
        }
        break;
    }

     */
}