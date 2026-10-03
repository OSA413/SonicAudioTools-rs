mod dump;
mod help;
mod version;

use std::{env, fs};

use common_binary::error::CommonBinaryError;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut csb_path: Option<String> = None;
    let mut cpk_path: Option<String> = None;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--help" | "-h" => {
                help::print();
                return;
            }
            "--version" | "-v" => {
                version::print();
                return;
            }
            "--cpk" => {
                let flag = args[index].clone();
                index += 1;
                if index < args.len() {
                    let value = args[index].clone();
                    cpk_path = Some(value);
                } else {
                    eprintln!("Missing value for {flag}");
                    help::print();
                    return;
                }
            }
            arg if arg.starts_with('-') => {
                eprintln!("Unknown option: {arg}");
                help::print();
                return;
            }
            _ => {
                if csb_path.is_none() {
                    csb_path = Some(args[index].clone());
                } else {
                    eprintln!("Unexpected extra argument: {}", args[index]);
                    help::print();
                    return;
                }
            }
        }
        index += 1;
    }

    convert(&csb_path.unwrap(), cpk_path.as_deref()).unwrap()
}

fn convert(
    csb_path: &str,
    cpk_path: Option<&str>,
) -> Result<(), CommonBinaryError> {
    let csb_source = fs::read(csb_path)?;
    let cpk_source = match cpk_path {
        Some(path) => Some(fs::read(path)?),
        None => None,
    };

    let result = dump::dump(&csb_source, cpk_source.as_deref())?;

    let json = serde_json::to_string_pretty(&result)?;
    println!("{json}");
    
    Ok(())
}