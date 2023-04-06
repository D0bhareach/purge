use std::{
    env,
    fs::File,
    io::{Error as IOError, BufReader, BufWriter, BufRead, Write},
};
use regex::{self, Regex};
use anyhow::anyhow;
// use thiserror;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("transparent")]
    RegErr(#[from]regex::Error),
    #[error("transparent")]
    IOErr(#[from] IOError),
    #[error("{0}")]
    Any(#[from] anyhow::Error), 
}
struct State {
    process_next: bool,
}

fn main() -> Result<(), AppError> {
    let ignore = vec!["// FIX", "// TODO:"];
    let reg_comment = Regex::new(r"s*//s*")?;
    let mut state = State {
        process_next: false,
    };
    let mut a = env::args();
    if a.len() < 2 {
        return Err(AppError::Any(anyhow!("Must provide name of file to work.")));
    }
    // checked it already
    let fname = a.nth(1).unwrap();
    let file: File = File::open(fname)?;

    let reader = BufReader::new(&file);
    let mut writer = BufWriter::new(File::create("tmp")?);
   #[allow(clippy::if_same_then_else)]
    'main_loop: for line in reader.lines() {
       let line = line.as_ref().unwrap();
       if reg_comment.is_match(line) {
        if state.process_next {
                    continue 'main_loop;
            }
            else {
                for ign in ignore.iter() {
                    if line.contains(ign) {
                        state.process_next = true;
                        continue 'main_loop;
                    }        
                }
            }  

       }
        else {
            state.process_next = false;
        }

        writeln!(writer, "{}", line)?;
   }
    Ok(())
}
