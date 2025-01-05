use std::{path::Path, time::Instant};

pub fn get_input() -> String {
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();

    let filename = format!("{}.txt", pkg_name);

    let path = Path::new("inputs").join(filename);

    std::fs::read_to_string(&path)
        .map(|content| content.trim().to_string())
        .unwrap_or_else(|_| panic!("Error reading file at path {:?}", path))
}

pub fn get_input_dbg() -> String {
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let pkg_path_name = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let filename = format!("{}_dbg.txt", pkg_name);

    let path = Path::new(&pkg_path_name)
        .parent()
        .unwrap()
        .join("inputs")
        .join(filename);

    std::fs::read_to_string(&path)
        .map(|content| content.trim().to_string())
        .unwrap_or_else(|_| panic!("Error reading file at path {:?}", path))
}

pub fn submit_p1<F>(f: F)
where
    F: Fn(&str) -> i64,
{
    submit(f, 1);
}

pub fn submit_p2<F>(f: F)
where
    F: Fn(&str) -> i64,
{
    submit(f, 2);
}

fn submit<F>(f: F, part: u8)
where
    F: Fn(&str) -> i64,
{
    if cfg!(debug_assertions) {
        println!("Warning: code is running in debug mode.\n");
    }

    let input = get_input();

    let start = Instant::now();
    let result = f(&input);
    let end = start.elapsed().as_secs_f32() * 1000.0;

    println!("Result for part {part}: {result} (🚀 {end:.2} ms)");
}
