use binrw::{binrw, io::Cursor, BinReaderExt};
use byteorder::{LittleEndian, ReadBytesExt};
use comfy_table::Row;
use std::collections::BTreeMap;

const COMPRESSION_CODES_STR: &'static str = include_str!("compression_codes.json");

#[binrw]
#[br(magic = b"fmt ")]
struct FmtChunk {
    chunk_size: u32,
    compression_code: u16,
    number_of_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
}
// If the FmtChunk size is 40, this is the rest of it.
#[binrw]
struct ExtendedFmtChunk {
    extra_fmt_bytes_num: u16,
    num_valid_bits: u16,
    channel_mask: u32,
    sub_format: [u8; 16],
}

pub fn parse_fmt_chunk(buffer: &[u8]) -> Result<(Vec<Row>, usize), String> {
    let compression_codes_map = {
        let compression_codes: Vec<(u16, String)> =
            serde_json::from_str(COMPRESSION_CODES_STR).unwrap();
        compression_codes
            .into_iter()
            .collect::<BTreeMap<u16, String>>()
    };
    let get_compression_code_str = move |compression_code: u16| -> String {
        compression_codes_map
            .get(&compression_code)
            .map_or("UNKNOWN".to_string(), |s| s.to_owned())
    };

    let mut cursor = Cursor::new(&buffer);

    let fmt_chunk: FmtChunk = cursor.read_le().unwrap();

    let mut rows = vec![];
    rows.push(Row::from(vec!["chunk id", "'fmt '"]));

    rows.push(Row::from(vec![
        "size of fmt chunk (in bytes)",
        &fmt_chunk.chunk_size.to_string(),
    ]));

    rows.push(Row::from(vec![
        "compression code",
        &format!(
            "{} ({})",
            fmt_chunk.compression_code,
            get_compression_code_str(fmt_chunk.compression_code)
        ),
    ]));

    rows.push(Row::from(vec![
        "number of channels",
        &fmt_chunk.number_of_channels.to_string(),
    ]));
    rows.push(Row::from(vec![
        "sampling rate",
        &fmt_chunk.sample_rate.to_string(),
    ]));
    rows.push(Row::from(vec![
        "byte rate",
        &fmt_chunk.byte_rate.to_string(),
    ]));
    rows.push(Row::from(vec![
        "block align",
        &fmt_chunk.block_align.to_string(),
    ]));
    rows.push(Row::from(vec![
        "bits per sample",
        &fmt_chunk.bits_per_sample.to_string(),
    ]));

    if fmt_chunk.chunk_size == 18 {
        cursor.read_u16::<LittleEndian>().unwrap();
    } else if fmt_chunk.chunk_size == 40 {
        let extensible_chunk: ExtendedFmtChunk = cursor.read_le().unwrap();
        rows.push(Row::from(vec![
            "Extra Format Bytes",
            &extensible_chunk.extra_fmt_bytes_num.to_string(),
        ]));
        rows.push(Row::from(vec![
            "Number of valid bits",
            &extensible_chunk.num_valid_bits.to_string(),
        ]));
        rows.push(Row::from(vec![
            "Speaker position mask",
            &extensible_chunk.channel_mask.to_string(),
        ]));

        // compression code is taken from first 2 bytes of GUID
        let compression_code =
            u16::from_le_bytes(extensible_chunk.sub_format[0..2].try_into().unwrap());
        rows.push(Row::from(vec![
            "Actual compression code",
            &format!(
                "{} ({})",
                compression_code,
                get_compression_code_str(compression_code)
            ),
        ]));

        let guid_string = extensible_chunk
            .sub_format
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join("");

        rows.push(Row::from(vec!["GUID", &guid_string]));
    }

    Ok((rows, cursor.position() as usize))
}
