use std::{collections::BTreeMap, fmt::Debug};
use binrw::binrw;
use lazy_static::lazy_static;

#[binrw]
#[br(magic = b"RIFF")]
#[derive(Debug)]
pub struct RiffChunk {
    pub file_size: u32,
    wave_ident: [u8; 4],
}
impl RiffChunk {
    pub fn get_wave_ident(&self) -> String {
        String::from_utf8(self.wave_ident.to_vec()).unwrap()
    }
}

const COMPRESSION_CODES_STR: &'static str = include_str!("compression_codes.json");

lazy_static! {
    static ref COMPRESSION_CODES_MAP: BTreeMap<u16, String> =
        serde_json::from_str::<Vec<(u16, String)>>(COMPRESSION_CODES_STR)
            .unwrap()
            .into_iter()
            .collect();
}

#[binrw]
#[br(magic = b"fmt ")]
#[derive(Debug)]
pub struct FmtChunk {
    pub chunk_size: u32,
    pub compression_code: u16,
    pub number_of_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,

    #[br(if(chunk_size == 18 || chunk_size == 40))]
    extra_bytes: Option<u16>,

    #[br(if(chunk_size == 40))]
    pub extended_fmt_sub_chunk: Option<ExtendedFmtSubChunk>,
}
impl FmtChunk {
    pub fn get_compression_code_str(&self) -> String {
        COMPRESSION_CODES_MAP
            .get(&self.compression_code)
            .map_or("UNKNOWN".to_string(), |s| s.to_owned())
    }
}

// If the FmtChunk size is 40, this is the rest of it.
#[binrw]
#[derive(Debug)]
pub struct ExtendedFmtSubChunk {
    pub num_valid_bits: u16,
    pub channel_mask: u32,
    // first 2 bytes are compression code, next 14 are GUID "\x00\x00\x00\x00\x10\x00\x80\x00\x00\xAA\x00\x38\x9B\x71"
    pub compression_code: u16,
    pub wave_guid: [u8; 14],
}

impl ExtendedFmtSubChunk {
    pub fn get_compression_code_str(&self) -> String {
        COMPRESSION_CODES_MAP
            .get(&self.compression_code)
            .map_or("UNKNOWN".to_string(), |s| s.to_owned())
    }

    pub fn get_guid(&self) -> String {
        self.wave_guid
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join(" ")
    }
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
#[binrw]
pub struct ListInfoSubChunk {
    pub info_id: [u8; 4],
    pub chunk_size: u32,

    // round up evenly (might be able to use align_after instead)
    #[br(count = (chunk_size + 1) & !1)]
    pub data: Vec<u8>,
}
impl ListInfoSubChunk {
    pub fn get_info_id(&self) -> String {
        String::from_utf8(self.info_id.to_vec()).unwrap()
    }
    pub fn get_text(&self) -> String {
        String::from_utf8(self.data.clone()).unwrap()
    }
}

impl Debug for ListInfoSubChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info_id = String::from_utf8(self.info_id.to_vec()).unwrap();
        let data = String::from_utf8(self.data.to_vec()).unwrap();
        write!(f, "ListInfoSubChunk {{ info_id: {}, chunk_size: {}, data: {} }}", info_id, self.chunk_size, data)
    }
}

#[binrw]
#[br(magic = b"data")]
#[derive(Debug)]
pub struct DataChunk {
    chunk_size: u32,
    #[br(count = chunk_size)]
    sample_data: Vec<u8>,
}
