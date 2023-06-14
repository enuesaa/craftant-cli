use std::process::Command;
use crate::service::registry::Registry;

/**
 * see https://keens.github.io/blog/2016/12/02/rustnopurosesu/
 * see https://www.howtosolutions.net/2021/10/rust-creating-simple-terminal-interactive-input-prompt-app/
 * see https://stackoverflow.com/questions/72750736/run-command-stream-stdout-stderr-and-capture-results
 * see https://stackoverflow.com/questions/66060139/how-to-tee-stdout-stderr-from-a-subprocess-in-rust
 */
pub fn run_handler(name: &str) {
    let registry = Registry::new();
    if let Ok(commanddef) = registry.get_command(name) {
        println!("Run following command..");
        println!("  {}", commanddef.command);
        println!("");
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(commanddef.command)
            .spawn()
            .unwrap();

        let status = child.wait().unwrap();
        println!("");
        println!("Command completed with status {}", status.code().unwrap());
    } else {
        println!("");
        println!("Command not found.");
    }
}
