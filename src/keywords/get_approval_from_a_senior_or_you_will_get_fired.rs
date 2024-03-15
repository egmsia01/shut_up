#[macro_export]
macro_rules! get_approval_from_a_senior_or_you_will_get_fired {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}