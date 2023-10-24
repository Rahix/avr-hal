use crate::avrdude;

pub trait Board {
    fn display_name(&self) -> &str;
    fn needs_reset(&self) -> Option<&str>;
    fn avrdude_options(&self) -> avrdude::AvrdudeOptions;
    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>>;
}

pub fn get_board(board: &str) -> Option<Box<dyn Board>> {
    Some(match board {
        "uno" => Box::new(ArduinoUno),
        "nano" => Box::new(ArduinoNano),
        "nano-new" => Box::new(ArduinoNanoNew),
        "leonardo" => Box::new(ArduinoLeonardo),
        "micro" => Box::new(ArduinoMicro),
        "mega2560" => Box::new(ArduinoMega2560),
        "mega1280" => Box::new(ArduinoMega1280),
        "diecimila" => Box::new(ArduinoDiecimila),
        "promicro" => Box::new(SparkFunProMicro),
        "promini-5v" => Box::new(SparkFunProMini5V),
        "trinket-pro" => Box::new(TrinketPro),
        "trinket" => Box::new(Trinket),
        "nano168" => Box::new(Nano168),
        "duemilanove" => Box::new(ArduinoDuemilanove),
        _ => return None,
    })
}

// ----------------------------------------------------------------------------

fn find_port_from_vid_pid_list(list: &[(u16, u16)]) -> anyhow::Result<std::path::PathBuf> {
    for serialport::SerialPortInfo {
        port_name,
        port_type,
    } in serialport::available_ports().unwrap()
    {
        if let serialport::SerialPortType::UsbPort(usb_info) = port_type {
            for (vid, pid) in list.iter() {
                if usb_info.vid == *vid && usb_info.pid == *pid {
                    return Ok(port_name.into());
                }
            }
        }
    }
    Err(anyhow::anyhow!("Serial port not found."))
}

// ----------------------------------------------------------------------------

struct ArduinoUno;

impl Board for ArduinoUno {
    fn display_name(&self) -> &str {
        "Arduino Uno"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: None,
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(find_port_from_vid_pid_list(&[
            (0x2341, 0x0043),
            (0x2341, 0x0001),
            (0x2A03, 0x0043),
            (0x2341, 0x0243),
        ]))
    }
}

struct ArduinoMicro;

impl Board for ArduinoMicro {
    fn display_name(&self) -> &str {
        "Arduino Micro"
    }

    fn needs_reset(&self) -> Option<&str> {
        Some("Reset the board by pressing the reset button once.")
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "avr109",
            partno: "atmega32u4",
            baudrate: Some(115200),
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(find_port_from_vid_pid_list(&[
            (0x2341, 0x0037),
            (0x2341, 0x8037),
            (0x2A03, 0x0037),
            (0x2A03, 0x8037),
            (0x2341, 0x0237),
            (0x2341, 0x8237),
        ]))
    }
}

struct ArduinoNano;

impl Board for ArduinoNano {
    fn display_name(&self) -> &str {
        "Arduino Nano"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: Some(57600),
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(Err(anyhow::anyhow!("Not able to guess port")))
    }
}

struct ArduinoNanoNew;

impl Board for ArduinoNanoNew {
    fn display_name(&self) -> &str {
        "Arduino Nano (New Bootloader)"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: Some(115200),
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(Err(anyhow::anyhow!("Not able to guess port")))
    }
}

struct ArduinoLeonardo;

impl Board for ArduinoLeonardo {
    fn display_name(&self) -> &str {
        "Arduino Leonardo"
    }

    fn needs_reset(&self) -> Option<&str> {
        let a = self.guess_port();
        match a {
            Some(Ok(name)) => {
                match serialport::new(name.to_str().unwrap(), 1200).open() {
                    Ok(_) => {
                        std::thread::sleep(core::time::Duration::from_secs(1));
                        None
                    },
                    Err(_) => Some("Reset the board by pressing the reset button once.")
                }
            },
            _ => Some("Reset the board by pressing the reset button once.")
        }
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "avr109",
            partno: "atmega32u4",
            baudrate: None,
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(find_port_from_vid_pid_list(&[
            (0x2341, 0x0036),
            (0x2341, 0x8036),
            (0x2A03, 0x0036),
            (0x2A03, 0x8036),
        ]))
    }
}


