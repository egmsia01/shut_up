#[macro_export]
macro_rules! i_am_your_father {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}