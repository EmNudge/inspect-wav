// If the compression_code is *not* PCM, there is a fact chunk
#[binrw]
#[br(magic = b"fact")]
#[derive(Debug)]
pub struct FactChunk {
    chunk_size: u32,
    sample_length: u32,
}