#[macro_export]
macro_rules! i_hereby_declare_that_i_fully_understand_that_any_consequence_is_my_own_liability {
    ($($body:tt)*) => {
        unsafe { $($body)* }
    };
}