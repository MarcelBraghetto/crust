pub mod core;
pub mod log_tag;
pub mod opengl;
pub mod scenes;

use crate::core::logs;

pub fn main() {
    std::process::exit(match core::launcher::launch() {
        Ok(_) => 0,
        Err(err) => {
            logs::out(log_tag!(), &format!("Fatal error: {:?}", err));
            1
        }
    });
}
