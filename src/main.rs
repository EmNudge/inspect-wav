
use comfy_table::Table;
use std::{env, fs::File, io::Read, path::Path};

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
    table.set_header(vec!["Property", "Value"]);

    let mut buffer = [0; 40];
    file.read(&mut buffer).unwrap();

    // RIFF chunk
    if b"RIFF" == &buffer[..4] {
        table.add_row(vec!["chunk id", "'RIFF'"]);
    } else {
        println!("ERR: does not have 'RIFF' header");
        return;
    }

    let add_row_num_buf = |table: &mut Table, name: &str, buffer: &[u8]| {
        table.add_row(vec![
            name,
            &(if buffer.len() == 2 {
                u16::from_le_bytes(buffer.try_into().unwrap()).to_string()
            } else {
                u32::from_le_bytes(buffer.try_into().unwrap()).to_string()
            }),
        ]);
    };

    add_row_num_buf(&mut table, "size of file (in bytes)", &buffer[4..8]);

    if b"WAVE" == &buffer[8..12] {
        table.add_row(vec!["wave identifier", "WAVE"]);
    } else {
        println!("ERR: does not have 'WAVE' identifier");
        return;
    }

    // fmt chunk
    if b"fmt " == &buffer[12..16] {
        table.add_row(vec!["chunk id", "'fmt '"]);
    } else {
        println!("ERR: does not have 'fmt ' chunk");
        return;
    }

    add_row_num_buf(&mut table, "size of fmt chunk (in bytes)", &buffer[16..20]);
    add_row_num_buf(&mut table, "size of fmt chunk (in bytes)", &buffer[16..20]);
    add_row_num_buf(&mut table, "compression code", &buffer[20..22]);
    add_row_num_buf(&mut table, "number of channels", &buffer[22..24]);
    add_row_num_buf(&mut table, "sampling rate", &buffer[24..28]);
    add_row_num_buf(&mut table, "byte rate", &buffer[28..32]);
    add_row_num_buf(&mut table, "block align", &buffer[32..34]);
    add_row_num_buf(&mut table, "bits per sample", &buffer[34..36]);

    println!("{table}");
}