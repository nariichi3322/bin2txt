#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_dbg {
    ($( $args:expr ),*) => { dbg!( $( $args ),* ); }
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_dbg {
    ($( $args:expr ),*) => {
        ()
    };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_print {
    ($( $args:expr ),*) => { print!( $( $args ),* ); }
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_print {
    ($( $args:expr ),*) => {
        ()
    };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_println {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_println {
    ($( $args:expr ),*) => {
        ()
    };
}
