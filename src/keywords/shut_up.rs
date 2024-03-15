#[macro_export]
macro_rules! shut_up {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}