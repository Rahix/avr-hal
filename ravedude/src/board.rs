use std::collections::HashMap;

use crate::config;

pub fn get_board(board_name: Option<&str>) -> anyhow::Result<config::BoardConfig> {
    match board_name {
        Some(board_name) => {
            let mut all_boards: HashMap<String, config::BoardConfig> =
                toml::from_str(include_str!("boards.toml"))
                    .expect("boards.toml in ravedude source is invalid");

            all_boards.remove(board_name).ok_or_else(|| {
                let mut msg = format!("invalid board: {board_name}\n");

                msg.push_str("valid boards:");

                for board in all_boards.keys() {
                    msg.push('\n');
                    msg.push_str(&board);
                }
                anyhow::anyhow!(msg)
            })
        }
        None => {
            let file_contents = std::fs::read_to_string("Ravedude.toml")
                .map_err(|_| anyhow::anyhow!("no board given and couldn't find Ravedude.toml in project, either pass a board as an argument or make a Ravedude.toml."))?;
            Ok(toml::from_str(&file_contents)?)
        }
    }
}
