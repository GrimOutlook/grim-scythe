/// Log the error using the `log` crate and throw a panic with the same message
#[clippy::format_args]
#[macro_export]
macro_rules! perror {
    ($($arg:tt)+) => ({
        use log::error;
        error!($($arg)+);
        panic!($($arg)+);
    });
}
