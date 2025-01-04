macro_rules! impl_simple_pwm {
    (
        board: $($board:ident)::+  $(,)?
    ) => {
        pub use $($board)::+::hal::simple_pwm::*;

}

}

pub(crate) use impl_simple_pwm;
