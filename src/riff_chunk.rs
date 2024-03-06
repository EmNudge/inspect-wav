use binrw::{binrw, io::Cursor, BinReaderExt};
use comfy_table::Row;
use std::str::from_utf8;

#[binrw]
#[br(magic = b"RIFF")]
struct RiffChunk {
    file_size: u32,
    wave_ident: [u8; 4],
}

pub fn parse_riff_chunk(buffer: &[u8]) -> Result<(Vec<Row>, usize), String> {
    let mut rows = vec![];

    let mut cursor = Cursor::new(buffer);

    let riff_chunk: RiffChunk = cursor.read_le().unwrap();

    rows.push(Row::from(vec!["chunk id", "'RIFF'"]));

    rows.push(Row::from(vec![
        "size of file (in bytes)",
        &riff_chunk.file_size.to_string(),
    ]));

    rows.push(Row::from(vec![
        "wave identifier",
        from_utf8(&riff_chunk.wave_ident).expect("WAVE chunk is not valid UTF-8"),
    ]));

    Ok((rows, cursor.position() as usize))
}
