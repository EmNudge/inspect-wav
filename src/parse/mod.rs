mod riff_chunk;
pub use riff_chunk::RiffChunk;

mod data_chunk;
pub use data_chunk::DataChunk;

mod fmt_chunk;
pub use fmt_chunk::FmtChunk;

mod id3v2_chunk;
pub use id3v2_chunk::ID3v2Chunk;

mod list_chunk;
pub use list_chunk::ListInfoChunk;

mod unknown_chunk;
pub use unknown_chunk::UnknownChunk;

use lazy_static::lazy_static;

use std::collections::BTreeMap;
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
