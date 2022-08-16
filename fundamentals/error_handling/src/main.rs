use anyhow::Context;
use anyhow::{anyhow, Result};
use std::path::PathBuf;

// Without anyhow
// fn error_me(throw: bool) -> Result<(), usize> {
//     if throw {
//         Err(1)
//     } else {
//         Ok(())
//     }
// }

// fn main() -> Result<(), usize> {
//     let value = error_me(false)?;

//     if error_me(true).is_ok() {
//         println!("Ok");
//     } else {
//         println!("Err");
//     }
//     return Ok(());
// }

// with anyhow

fn error_me(throw: bool) -> Result<()> {
    if throw {
        return Err(anyhow!("Error"));
    }
    std::fs::read(PathBuf::from("/foo")).context("Hey! Unable to read file")?;

    Ok(())
}

fn main() -> Result<()> {
    let value = error_me(false)?;

    return Ok(());
}
