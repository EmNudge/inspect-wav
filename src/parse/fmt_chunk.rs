use binrw::BinRead;
use std::fmt::Debug;

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
    pub extra_bytes: Option<u16>, // should just be '22'

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
