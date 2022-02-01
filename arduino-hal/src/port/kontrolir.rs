
pub use atmega_hal::port::mode;
pub use atmega_hal::port::Pin;

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins connected to peripherals **AnalysIR KontroLIR**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][pins] macro.
    pub struct Pins from atmega_hal::Pins {
        pub irin1: atmega_hal::port::PD2 = pd2,
        pub irin2: atmega_hal::port::PD3 = pd3,
        /// Infrared out - Timer1
        pub irout: atmega_hal::port::PB1 = pb1,
        /// Front panel led
        pub led: atmega_hal::port::PE3 = pe3,
        // Keyboard matrix pins
        pub row1: atmega_hal::port::PB0 = pb0,
        pub row2: atmega_hal::port::PB2 = pb2,
        pub row3: atmega_hal::port::PB3 = pb3,
        pub row4: atmega_hal::port::PB4 = pb4,
        pub row5: atmega_hal::port::PB5 = pb5,
        pub row6: atmega_hal::port::PD6 = pd6,
        pub row7: atmega_hal::port::PD7 = pd7,
        pub col1: atmega_hal::port::PC0 = pc0,
        pub col2: atmega_hal::port::PC1 = pc1,
        pub col3: atmega_hal::port::PC2 = pc2,
        pub col4: atmega_hal::port::PC3 = pc3,
        pub col5: atmega_hal::port::PE0 = pe0,
        pub col6: atmega_hal::port::PE1 = pe1,
        pub col7: atmega_hal::port::PE2 = pe2,

        // Serial
        pub rxd: atmega_hal::port::PD0 = pd0,
        pub txd: atmega_hal::port::PD1 = pd1,

        // I2C pins connected to eeprom
        pub sclk: atmega_hal::port::PC4 = pc4,
        pub sda: atmega_hal::port::PC5 = pc5,
    }
}
