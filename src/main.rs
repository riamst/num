use std::env;
use std::process;

struct Config {
    num: String,
    out_base: String,
}

fn main() {
    let config = build(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    args.next();
    let num = args.next().ok_or("Didn't get a number")?;
    let out_base = args.next().unwrap_or("d".to_string());

    Ok(Config{num, out_base})
}

fn run(config: Config) -> Result<(), &'static str> {
    let num = parse_num(&config.num)?;
    let output = convert(num, config.out_base)?;

    println!("{}", output);
    Ok(())
}

fn parse_num(num_str: &str) -> Result<usize,&'static str> {
    todo!()
}

fn convert(num: usize, out_base: String) -> Result<String,&'static str> {
    todo!()
}
