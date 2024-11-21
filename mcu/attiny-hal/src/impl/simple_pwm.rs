macro_rules! impl_mod_simple_pwm {
    (
        hal: crate::$hal:ident,
        $(impl: {
            $($impl:item)*
        },)?
    ) => {
        pub mod simple_pwm {
            use crate::$hal as hal;
            $($($impl)*)?
        }
    }
}

pub(crate) use impl_mod_simple_pwm;
