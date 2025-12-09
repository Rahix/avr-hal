use anyhow::Context as _;
use serde::{Deserialize, Serialize};
use std::{num::NonZeroU32, str::FromStr};

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
    let baudrate = val
        .as_ref()
        .map(|val| val.map_or_else(|| Ok(-1), |x| i32::try_from(x.get())))
        .transpose()
        .map_err(|e| serde::ser::Error::custom(format!("failed serializing baudrate: {e}")))?;

    baudrate.serialize(serializer)
}

fn deserialize_baudrate<'de, D>(deserializer: D) -> Result<Option<Option<NonZeroU32>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(match Option::<i32>::deserialize(deserializer)? {
        None => None,
        Some(-1) => Some(None),
        Some(baudrate) => Some(Some(
            u32::try_from(baudrate)
                .map_err(|_e| serde::de::Error::custom(format!("baudrate too high: {baudrate}")))
                .and_then(|b| {
                    NonZeroU32::new(b).ok_or_else(|| {
                        serde::de::Error::custom(format!("baudrate must not be zero: {baudrate}"))
                    })
                })?,
        )),
    })
}

impl RavedudeConfig {
    pub fn from_args(args: &crate::Args) -> anyhow::Result<Self> {
        Ok(Self {
            general_options: RavedudeGeneralConfig {
                open_console: args.open_console,
                console_port: args.console_port.clone(),
                serial_baudrate: match args.baudrate {
                    Some(serial_baudrate) => Some(
                        NonZeroU32::new(serial_baudrate)
                            .ok_or_else(|| anyhow::anyhow!("baudrate must not be 0"))?,
                    ),
                    None => None,
                },
                port: args.port.clone(),
                reset_delay: args.reset_delay,
                board: args.legacy_board_name().clone(),
                output_mode: args.output_mode.unwrap_or_default(),
                newline_after: None,
                newline_on: None,
            },
            board_config: Default::default(),
        })
    }
}

impl RavedudeGeneralConfig {
    pub fn newline_mode(&self) -> anyhow::Result<NewlineMode> {
        if self.output_mode == OutputMode::Ascii {
            if self.newline_on.is_some() || self.newline_on.is_some() {
                anyhow::bail!(
                    "newline_on and newline_after cannot be used with output_mode = \"ascii\""
                )
            }

            return Ok(NewlineMode::Off);
        }

        Ok(match (self.newline_on.as_ref(), self.newline_after) {
            (Some(_), Some(_)) => {
                anyhow::bail!("newline_on and newline_after cannot be used at the same time")
            }
            (Some(on_str), None) => NewlineMode::On(parse_newline_on(on_str)?),
            (None, Some(after)) => NewlineMode::After(after),
            (None, None) => NewlineMode::After(match self.output_mode {
                OutputMode::Hex | OutputMode::Dec => 16,
                OutputMode::Bin => 8,
                OutputMode::Ascii => unreachable!(),
            }),
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct RavedudeGeneralConfig {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub open_console: bool,
    pub console_port: Option<std::path::PathBuf>,
    pub serial_baudrate: Option<NonZeroU32>,
    pub port: Option<std::path::PathBuf>,
    pub reset_delay: Option<u64>,
    pub board: Option<String>,
    #[serde(default)]
    pub output_mode: OutputMode,
    pub newline_on: Option<String>,
    pub newline_after: Option<u8>,
}

impl RavedudeGeneralConfig {
    /// Apply command-line overrides to this configuration. Command-line arguments take priority over Ravedude.toml
    pub fn apply_overrides_from(&mut self, args: &crate::Args) -> anyhow::Result<()> {
        if args.open_console {
            self.open_console = true;
        }
        if let Some(console_port) = args.console_port.clone() {
            self.console_port = Some(console_port);
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
        if let Some(output_mode) = args.output_mode {
            self.output_mode = output_mode;
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
            // inherit is used to decide what BoardConfig to inherit and isn't used anywhere else
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
    // Inner option to represent whether the baudrate exists, outer option to allow for overriding.
    // Option<if baudrate == -1 { None } else { NonZeroU32(baudrate) }>
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

fn find_port(ports: &[BoardPortID]) -> anyhow::Result<std::path::PathBuf> {
    for serialport::SerialPortInfo {
        port_name,
        port_type,
    } in
        serialport::available_ports().context("failed fetching list of available serial ports")?
    {
        if let serialport::SerialPortType::UsbPort(usb_info) = port_type {
            for &BoardPortID { vid, pid } in ports {
                if usb_info.vid == vid && usb_info.pid == pid {
                    return Ok(port_name.into());
                }
            }
        }
    }
    Err(anyhow::anyhow!("Serial port not found."))
}

impl BoardConfig {
    pub fn guess_port(&self) -> Option<anyhow::Result<std::path::PathBuf>> {
        match &self.usb_info {
            Some(BoardUSBInfo::Error(err)) => Some(Err(anyhow::anyhow!(err.clone()))),
            Some(BoardUSBInfo::PortIds(ports)) => Some(find_port(ports)),
            None => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum OutputMode {
    #[default]
    Ascii,
    Hex,
    Dec,
    Bin,
}

impl FromStr for OutputMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ascii" => Ok(Self::Ascii),
            "hex" => Ok(Self::Hex),
            "dec" => Ok(Self::Dec),
            "bin" => Ok(Self::Bin),
            _ => Err(anyhow::anyhow!("unknown output mode")),
        }
    }
}

#[derive(Copy, Clone)]
pub enum NewlineMode {
    /// Break lines when encountering this byte
    On(u8),
    /// Break lines after this many bytes
    After(u8),
    Off,
}

impl NewlineMode {
    pub fn space_after(&self) -> Option<u8> {
        if let NewlineMode::After(bytes) = self {
            if bytes % 4 == 0 {
                return Some(4);
            }
        };
        None
    }
}

fn parse_newline_on(s: &str) -> Result<u8, anyhow::Error> {
    if let Ok(c) = s.parse::<char>() {
        return u8::try_from(c).context("non-byte character in `newline-on`");
    }

    // if it starts with 0x then parse the hex byte
    if &s[0..2] == "0x" {
        if s.len() != 4 {
            anyhow::bail!("hex byte must have 2 digits");
        }
        return u8::from_str_radix(&s[2..4], 16).context("invalid hex byte");
    }

    // if it starts with 0b then parse the binary byte
    if &s[0..2] == "0b" {
        if s.len() != 10 {
            anyhow::bail!("binary byte must have 8 digits");
        }
        return u8::from_str_radix(&s[2..10], 2).context("invalid binary byte");
    }

    anyhow::bail!("must be a single character or a byte in hex or binary notation");
}

#[cfg(test)]
mod tests {
    use super::parse_newline_on;

    #[test]
    fn test_parse_newline_on() {
        assert_eq!(parse_newline_on("a").unwrap(), 'a' as u8);
        assert_eq!(parse_newline_on("\n").unwrap(), '\n' as u8);
        assert_eq!(parse_newline_on("0x41").unwrap(), 0x41);
        assert_eq!(parse_newline_on("0b01000001").unwrap(), 0b01000001);
        assert!(parse_newline_on("not a char").is_err());
        assert!(parse_newline_on("0x").is_err());
        assert!(parse_newline_on("0xzz").is_err());
        assert!(parse_newline_on("0b").is_err());
        assert!(parse_newline_on("0b0a0a0a0a").is_err());
    }
}
