use std::{env, fs::File, io::Read, path::Path};
use binrw::{io::Cursor, BinReaderExt};
use byteorder::{LittleEndian, ReadBytesExt};

mod parse_chunk;
mod print_chunk;

use parse_chunk::{RiffChunk, FmtChunk, ExtendedFmtChunk, FactChunk, DataChunk};
use print_chunk::{ print_fmt_chunk, print_riff_chunk };

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

    let mut buffer = [0; 300];
    file.read(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer);
    
    print_riff_chunk(cursor.read_le::<RiffChunk>().unwrap());

    let fmt_chunk: FmtChunk = cursor.read_le().unwrap();
    if fmt_chunk.chunk_size > 16 {
        cursor.read_u16::<LittleEndian>().unwrap();
    } 

    if fmt_chunk.chunk_size == 40 {
        let ext_fmt_chunk: ExtendedFmtChunk = cursor.read_le().unwrap();
        print_fmt_chunk(fmt_chunk, Some(ext_fmt_chunk));
    } else {
        print_fmt_chunk(fmt_chunk, None);  
    }

    println!("parsed {} bytes", cursor.position());
}
