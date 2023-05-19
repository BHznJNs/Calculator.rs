use super::commands;

pub fn output() {
    println!("Usage: calculator [SCRIPT_PATH] [OPTIONS]\n");

    println!("Options:");
    for i in 0..commands::COMMAND_COUNT {
        println!("{}, {}", commands::COMMANDS[i][0], commands::COMMANDS[i][1]);
        println!("  {}", commands::COMMAND_DESCRIPTIONS[i]);
    }
}