struct ArduinoMega1280;

impl Board for ArduinoMega1280 {
    fn display_name(&self) -> &str {
        "Arduino Mega 1280"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega1280",
            baudrate: Some(57600),
            do_chip_erase: false,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
       // This board uses a generic serial interface id 0403:6001 which is too common for auto detection.
       Some(Err(anyhow::anyhow!("Unable to guess port.")))
    }
}


struct ArduinoMega2560;

impl Board for ArduinoMega2560 {
    fn display_name(&self) -> &str {
        "Arduino Mega 2560"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "wiring",
            partno: "atmega2560",
            baudrate: Some(115200),
            do_chip_erase: false,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(find_port_from_vid_pid_list(&[
            (0x2341, 0x0010),
            (0x2341, 0x0042),
            (0x2A03, 0x0010),
            (0x2A03, 0x0042),
            (0x2341, 0x0210),
            (0x2341, 0x0242),
        ]))
    }
}

struct ArduinoDiecimila;

impl Board for ArduinoDiecimila {
    fn display_name(&self) -> &str {
        "Arduino Diecimila"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega168",
            baudrate: Some(19200),
            do_chip_erase: false,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(Err(anyhow::anyhow!("Not able to guess port")))
    }
}

struct SparkFunProMicro;

impl Board for SparkFunProMicro {
    fn display_name(&self) -> &str {
        "SparkFun Pro Micro"
    }

    fn needs_reset(&self) -> Option<&str> {
        Some("Reset the board by quickly pressing the reset button **twice**.")
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "avr109",
            partno: "atmega32u4",
            baudrate: None,
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(find_port_from_vid_pid_list(&[
            (0x1B4F, 0x9205), //5V
            (0x1B4F, 0x9206), //5V
            (0x1B4F, 0x9203), //3.3V
            (0x1B4F, 0x9204), //3.3V
        ]))
    }
}

struct SparkFunProMini5V;

impl Board for SparkFunProMini5V {
    fn display_name(&self) -> &str {
        "SparkFun Pro Mini 5V (16MHz)"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: Some(57600),
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(Err(anyhow::anyhow!("Not able to guess port")))
    }
}

struct TrinketPro;

impl Board for TrinketPro {
    fn display_name(&self) -> &str {
        "Trinket Pro"
    }

    fn needs_reset(&self) -> Option<&str> {
        Some("Reset the board by pressing the reset button once.")
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "usbtiny",
            partno: "atmega328p",
            baudrate: None,
            do_chip_erase: false,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        None // The TrinketPro does not have USB-to-Serial.
    }
}

struct Trinket;

impl Board for Trinket {
    fn display_name(&self) -> &str {
        "Trinket"
    }

    fn needs_reset(&self) -> Option<&str> {
        Some("Reset the board by pressing the reset button once.")
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "usbtiny",
            partno: "attiny85",
            baudrate: None,
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        None // The Trinket does not have USB-to-Serial.
    }
}

struct Nano168;

impl Board for Nano168 {
    fn display_name(&self) -> &str {
        "Nano Clone (ATmega168)"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega168",
            baudrate: Some(19200),
            do_chip_erase: false,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(Err(anyhow::anyhow!("Not able to guess port")))
    }
}


struct ArduinoDuemilanove;

impl Board for ArduinoDuemilanove {
    fn display_name(&self) -> &str {
        "Arduino Duemilanove"
    }

    fn needs_reset(&self) -> Option<&str> {
        None
    }

    fn avrdude_options(&self) -> avrdude::AvrdudeOptions {
        avrdude::AvrdudeOptions {
            programmer: "arduino",
            partno: "atmega328p",
            baudrate: Some(57600),
            do_chip_erase: true,
        }
    }

    fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        Some(Err(anyhow::anyhow!("Not able to guess port")))
    }
}
