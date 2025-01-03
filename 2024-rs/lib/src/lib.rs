use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub part: u8,

    #[arg(short, long, default_value_t = false)]
    pub dbg: bool,
}

pub fn get_input(args: &Args) -> String {
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();

    let filename = if args.dbg {
        format!("{}_dbg.txt", pkg_name)
    } else {
        format!("{}.txt", pkg_name)
    };

    let path = Path::new("inputs").join(filename);

    std::fs::read_to_string(&path)
        .map(|content| content.trim().to_string())
        .unwrap_or_else(|_| panic!("Error reading file at path {:?}", path))
}
