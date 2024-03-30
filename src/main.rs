use std::env;
use std::process;
use std::path::{Path,PathBuf};
use std::str::FromStr;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("expecting jit info <dir>");
        process::exit(1);
    }
    let cmd: &String = &args[1];
    if cmd != "init" {
        eprintln!("expected init command");
        process::exit(1);
    }

    if args.len() > 3 {
        eprintln!("expected init jit <dir>");
        process::exit(1);
    }

    let path_buf_res: Result<PathBuf, std::convert::Infallible> = PathBuf::from_str(&args[2]);

    match path_buf_res {
        Ok(path_buf) => {
            let path = path_buf.as_path();
            if path.is_absolute() {
                fs::create_dir_all(&path)?;
                println!("made a new directory!");
                Ok(())
            } else {
                let cwd_path_res = env::current_dir()?;
                let cwd_path = cwd_path_res.as_path();
                let new_dir = cwd_path.join(&path_buf);
                fs::create_dir_all(&new_dir)?;
                println!("made a new relative directory!");
                Ok(())
            }
        }
        Err(_) => {
            eprintln!("malformed path");
            Ok(())
        }
    }

    
}

