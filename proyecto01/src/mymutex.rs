use std::arch::asm;
use std::thread::sleep;
use std::time::Duration;
use libc::usleep;

//Funcion que se encarga de inicializar el lock
pub (crate) fn init_mutex(mutex: &mut i32) {
    mutex = &mut 0;
}


/*
REVISAR ESTA FUNCION
*/
//Funcion que se encarga de realizar guardar un nuevo valor en la variable deseada y lo retorna atomicamente seguro
/*pub (crate) fn atomic_change (mutex: &mut i32) -> i32{
    let mut aux = 1;
    asm!(
        "xchgl %0, %1;\n"
        : "=r"(aux), "+m"(mutex)
        : "0"(aux)
        : "memory");
    return aux;
}*/

//Funcion que se encarga de llamar a la funcion de atomic_exchange
pub (crate) fn test_and_set(mutex: &mut i32) -> i32 {
    return atomic_change(mutex);
}

//Funcion que se encarga de romper el mutex
pub (crate) fn destroy_mutex(mutex: &mut i32) {
    drop(mutex);
}


/*
REVISAR ESTA FUNCION
*/
//Funcion que se encarga de bloquear el mutex
pub (crate) fn lock_mutex(mutex: &mut i32) {
    while (mutex){
        sleep(Duration::from_millis(1000));  
    }
    test_and_set(mutex);
}


/*
REVISAR ESTA FUNCION
*/
//Funcion que se encarga de desbloquear el mutex
/*pub (crate) fn unlock_mutex(mutex: &mut i32){
    let mut aux = 0;
    asm!(
        "xchgl %0, %1;\n"
        : "=r"(aux), "+m"(*lock)
        : "0"(aux)
        :"memory");
}*/


/*
REVISAR ESTA FUNCION
*/
//Funcion que se encarga de intentar bloquear el mutex
pub (crate) unsafe fn try_lock_mutex(mutex: &mut i32){
    while(mutex){
        usleep(1000);
    }
    test_and_set(mutex);
}

