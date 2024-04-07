use std::collections::HashMap;

use crate::config;

fn get_all_boards() -> anyhow::Result<HashMap<String, config::BoardOptions>> {
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

pub fn get_board(board_name: Option<&str>) -> anyhow::Result<config::RavedudeConfig> {
    Ok(match board_name {
        Some(board_name) => {
            let mut all_boards = get_all_boards()?;

            config::RavedudeConfig {
                board_config: all_boards.remove(board_name).ok_or_else(|| {
                    let mut msg = format!("invalid board: {board_name}\n");

                    msg.push_str("valid boards:");

                    for board in all_boards.keys() {
                        msg.push('\n');
                        msg.push_str(&board);
                    }
                    anyhow::anyhow!(msg)
                })?,
                ..Default::default()
            }
        }
        None => {
            let file_contents = std::fs::read_to_string("Ravedude.toml")
                .map_err(|_| anyhow::anyhow!("no board given and couldn't find Ravedude.toml in project, either pass a board as an argument or make a Ravedude.toml."))?;

            let mut board: config::RavedudeConfig = toml::from_str(&file_contents)
                .map_err(|err| anyhow::anyhow!("invalid Ravedude.toml:\n{}", err))?;

            if let Some(inherit) = board.board_config.inherit.as_deref() {
                let base_board = get_board(Some(inherit))?.board_config;
                board.board_config = board.board_config.merge(base_board);
            }
            board
        }
    })
}

#[cfg(test)]
mod tests {
    use super::get_all_boards;

    #[test]
    fn validate_board_list() -> anyhow::Result<()> {
        let all_boards = get_all_boards()?;

        for board in all_boards.values() {
            assert!(board.name.is_some());
            assert!(board.inherit.is_none());
            assert!(board.reset.is_some());
            assert!(board.avrdude.is_some());
            let avrdude = board.avrdude.as_ref().unwrap();
            assert!(avrdude.programmer.is_some());
            assert!(avrdude.partno.is_some());
            assert!(avrdude.baudrate.is_some());
            assert!(avrdude.do_chip_erase.is_some());
        }

        Ok(())
    }
}
