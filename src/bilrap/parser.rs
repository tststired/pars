use crate::MErrors::{self};
use std::os::unix::process::ExitStatusExt;
use pars_libs::{parse_line, Remote, RemoteCommand};
use std::collections::HashMap;
use std::process::{Command, Output};
use std::sync::{Arc, Mutex};
use tokio::io::AsyncBufReadExt;
use tokio::sync;
use tokio::task;


fn proc_line(
    buffer: String,
    args: Arc<HashMap<String, String>>,
    guard: Arc<Mutex<bool>>,
    semaphore: Arc<sync::Semaphore>,
) -> Result<(), MErrors> {
    let instru_vec = match parse_line(&buffer) {
        Some(_) => parse_line(&buffer).unwrap(),
        None => return Err(MErrors::UTF8Error),
    };

    let halt_state = args.get("-e").unwrap();
    let mut placeholder_arg: String;
    let mut buffer = String::new();

    for i in instru_vec {
        placeholder_arg = "".to_string();
        if i.len() > 1 {
            placeholder_arg = i[1..].join(" ");
        };

        let output = Command::new(&i[0])
            .arg(&placeholder_arg)
            .output()
            .unwrap_or_else(|_| Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            });    

        //let x = Command::new(&i[0]).remote_spawn(&Remote::Local, &placeholder_arg);
        // i dont have time to debug this >_> but basically i need to branch this off for new spawn
        // shouldnt be here in the first place if remote

        

        let out = String::from_utf8_lossy(&output.stdout);
        let x = *guard.lock().unwrap();
        match x {
            true => {
                match output.status.code() {
                    Some(0) => (),
                    _ => match halt_state.as_str() {
                        "lazy" => semaphore.close(),
                        "eager" => {
                            semaphore.close();
                            {
                                *guard.lock().unwrap() = false;
                            }
                            break;
                        }
                        "never" => break,
                        _ => unreachable!(),
                    },
                }
                buffer.push_str(&out);
            }
            false => (),
        }
    }
    print!("{}", buffer);
    Ok(())
}

pub async fn read_lines_async(args: HashMap<String, String>, sbool: bool) -> Result<(), MErrors> {
    let workers = args.get("-J").unwrap().parse::<usize>()?;
    let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
    let guard = Arc::new(Mutex::new(true));
    let args = Arc::new(args);
    let (ttx, mut trx) = sync::mpsc::channel::<String>(workers * 10);
    let mut set: task::JoinSet<Result<(), MErrors>> = task::JoinSet::new();
    let semaphore = Arc::new(sync::Semaphore::new(workers));

    let send_dispatch: task::JoinHandle<Result<(), MErrors>> = task::spawn(async move {
        loop {
            let mut buffer = String::new();
            match reader.read_line(&mut buffer).await {
                Err(err) => return Err(MErrors::ParseError(err.to_string())),
                Ok(0) => {
                    return Ok(());
                }
                Ok(_) => {
                    ttx.send(buffer).await?;
                }
            }
        }
    });

    while let Some(line) = trx.recv().await {
        let guard = Arc::clone(&guard);
        let args = Arc::clone(&args);
        let semaphore = Arc::clone(&semaphore);
        let permit = semaphore.clone().acquire_owned().await;
        if permit.is_ok() && !semaphore.is_closed() {
            if !sbool {
                set.spawn(async move {
                    let x = proc_line(line, args, guard, semaphore);
                    drop(permit);
                    x
                });
            } else {

            }
        }
    }

    let _ = send_dispatch.await?;
    while let Some(i) = set.join_next().await {
        let _ = i?;
    }

    Ok(())
}
