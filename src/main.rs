use binrw::{io::Cursor, BinReaderExt};
use color_eyre::eyre::{eyre, Result};
use std::{fs::File, io::Read};

mod args;
mod parse;
mod print_chunk;
mod print_utils;

use parse::{DataChunk, FmtChunk, ID3v2Chunk, ListInfoChunk, RiffChunk, UnknownChunk};
use print_utils::print_position;

fn main() -> Result<()> {
    let args = args::get_args()?;
    let mut file = File::open(&args.file).unwrap();

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();

    let mut cursor = Cursor::new(&buffer);

    let riff_chunk: RiffChunk = cursor.read_le().unwrap();
    println!("{}", riff_chunk);
    print_position(&cursor);

    // riff_chunk.file_size excludes the RIFF header and the u32 describing the size (4 + 4 bytes)
    while cursor.position() as u32 - 8 < riff_chunk.file_size {
        if let Ok(fmt_chunk) = cursor.read_le::<FmtChunk>() {
            println!("{}", fmt_chunk);
        } else if let Ok(data_chunk) = cursor.read_le::<DataChunk>() {
            println!("{}", data_chunk);
        } else if let Ok(list_chunk) = cursor.read_le::<ListInfoChunk>() {
            println!("{}", list_chunk);
        } else if let Ok(id3_chunk) = cursor.read_le::<ID3v2Chunk>() {
            println!("{}", id3_chunk);
        } else if let Ok(unknown_chunk) = cursor.read_le::<UnknownChunk>() {
            println!("{}", unknown_chunk);
        } else {
            let mut word_buff = [0u8; 4];
            cursor.read_exact(&mut word_buff).unwrap();
            return Err(eyre!(
                "Unknown chunk: {:?}",
                String::from_utf8(word_buff.to_vec()).unwrap()
            ));
        }
        print_position(&cursor);
    }

    assert!(cursor.position() as usize == buffer.len());
    println!("\nFinished parsing!");

    Ok(())
}
