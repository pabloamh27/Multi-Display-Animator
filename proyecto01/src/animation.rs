/*use crate::mymutex::{init_mutex, lock_mutex, destroy_mutex};
use ncurses::{WINDOW};

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
    let mut mutex: i32 = 0;
    init_mutex(&mut mutex);
}

pub (crate) fn move_figure() {
    let mut mutex: i32 = 0;
    lock_mutex(&mut mutex);
}
*/