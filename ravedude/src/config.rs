use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct BoardConfig {
    pub name: Option<String>,
    pub inherit: Option<String>,
    pub reset: Option<ResetOptions>,
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
            reset: self.reset.or(base.reset),
            avrdude: match self.avrdude {
                Some(avrdude) => base.avrdude.map(|base_avrdude| avrdude.merge(base_avrdude)),
                None => base.avrdude,
            },
            usb_info: self.usb_info.or(base.usb_info),

            // overrides aren't related to the board
            overrides: self.overrides,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ResetOptions {
    pub automatic: bool,
    pub message: Option<String>,
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
    Ok(match Option::<i32>::deserialize(deserializer)? {
        None => None,
        Some(-1) => Some(None),
        Some(baudrate) => Some(Some(NonZeroU32::new(baudrate as _).ok_or_else(|| {
            serde::de::Error::custom(format!("invalid baudrate: {baudrate}"))
        })?)),
    })
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
            args.open_console = open_console;
        }
        if let Some(serial_baudrate) = self.serial_baudrate {
            args.baudrate = Some(serial_baudrate.get());
        }
        if let Some(port) = self.port.take() {
            args.port = Some(port);
        }
        if let Some(reset_delay) = self.reset_delay {
            args.reset_delay = Some(reset_delay);
        }
    }
}
