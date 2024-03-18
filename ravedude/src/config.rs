use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
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
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ResetOptions<'a> {
    automatic: bool,
    #[serde(default)]
    message: Option<&'a str>,
}

fn serialize_reset_message<S>(val: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let reset_options = ResetOptions {
        automatic: val.is_none(),
        message: val.as_deref(),
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

    Ok(reset_options.message.map(ToOwned::to_owned))
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BoardAvrdudeOptions {
    pub programmer: String,
    pub partno: String,
    pub baudrate: Option<NonZeroU32>,
    pub do_chip_erase: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BoardUSBInfo {
    pub port_ids: Option<Vec<BoardPortID>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BoardPortID {
    pub vid: u16,
    pub pid: u16,
}
