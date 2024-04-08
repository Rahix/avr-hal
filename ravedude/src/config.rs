use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RavedudeConfig {
    #[serde(rename = "general")]
    pub general_options: RavedudeGeneralConfig,

    #[serde(rename = "board")]
    pub board_config: Option<BoardConfig>,
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

impl RavedudeConfig {
    pub fn from_args(args: &crate::Args) -> anyhow::Result<Self> {
        Ok(Self {
            general_options: RavedudeGeneralConfig {
                open_console: args.open_console.then_some(true),
                serial_baudrate: match args.baudrate {
                    Some(serial_baudrate) => Some(
                        NonZeroU32::new(serial_baudrate)
                            .ok_or_else(|| anyhow::anyhow!("baudrate must not be 0"))?,
                    ),
                    None => None,
                },
                port: args.port.clone(),
                reset_delay: args.reset_delay,
                board: args.board.clone(),
            },
            board_config: Default::default(),
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RavedudeGeneralConfig {
    pub open_console: Option<bool>,
    pub serial_baudrate: Option<NonZeroU32>,
    pub port: Option<std::path::PathBuf>,
    pub reset_delay: Option<u64>,
    pub board: Option<String>,
}

impl RavedudeGeneralConfig {
    pub fn apply_overrides(&mut self, args: &crate::Args) -> anyhow::Result<()> {
        // command line args take priority over Ravedude.toml
        if args.open_console {
            self.open_console = Some(true);
        }
        if let Some(serial_baudrate) = args.baudrate {
            self.serial_baudrate = Some(
                NonZeroU32::new(serial_baudrate)
                    .ok_or_else(|| anyhow::anyhow!("baudrate must not be 0"))?,
            );
        }
        if let Some(port) = args.port.clone() {
            self.port = Some(port);
        }
        if let Some(reset_delay) = args.reset_delay {
            self.reset_delay = Some(reset_delay);
        }
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct BoardConfig {
    pub name: Option<String>,
    pub inherit: Option<String>,
    pub reset: Option<ResetOptions>,
    pub avrdude: Option<BoardAvrdudeOptions>,
    pub usb_info: Option<BoardUSBInfo>,
}

impl BoardConfig {
    pub fn merge(self, base: BoardConfig) -> Self {
        Self {
            name: self.name.or(base.name),
            // inherit is used to decide what BoardGeneralOptions to inherit and isn't used anywhere else
            inherit: None,
            reset: self.reset.or(base.reset),
            avrdude: match self.avrdude {
                Some(avrdude) => base.avrdude.map(|base_avrdude| avrdude.merge(base_avrdude)),
                None => base.avrdude,
            },
            usb_info: self.usb_info.or(base.usb_info),
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
