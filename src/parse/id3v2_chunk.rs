use binrw::{io::TakeSeekExt, BinRead};
use std::fmt::Debug;

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
        parse_with = binrw::helpers::until_eof,
        // if id3v2_size is odd, throw away a byte
        align_after = 2,
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
