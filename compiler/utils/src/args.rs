use std::env;
use std::io::{Error, ErrorKind};

pub fn get_args() -> Result<Vec<String>, Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::NotFound,
            "do not have enough arguments",
        ));
    }

    Ok(args[1..].to_owned())
}
