#[macro_export]
macro_rules! cpp_start {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}