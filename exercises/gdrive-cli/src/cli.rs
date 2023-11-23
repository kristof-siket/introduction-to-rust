use clap::Command;

use crate::commands::upload;

pub fn cli() -> Command {
    Command::new("gdrive")
        .version("0.1.0")
        .author("Kristof Siket")
        .about("Google Drive CLI")
        .subcommand(upload::upload_cmd()) // Use the upload module correctly
}
