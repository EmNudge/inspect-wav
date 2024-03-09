use binrw::BinRead;
use std::fmt::Debug;

#[derive(Debug, BinRead)]
#[br(magic = b"RIFF")]
pub struct RiffChunk {
    pub file_size: u32,

    #[br(map = |s: [u8; 4]| String::from_utf8(s.to_vec()).unwrap())]
    pub wave_ident: String,
}
