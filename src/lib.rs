use std::io::Write;
use std::{fs, io};
pub type Result<T = (), E = Box<dyn std::error::Error + 'static>> = std::result::Result<T, E>;

pub struct Rlox;
impl Rlox {
    pub fn run_file(path: &str) -> Result {
        let source = fs::read_to_string(path)?;
        Self::run(source.trim())
    }
    pub fn run_prompt() -> Result {
        let mut line = String::new();
        loop {
            print!("> ");
            line.clear();
            io::stdout().flush()?;
            if let 0 = io::stdin().read_line(&mut line)? {
                break;
            }
            Self::run(line.trim())?;
        }
        Ok(())
    }
    fn run(source: &str) -> Result {
        println!("{source}");
        Ok(())
    }
}
