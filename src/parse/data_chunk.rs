use binrw::BinRead;
use std::fmt::Debug;

#[derive(Debug, BinRead)]
#[br(magic = b"data")]
pub struct DataChunk {
    pub chunk_size: u32,
    #[br(count = chunk_size)]
    pub sample_data: Vec<u8>,
}
