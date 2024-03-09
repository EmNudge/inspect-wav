use binrw::BinRead;
use std::fmt::Debug;

#[derive(Debug, BinRead)]
pub struct UnknownChunk {
    #[br(map = |s: [u8; 4]| String::from_utf8(s.to_vec()).unwrap())]
    pub chunk_id: String,

    pub chunk_size: u32,

    #[br(count = chunk_size)]
    pub data: Vec<u8>,
}
