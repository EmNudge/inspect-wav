use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Table};
use std::{env, fs::File, io::Read, path::Path};

mod fmt_chunk;
mod riff_chunk;
fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(path) = args.get(1) else {
        println!("Usage: inspect_wav <path to wav file>");
        return;
    };

    if !Path::new(path).exists() {
        println!("ERR: file does not exist");
        return;
    }

    let mut file = File::open(path).unwrap();

    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);

    let mut buffer = [0; 300];
    file.read(&mut buffer).unwrap();

    let (riff_rows, cursor_position) = riff_chunk::parse_riff_chunk(&buffer).unwrap();

    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.add_rows(riff_rows);
    println!("{table}");

    let (fmt_rows, cursor_position) =
        fmt_chunk::parse_fmt_chunk(&buffer[(cursor_position as usize)..]).unwrap();

    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.add_rows(fmt_rows);
    println!("{table}");

    println!("parsed {cursor_position} bytes");
}
