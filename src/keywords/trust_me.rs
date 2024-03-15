#[macro_export]
macro_rules! trust_me {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}