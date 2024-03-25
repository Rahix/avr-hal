use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

use crate::warning;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct BoardConfig {
    pub name: String,
    #[serde(
        serialize_with = "serialize_reset_message",
        deserialize_with = "deserialize_reset_message",
        rename = "reset"
    )]
    pub reset_message: Option<String>,
    pub avrdude: BoardAvrdudeOptions,
    pub usb_info: Option<BoardUSBInfo>,

    #[serde(flatten)]
    pub overrides: BoardOverrides,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ResetOptions {
    automatic: bool,
    message: Option<String>,
}

fn serialize_reset_message<S>(val: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let reset_options = ResetOptions {
        automatic: val.is_none(),
        message: val.clone(),
    };

    reset_options.serialize(serializer)
}

fn deserialize_reset_message<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let reset_options = ResetOptions::deserialize(deserializer)?;

    if reset_options.automatic && reset_options.message.is_some() {
        return Err(serde::de::Error::custom(
            "cannot have automatic reset with a message for non-automatic reset",
        ));
    }
    if !reset_options.automatic && reset_options.message.is_none() {
        return Err(serde::de::Error::custom(
            "non-automatic reset option must have a message with it",
        ));
    }

    Ok(reset_options.message)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct BoardAvrdudeOptions {
    pub programmer: String,
    pub partno: String,
    pub baudrate: Option<NonZeroU32>,
    pub do_chip_erase: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum BoardUSBInfo {
    PortIds(Vec<BoardPortID>),
    #[serde(rename = "error")]
    Error(String),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BoardPortID {
    pub vid: u16,
    pub pid: u16,
}

impl BoardConfig {
    pub fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        match &self.usb_info {
            Some(BoardUSBInfo::Error(err)) => Some(Err(anyhow::anyhow!(err.clone()))),
            Some(BoardUSBInfo::PortIds(ports)) => {
                for serialport::SerialPortInfo {
                    port_name,
                    port_type,
                } in serialport::available_ports().unwrap()
                {
                    if let serialport::SerialPortType::UsbPort(usb_info) = port_type {
                        for &BoardPortID { vid, pid } in ports {
                            if usb_info.vid == vid && usb_info.pid == pid {
                                return Some(Ok(port_name.into()));
                            }
                        }
                    }
                }
                Some(Err(anyhow::anyhow!("Serial port not found.")))
            }
            None => None,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct BoardOverrides {
    open_console: Option<bool>,
    serial_baudrate: Option<NonZeroU32>,
    port: Option<std::path::PathBuf>,
    reset_delay: Option<u64>,
}

impl BoardOverrides {
    pub fn apply_overrides(&mut self, args: &mut crate::Args) {
        // command line args take priority over Ravedude.toml
        if let Some(open_console) = self.open_console {
            if args.open_console {
                warning!(
                    "Overriding console with {} (was {} in Ravedude.toml)",
                    args.open_console,
                    open_console,
                );
            } else {
                args.open_console = open_console;
            }
        }
        if let Some(serial_baudrate) = self.serial_baudrate {
            if let Some(args_baudrate) = args.baudrate {
                warning!(
                    "Overriding baudrate with {} (was {} in Ravedude.toml)",
                    args_baudrate,
                    serial_baudrate
                );
            } else {
                args.baudrate = Some(serial_baudrate.get());
            }
        }
        if let Some(port) = self.port.take() {
            if let Some(ref args_port) = args.port {
                warning!(
                    "Overriding port with {} (was {} in Ravedude.toml)",
                    port.to_str().unwrap(),
                    args_port.to_str().unwrap()
                );
            } else {
                args.port = Some(port);
            }
        }
        if let Some(reset_delay) = self.reset_delay {
            if let Some(args_reset_delay) = args.reset_delay {
                warning!(
                    "Overriding reset delay with {} (was {} in Ravedude.toml)",
                    args_reset_delay,
                    reset_delay
                );
            } else {
                args.reset_delay = Some(reset_delay);
            }
        }
    }
}
