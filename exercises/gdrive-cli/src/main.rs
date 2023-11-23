mod controller;
mod commands;
mod cli;
mod services;

fn main() {
    let matches = cli::cli().get_matches();
    
    controller::control_command(&matches);
}
