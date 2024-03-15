#[macro_export]
macro_rules! showtime {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}