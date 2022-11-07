
pub use atmega_hal::port::mode;
pub use atmega_hal::port::Pin;

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins connected to peripherals **AnalysIR KontroLIR**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][pins] macro.
    pub struct Pins from atmega_hal::Pins {
        /// Infrared receiver 1 (Optional part)
        pub ir_rx1: atmega_hal::port::PD2 = pd2,
        /// Infrared receiver 2 (Optional part)
        pub ir_rx2: atmega_hal::port::PD3 = pd3,
        /// Receiver power
        pub ir_rx_pwr: atmega_hal::port::PD5 = pd5,
        /// Infrared transmitter (Can be used with Timer1)
        pub ir_tx: atmega_hal::port::PB1 = pb1,
        /// Front panel led
        pub led: atmega_hal::port::PE3 = pe3,

        /// Keyboard matrix row1
        pub kbd_r1: atmega_hal::port::PB0 = pb0,
        /// Keyboard matrix row2
        pub kbd_r2: atmega_hal::port::PB2 = pb2,
        /// Keyboard matrix row3
        pub kbd_r3: atmega_hal::port::PB3 = pb3,
        /// Keyboard matrix row4
        pub kbd_r4: atmega_hal::port::PB4 = pb4,
        /// Keyboard matrix row5
        pub kbd_r5: atmega_hal::port::PB5 = pb5,
        /// Keyboard matrix row6
        pub kbd_r6: atmega_hal::port::PD6 = pd6,
        /// Keyboard matrix row7
        pub kbd_r7: atmega_hal::port::PD7 = pd7,
        /// Keyboard matrix column 1
        pub kbd_c1: atmega_hal::port::PC0 = pc0,
        /// Keyboard matrix column 2
        pub kbd_c2: atmega_hal::port::PC1 = pc1,
        /// Keyboard matrix column 3
        pub kbd_c3: atmega_hal::port::PC2 = pc2,
        /// Keyboard matrix column 4
        pub kbd_c4: atmega_hal::port::PC3 = pc3,
        /// Keyboard matrix column 5
        pub kbd_c5: atmega_hal::port::PE0 = pe0,
        /// Keyboard matrix column 6
        pub kbd_c6: atmega_hal::port::PE1 = pe1,
        /// Keyboard matrix column 7
        pub kbd_c7: atmega_hal::port::PE2 = pe2,

        /// Serial rx
        pub rxd: atmega_hal::port::PD0 = pd0,
        /// Serial tx
        pub txd: atmega_hal::port::PD1 = pd1,

        // I2C pins connected to eeprom (Optional part)
        /// EEPROM power
        pub eeprom_pwr: atmega_hal::port::PD4 = pd4,
        /// EEPROM I2c clock
        pub eeprom_sclk: atmega_hal::port::PC4 = pc4,
        /// EEPROM I2c data
        pub eeprom_sda: atmega_hal::port::PC5 = pc5,
    }
}
