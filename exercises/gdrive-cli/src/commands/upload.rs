use clap::{arg, Command};

pub fn upload_cmd() -> Command { 
    return Command::new("upload")
        .about("Upload a file to Google Drive")
        .arg(arg!(<LOCAL_FILE_PATH> "Relative path of the file to upload"))
        .arg_required_else_help(true)
        .arg(arg!(<REMOTE_FOLDER_NAME> "Folder path on Google Drive to upload to"))
        .arg_required_else_help(true);
}
