use std::{collections::BTreeMap, fmt::Debug};
use binrw::{binrw, BinRead};
use lazy_static::lazy_static;

const COMPRESSION_CODES_STR: &'static str = include_str!("compression_codes.json");

lazy_static! {
    static ref COMPRESSION_CODES_MAP: BTreeMap<u16, String> =
        serde_json::from_str::<Vec<(u16, String)>>(COMPRESSION_CODES_STR)
            .unwrap()
            .into_iter()
            .collect();
}

pub fn get_compression_code_str(compression_code: u16) -> String {
    COMPRESSION_CODES_MAP
        .get(&compression_code)
        .map_or("UNKNOWN".to_string(), |s| s.to_owned())
}

#[derive(Debug, BinRead)]
#[br(magic = b"RIFF")]
pub struct RiffChunk {
    pub file_size: u32,

    #[br(map = |s: [u8; 4]| String::from_utf8(s.to_vec()).unwrap())]
    pub wave_ident: String,
}

#[derive(Debug, BinRead)]
#[br(magic = b"fmt ")]
pub struct FmtChunk {
    pub chunk_size: u32,
    pub compression_code: u16,
    pub number_of_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,

    #[br(if(chunk_size == 18 || chunk_size == 40))]
    _extra_bytes: Option<u16>,

    #[br(if(chunk_size == 40))]
    pub extended_fmt_sub_chunk: Option<ExtendedFmtSubChunk>,
}

// If the FmtChunk size is 40, this is the rest of it.
#[derive(Debug, BinRead)]
pub struct ExtendedFmtSubChunk {
    pub num_valid_bits: u16,
    pub channel_mask: u32,
    // first 2 bytes are compression code, next 14 are GUID "\x00\x00\x00\x00\x10\x00\x80\x00\x00\xAA\x00\x38\x9B\x71"
    pub compression_code: u16,

    pub wav_guid: [u8; 14],
}

// If the compression_code is not PCM, there is a fact chunk
#[binrw]
#[br(magic = b"fact")]
#[derive(Debug)]
pub struct FactChunk {
    chunk_size: u32,
    sample_length: u32,
}

// A LIST chunk may be present
#[binrw]
#[br(magic = b"LIST")]
#[derive(Debug)]
pub struct ListInfoChunk {
    pub chunk_size: u32,
    
    // how do we do count with chunk_size on structured info
    #[br(count = chunk_size, magic = b"INFO")]
    pub data: Vec<u8>,
}
// A LIST chunk may contain many INFO subchunks.
#[derive(BinRead)]
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

#[binrw]
#[br(magic = b"data")]
#[derive(Debug)]
pub struct DataChunk {
    chunk_size: u32,
    #[br(count = chunk_size)]
    sample_data: Vec<u8>,
}
