/// Hello in the last assign you asked why I called "pub mod gvs;" https://imgur.com/a/qp01Xpn
/// It's reverse of svg and I couldn't think of a folder name That's why parlib is bilrap
pub mod bilrap;
use bilrap::{errors::MErrors, parser::read_lines_async};
use std::collections::HashMap;
use std::env;


fn check_args() -> Result<HashMap<String, String>, MErrors> {
    let args = env::args().collect::<Vec<String>>();
    let mut iter = args.iter().peekable();
    let mut hmap: HashMap<String, String> = HashMap::new();

    if iter.next().is_none() {
        return Err(MErrors::UsageError);
    } // no args after running

    while let Some(i) = iter.next() {
        match i.as_str() {
            "-J" => match iter.next() {
                Some(val) => {
                    let val = val.parse::<i32>()?;
                    hmap.insert(i.to_string(), val.to_string());
                }
                None => return Err(MErrors::UsageError),
            },
            "-e" | "--halt" => match iter.next() {
                Some(val) => {
                    match val.as_str() {
                        "lazy" | "eager" | "never" => {
                            hmap.insert("-e".to_string(), val.to_string())
                        }
                        _ => return Err(MErrors::UsageError),
                    };
                    hmap.insert(i.to_string(), val.to_string());
                }
                None => return Err(MErrors::UsageError),
            },
            "-r" => match iter.next() {
                Some(val) => {
                    hmap.insert(i.to_string(), val.to_string());
                    hmap.insert("--server".to_string(), "true".to_string());
                    if !hmap.contains_key("-J") { hmap.insert("-J".to_string(), "1".to_string()); };
                }
                None => return Err(MErrors::UsageError),
            },
            _ => return Err(MErrors::UsageError),
        }
    }

    if !hmap.contains_key("-J") && !hmap.contains_key("-r") { return Err(MErrors::UsageError); };
    if !hmap.contains_key("--server") { hmap.insert("--server".to_string(), "false".to_string()); };
    if !hmap.contains_key("-e") { hmap.insert("-e".to_string(), "never".to_string()); };
    Ok(hmap)
}


fn main() -> Result<(), MErrors> {
    let args = check_args()?;
    let workers = args.get("-J").unwrap().parse::<usize>()?;

    let body = async {
        if args.get("--server").unwrap() == "true" {
            //let launch = "cargo run -- -J 1";
            

        } else {
            read_lines_async(args, false).await?;
        }
        Ok(())
    };

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(workers)
        .max_blocking_threads(workers)
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}
