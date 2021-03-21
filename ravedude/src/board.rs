use crate::avrdude;

pub trait Board {
    fn display_name(&self) -> &str;
    fn needs_reset(&self) -> bool;
    fn avrdude_options(&self) -> avrdude::AvrdudeOptions;
    fn guess_port(&self) -> Option<std::path::PathBuf>;
}

pub fn get_board(board: &str) -> Option<Box<dyn Board>> {
    Some(match board {
        "uno" => Box::new(ArduinoUno),
        "nano" => Box::new(ArduinoNano),
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

struct ArduinoUno;

impl Board for ArduinoUno {
    fn display_name(&self) -> &str {
        "Arduino Uno"
    }

    fn needs_reset(&self) -> bool {
        false
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: None,
            do_chip_erase: true,
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

struct ArduinoNano;

impl Board for ArduinoNano {
    fn display_name(&self) -> &str {
        "Arduino Nano"
    }

    fn needs_reset(&self) -> bool {
        false
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: Some(57600),
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<std::path::PathBuf> {
        None
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
            do_chip_erase: true,
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
