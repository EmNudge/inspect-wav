use std::collections::BTreeMap;
use binrw::binrw;
use lazy_static::lazy_static;

#[binrw]
#[br(magic = b"RIFF")]
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
pub struct FmtChunk {
    pub chunk_size: u32,
    pub compression_code: u16,
    pub number_of_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
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
pub struct ExtendedFmtChunk {
    pub extra_fmt_bytes_num: u16,
    pub num_valid_bits: u16,
    pub channel_mask: u32,
    pub sub_format: [u8; 16],
}
impl ExtendedFmtChunk {
    pub fn get_compression_code_str(&self) -> String {
        let compression_code = u16::from_le_bytes(self.sub_format[0..2].try_into().unwrap());
        COMPRESSION_CODES_MAP
            .get(&compression_code)
            .map_or("UNKNOWN".to_string(), |s| s.to_owned())
    }

    pub fn get_guid(&self) -> String {
        self.sub_format
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join("")
    }
}

// If the compression_code is not PCM, there is a fact chunk
#[binrw]
#[br(magic = b"fact")]
pub struct FactChunk {
    chunk_size: u32,
    sample_length: u32,
}

#[binrw]
#[br(magic = b"data")]
pub struct DataChunk {
    chunk_size: u32,
    #[br(count = chunk_size)]
    sample_data: Vec<u8>,
}
