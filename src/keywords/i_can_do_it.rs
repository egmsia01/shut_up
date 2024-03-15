#[macro_export]
macro_rules! i_can_do_it {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}