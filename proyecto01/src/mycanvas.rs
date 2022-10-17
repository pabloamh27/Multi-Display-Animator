use ncurses::{initscr, noecho, curs_set};

pub (crate) fn init_canvas() {
    let mut x = 0;
    let mut y = 0;
    initscr();
    noecho();
    curs_set(false);
}