use std::{collections::HashMap, path::Path};

use crate::config;

fn get_all_boards() -> anyhow::Result<HashMap<String, config::BoardConfig>> {
    toml::from_str(include_str!("boards.toml")).map_err(|err| {
        if cfg!(test) {
            anyhow::anyhow!(
                "boards.toml in ravedude source is invalid.\n{}",
                err.message()
            )
        } else {
            anyhow::anyhow!(
                "boards.toml in ravedude source is invalid. This is a bug, please report it!\n{}",
                err.message()
            )
        }
    })
}

pub fn get_board_from_name(board_name: &str) -> anyhow::Result<config::RavedudeConfig> {
    Ok(config::RavedudeConfig {
        board_config: Some(get_board_config_from_name(board_name)?),
        ..Default::default()
    })
}

fn get_board_config_from_name(board_name: &str) -> anyhow::Result<config::BoardConfig> {
    let mut all_boards = get_all_boards()?;

    all_boards.remove(board_name).ok_or_else(|| {
        let mut msg = format!("invalid board: {board_name}\n");

        msg.push_str("valid boards:");

        for board in all_boards.keys() {
            msg.push('\n');
            msg.push_str(board);
        }
        anyhow::anyhow!(msg)
    })
}

pub fn get_board_from_manifest(manifest_path: &Path) -> anyhow::Result<config::RavedudeConfig> {
    Ok({
        let file_contents = std::fs::read_to_string(manifest_path)
            .map_err(|err| anyhow::anyhow!("Ravedude.toml read error:\n{}", err))?;

        let mut board: config::RavedudeConfig = toml::from_str(&file_contents)
            .map_err(|err| anyhow::anyhow!("invalid Ravedude.toml:\n{}", err))?;

        if let Some(board_config) = board.board_config.as_ref() {
            if let Some(board_name) = board.general_options.board.as_deref() {
                anyhow::bail!(
                    "can't both have board in [general] and [board] section; set inherit = \"{}\" under [board] to inherit its options",
                    board_name
                )
            }
            if let Some(inherit) = board_config.inherit.as_deref() {
                let base_board = get_board_config_from_name(inherit)?;
                board.board_config = Some(board.board_config.take().unwrap().merge(base_board));
            }
        } else if let Some(board_name) = board.general_options.board.as_deref() {
            let base_board = get_board_config_from_name(board_name)?;
            board.board_config = Some(base_board);
        }
        board
    })
}

#[cfg(test)]
mod tests {
    use super::get_all_boards;

    #[test]
    fn validate_board_list() -> anyhow::Result<()> {
        let all_boards = get_all_boards()?;

        for (name, board) in all_boards.iter() {
            assert!(
                board.name.is_some(),
                "Board {name:?} doesn't have a `name` key"
            );
            assert!(
                board.inherit.is_none(),
                "Board {name:?} has illegal `inherit` key"
            );
            assert!(
                board.reset.is_some(),
                "Board {name:?} doesn't have a `reset` key"
            );
            assert!(
                board.avrdude.is_some(),
                "Board {name:?} doesn't have an `avrdude` key"
            );
            let avrdude = board.avrdude.as_ref().unwrap();
            assert!(
                avrdude.programmer.is_some(),
                "Board {name:?}'s avrdude options doesn't have a `programmer` key"
            );
            assert!(
                avrdude.partno.is_some(),
                "Board {name:?}'s avrdude options doesn't have a `partno` key"
            );
            assert!(
                avrdude.baudrate.is_some(),
                "Board {name:?}'s avrdude options doesn't have a `baudrate` key"
            );
            assert!(
                avrdude.do_chip_erase.is_some(),
                "Board {name:?}'s avrdude options doesn't have a `do_chip_erase` key"
            );
        }

        Ok(())
    }
}
