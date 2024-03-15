#[macro_export]
macro_rules! freestyle {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}