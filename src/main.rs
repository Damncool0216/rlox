use rlox::Result;
use rlox::Rlox;
fn main() -> Result {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => Rlox::run_prompt()?,
        2 => Rlox::run_file(&args[1])?,
        _ => {
            println!("Usages: {} [script]", args[0]);
            std::process::exit(64);
        }
    }
    Ok(())
}
