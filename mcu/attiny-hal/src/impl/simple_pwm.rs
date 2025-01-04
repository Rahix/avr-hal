macro_rules! impl_mod_simple_pwm {
    ($($mod:item)*) => {
        pub mod simple_pwm {
            $($mod)*
        }
    }
}

pub(crate) use impl_mod_simple_pwm;
