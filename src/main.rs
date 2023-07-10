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
    let (prefix, rest) = num_str.split_at(2);
    match prefix {
        "0x" => usize::from_str_radix(rest,16),
        "0b" => usize::from_str_radix(rest,2),
        "0o" => usize::from_str_radix(rest, 8),
        _ => num_str.parse(),
    }.map_err(|_| "Error parsing input number")
}

fn convert(num: usize, out_base: String) -> Result<String,&'static str> {
    let a = &*out_base;
    match a {
        "d" => Ok(format!("{}", num)),
        "b" => Ok(format!("{:b}", num)),
        "o" => Ok(format!("{:o}", num)),
        "x" => Ok(format!("{:x}", num)),
        "X" => Ok(format!("{:X}", num)),
        _ => Err("Invalid output base")
    }
}
