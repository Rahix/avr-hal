use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

use crate::warning;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct BoardConfig {
    pub name: Option<String>,
    pub inherit: Option<String>,
    #[serde(
        serialize_with = "serialize_reset_message",
        deserialize_with = "deserialize_reset_message",
        rename = "reset"
    )]
    pub reset_message: Option<Option<String>>,
    pub avrdude: Option<BoardAvrdudeOptions>,
    pub usb_info: Option<BoardUSBInfo>,

    #[serde(flatten)]
    pub overrides: BoardOverrides,
}

impl BoardConfig {
    pub fn merge(self, base: BoardConfig) -> Self {
        Self {
            name: self.name.or(base.name),
            // inherit is used to decide what BoardConfig to inherit and isn't used anywhere else
            inherit: None,
            reset_message: self.reset_message.or(base.reset_message),
            avrdude: self
                .avrdude
                .and_then(|avrdude| base.avrdude.map(|base_avrdude| avrdude.merge(base_avrdude))),
            usb_info: self.usb_info.or(base.usb_info),

            // overrides aren't related to the board
            overrides: self.overrides,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ResetOptions {
    automatic: bool,
    message: Option<String>,
}

fn serialize_reset_message<S>(
    val: &Option<Option<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let reset_options = val.as_ref().map(|val| ResetOptions {
        automatic: val.is_none(),
        message: val.clone(),
    });

    reset_options.serialize(serializer)
}

fn deserialize_reset_message<'de, D>(deserializer: D) -> Result<Option<Option<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let Some(reset_options) = Option::<ResetOptions>::deserialize(deserializer)? else {
        return Ok(None);
    };

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

    Ok(Some(reset_options.message))
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct BoardAvrdudeOptions {
    pub programmer: Option<String>,
    pub partno: Option<String>,
    #[serde(
        serialize_with = "serialize_baudrate",
        deserialize_with = "deserialize_baudrate"
    )]
    pub baudrate: Option<Option<NonZeroU32>>,
    pub do_chip_erase: Option<bool>,
}
impl BoardAvrdudeOptions {
    pub fn merge(self, base: Self) -> Self {
        Self {
            programmer: self.programmer.or(base.programmer),
            partno: self.partno.or(base.partno),
            baudrate: self.baudrate.or(base.baudrate),
            do_chip_erase: self.do_chip_erase.or(base.do_chip_erase),
        }
    }
}
fn serialize_baudrate<S>(val: &Option<Option<NonZeroU32>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let baudrate = val.as_ref().map(|val| val.map_or(-1, |x| x.get() as i32));

    baudrate.serialize(serializer)
}

fn deserialize_baudrate<'de, D>(deserializer: D) -> Result<Option<Option<NonZeroU32>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let Some(baudrate) = Option::<i32>::deserialize(deserializer)? else {
        return Ok(None);
    };

    match NonZeroU32::new(baudrate as _) {
        None if baudrate == -1 => Ok(Some(None)),
        Some(nonzero_baudrate) => Ok(Some(Some(nonzero_baudrate))),
        _ => Err(serde::de::Error::custom(format!(
            "invalid baudrate: {baudrate}"
        ))),
    }
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
