use binrw::{io::Cursor, BinReaderExt};
use std::{env, fs::File, io::Read, path::Path};

mod parse_chunk;
mod print_chunk;

use parse_chunk::{FmtChunk, ListInfoChunk, RiffChunk};
use print_chunk::{print_fmt_chunk, print_list_chunk, print_riff_chunk};

use crate::{parse_chunk::DataChunk, print_chunk::print_data_chunk};

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

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer);

    let riff_chunk: RiffChunk = cursor.read_le().unwrap();
    print_riff_chunk(&riff_chunk);
    println!("parsed {} bytes", cursor.position());

    let fmt_chunk: FmtChunk = cursor.read_le().unwrap();
    print_fmt_chunk(&fmt_chunk);
    println!("parsed {} bytes", cursor.position());

    let list_chunk: ListInfoChunk = cursor.read_le().unwrap();
    print_list_chunk(&list_chunk);
    println!("parsed {} bytes", cursor.position());

    let data_chunk: DataChunk = cursor.read_le().unwrap();
    print_data_chunk(&data_chunk);
    println!("parsed {} bytes", cursor.position());

    // riff_chunk.file_size excludes the RIFF header and the u32 describing the size (4 + 4 bytes)
    assert!(riff_chunk.file_size == cursor.position() as u32 - 8);
}
