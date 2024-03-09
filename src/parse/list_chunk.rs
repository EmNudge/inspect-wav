use binrw::{ BinRead, io::TakeSeekExt};
use std::fmt::Debug;

// A LIST chunk may be present
#[derive(BinRead, Debug)]
#[br(magic = b"LIST")]
pub struct ListInfoChunk {
    pub chunk_size: u32,
    
    #[br(
        magic = b"INFO", 
        // use map_stream to make a new stream of only size chunk_size
        map_stream = |s| s.take_seek(chunk_size.into()),
        // read until end of input (the rest of chunk_size buffer)
        parse_with = binrw::helpers::until_eof
    )]
    pub data: Vec<ListInfoSubChunk>,
}
#[derive(BinRead, Debug)]
pub struct ListInfoSubChunk {
    #[br(map = |x: [u8; 4]| String::from_utf8(x.to_vec()).unwrap())]
    pub info_id: String,
    pub chunk_size: u32,

    #[br(
        count = chunk_size, 
        // parse even amount of bytes.
        align_after = 2, 
        map = |x: Vec<u8>| String::from_utf8(x).unwrap()
    )]
    pub text: String,
}