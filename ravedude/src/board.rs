use crate::avrdude;

pub trait Board {
    fn display_name(&self) -> &str;
    fn needs_reset(&self) -> bool;
    fn avrdude_options(&self) -> avrdude::AvrdudeOptions;
    fn guess_port(&self) -> Option<std::path::PathBuf>;
}

pub fn get_board(board: &str) -> Option<Box<dyn Board>> {
    Some(match board {
        "uno" => Box::new(ArduinoUno::new(UnoVariant::Uno)),
        "nano" => Box::new(ArduinoUno::new(UnoVariant::Nano)),
        "leonardo" => Box::new(ArduinoLeonardo),
        _ => return None,
    })
}

// ----------------------------------------------------------------------------

fn find_port_from_vid_pid_list(list: &[(u16, u16)]) -> Option<std::path::PathBuf> {
    for serialport::SerialPortInfo {
        port_name,
        port_type,
    } in serialport::available_ports().unwrap()
    {
        if let serialport::SerialPortType::UsbPort(usb_info) = port_type {
            for (vid, pid) in list.iter() {
                if usb_info.vid == *vid && usb_info.pid == *pid {
                    return Some(port_name.into());
                }
            }
        }
    }
    None
}

// ----------------------------------------------------------------------------

enum UnoVariant {
    Uno,
    Nano,
}

struct ArduinoUno {
    variant: UnoVariant,
}

impl ArduinoUno {
    pub fn new(variant: UnoVariant) -> Self {
        ArduinoUno { variant }
    }
}

impl Board for ArduinoUno {
    fn display_name(&self) -> &str {
        match self.variant {
            UnoVariant::Uno => "Arduino Uno",
            UnoVariant::Nano => "Arduino Nano",
        }
    }

    fn needs_reset(&self) -> bool {
        false
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: None,
        }
    }

    fn guess_port(&self) -> Option<std::path::PathBuf> {
        find_port_from_vid_pid_list(&[
            (0x2341, 0x0043),
            (0x2341, 0x0001),
            (0x2A03, 0x0043),
            (0x2341, 0x0243),
        ])
    }
}

struct ArduinoLeonardo;

impl Board for ArduinoLeonardo {
    fn display_name(&self) -> &str {
        "Arduino Leonardo"
    }

    fn needs_reset(&self) -> bool {
        true
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "avr109",
            partno: "atmega32u4",
            baudrate: None,
        }
    }

    fn guess_port(&self) -> Option<std::path::PathBuf> {
        find_port_from_vid_pid_list(&[
            (0x2341, 0x0036),
            (0x2341, 0x8036),
            (0x2A03, 0x0036),
            (0x2A03, 0x8036),
        ])
    }
}
