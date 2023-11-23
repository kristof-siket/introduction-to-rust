use crate::services::upload;

pub fn control_command(matches: &clap::ArgMatches) {
    match matches.subcommand() {
        Some(("upload", sub_matches)) => {
            let local_file_path = sub_matches.get_one::<String>("LOCAL_FILE_PATH").unwrap();
            let remote_folder_name = sub_matches.get_one::<String>("REMOTE_FOLDER_NAME").unwrap();
            
            upload::upload(&local_file_path, &remote_folder_name);
        }
        _ => unreachable!(),
    }
}
