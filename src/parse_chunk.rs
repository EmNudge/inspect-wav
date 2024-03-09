use std::{collections::BTreeMap, fmt::Debug};
use binrw::{binrw, BinRead, io::TakeSeekExt};
use lazy_static::lazy_static;

lazy_static! {
    static ref COMPRESSION_CODES_MAP: BTreeMap<u16, String> =
        serde_json::from_str::<Vec<(u16, String)>>(include_str!("compression_codes.json"))
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

// If the compression_code is *not* PCM, there is a fact chunk
#[binrw]
#[br(magic = b"fact")]
#[derive(Debug)]
pub struct FactChunk {
    chunk_size: u32,
    sample_length: u32,
}

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

#[derive(Debug, BinRead)]
#[br(magic = b"data")]
pub struct DataChunk {
    pub chunk_size: u32,
    #[br(count = chunk_size)]
    pub sample_data: Vec<u8>,
}

// ID3v2 is usually in mp3 files. It can appear in WAV files.
// https://mutagen-specs.readthedocs.io/en/latest/_sources/id3/id3v2.3.0.txt
#[derive(Debug, BinRead)]
#[br(magic = b"id3 ")]
pub struct ID3v2Chunk {
    pub chunk_size: u32,

    // We see repetition since we're now in a WAV 'id3 ' chunk and we will now see the ID3v2 data
    // which begins by using the magic "ID3" identifier
    #[br(magic = b"ID3")]
    
    // usually 3 (i.e. ID3v2.3.0 - confusing, I know)
    pub major_version: u8,
    // usually 0
    pub minor_version: u8,
    
    // bit flags - only bits 7, 6, and 5 are used
    // [_, _, _, _, footer?, experimental, xheader, unsync]
    pub flags: u8,

    // now the format defines its own size using big-endian
    #[br(big)]
    pub id3v2_size: u32,

    // if the xheader flag is there, the length of it comes next and the header info
    #[br(if((flags & 0b1000_0000) != 0))]
    pub xheader: Option<ID3v2XHeaderSubChunk>,

    #[br(
        map_stream = |s| s.take_seek(id3v2_size.into()),
        parse_with = binrw::helpers::until_eof
    )]
    pub tags: Vec<ID3v2TagSubChunk>,
}
#[derive(Debug, BinRead)]
#[br(big)]
pub struct ID3v2XHeaderSubChunk {
    pub chunk_size: u32,

    // only 1 bit is used (waste of space, honestly)
    // it defines if 4 bytes of CRC data is appended to the xheader
    pub extended_flags: u16,
    
    // no clue
    pub padding_size: u32,

    #[br(count = chunk_size)]
    pub data: Vec<u8>,
}
#[derive(Debug, BinRead)]
#[br(big)]
pub struct ID3v2TagSubChunk {
    #[br(map = |s: [u8; 4]| String::from_utf8(s.to_vec()).unwrap())]
    pub frame_id: String,

    pub frame_size: u32,
    pub flags: u16,

    #[br(
        count = frame_size,
        map = |s: Vec<u8>| String::from_utf8(s.to_vec()).unwrap(),
    )]
    pub data: String,
}