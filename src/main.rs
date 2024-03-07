use binrw::{io::Cursor, BinReaderExt};
use std::{env, fs::File, io::{BufRead, Read}, path::Path};

mod parse_chunk;
mod print_chunk;

use parse_chunk::{FmtChunk, RiffChunk};
use print_chunk::{print_fmt_chunk, print_riff_chunk};

use crate::{parse_chunk::{ListInfoChunk, ListInfoSubChunk}, print_chunk::print_list_chunk};

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

    let mut buffer = [0; 600];
    file.read(&mut buffer).unwrap();

    let mut cursor = Cursor::new(buffer);

    print_riff_chunk(&cursor.read_le::<RiffChunk>().unwrap());
    println!("parsed {} bytes", cursor.position());

    let fmt_chunk: FmtChunk = cursor.read_le().unwrap();
    print_fmt_chunk(&fmt_chunk);
    
    println!("parsed {} bytes", cursor.position());

    let list_chunk: ListInfoChunk = cursor.read_le().unwrap();
    let mut list_info_sub_chunks = vec![];
    {
        let mut cursor = Cursor::new(list_chunk.data.clone());
        while cursor.position() < list_chunk.chunk_size as u64 - 4 {
            let sub_chunk: ListInfoSubChunk = cursor.read_le().unwrap();
            // increment if odd-number of bytes was parsed
            if cursor.position() % 2 == 1 {
                cursor.consume(1);
            }
            list_info_sub_chunks.push(sub_chunk);
        }
    }
    print_list_chunk(&list_chunk, &list_info_sub_chunks);

    println!("parsed {} bytes", cursor.position());
}
