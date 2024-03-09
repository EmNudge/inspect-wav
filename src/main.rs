use binrw::{io::Cursor, BinReaderExt};
use std::{env, fs::File, io::Read, path::Path};

mod parse;
mod print_chunk;
mod print_utils;

use parse::{DataChunk, FmtChunk, ID3v2Chunk, ListInfoChunk, RiffChunk};
use print_chunk::{
    print_data_chunk, print_fmt_chunk, print_id3_chunk, print_list_chunk, print_riff_chunk,
};
use print_utils::print_position;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("");

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

    let mut cursor = Cursor::new(&buffer);

    let riff_chunk: RiffChunk = cursor.read_le().unwrap();
    print_riff_chunk(&riff_chunk);
    print_position(&cursor);

    // riff_chunk.file_size excludes the RIFF header and the u32 describing the size (4 + 4 bytes)
    while cursor.position() as u32 - 8 < riff_chunk.file_size {
        if let Ok(fmt_chunk) = cursor.read_le::<FmtChunk>() {
            print_fmt_chunk(&fmt_chunk);
        } else if let Ok(data_chunk) = cursor.read_le::<DataChunk>() {
            print_data_chunk(&data_chunk);
        } else if let Ok(list_chunk) = cursor.read_le::<ListInfoChunk>() {
            print_list_chunk(&list_chunk);
        } else if let Ok(id3_chunk) = cursor.read_le::<ID3v2Chunk>() {
            print_id3_chunk(&id3_chunk);
        } else {
            let mut word_buff = [0u8; 4];
            cursor.read_exact(&mut word_buff).unwrap();
            println!(
                "ERR: unknown chunk: {:?}",
                String::from_utf8(word_buff.to_vec()).unwrap()
            );
            return;
        }
        print_position(&cursor);
    }

    assert!(cursor.position() as usize == buffer.len());
    println!("\nFinished parsing!");
}
