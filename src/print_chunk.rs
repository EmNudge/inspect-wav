use crate::parse::{
    get_compression_code_str, DataChunk, FmtChunk, ID3v2Chunk, ListInfoChunk, RiffChunk,
    UnknownChunk,
};
use crate::print_utils::print_rows;
use owo_colors::OwoColorize;

pub fn print_riff_chunk(riff_chunk: &RiffChunk) {
    print_rows(vec![
        ("chunk id", "'RIFF'".blue().to_string()),
        (
            "size of file (in bytes)",
            riff_chunk.file_size.green().to_string(),
        ),
        ("wave identifier", riff_chunk.wave_ident.to_string()),
    ]);
}

pub fn print_fmt_chunk(fmt_chunk: &FmtChunk) {
    let mut rows = vec![
        ("chunk id", "'fmt '".blue().to_string()),
        (
            "size of fmt chunk (in bytes)",
            fmt_chunk.chunk_size.to_string().green().to_string(),
        ),
        (
            "compression code",
            format!(
                "{} ({})",
                fmt_chunk.compression_code,
                get_compression_code_str(fmt_chunk.compression_code),
            ),
        ),
        (
            "number of channels",
            fmt_chunk.number_of_channels.to_string(),
        ),
        ("sampling rate", fmt_chunk.sample_rate.to_string()),
        ("byte rate", fmt_chunk.byte_rate.to_string()),
        ("block align", fmt_chunk.block_align.to_string()),
        ("bits per sample", fmt_chunk.bits_per_sample.to_string()),
    ];

    if let Some(extra_bytes) = &fmt_chunk.extra_bytes {
        rows.push(("extra format bytes", extra_bytes.to_string()));
    }

    if let Some(extended_chunk) = &fmt_chunk.extended_fmt_sub_chunk {
        rows.extend(vec![
            (
                "number of valid bits",
                extended_chunk.num_valid_bits.to_string(),
            ),
            (
                "speaker position mask",
                extended_chunk.channel_mask.to_string(),
            ),
            (
                "actual compression code",
                format!(
                    "{} ({})",
                    extended_chunk.compression_code,
                    get_compression_code_str(extended_chunk.compression_code),
                ),
            ),
            (
                "WAV GUID",
                extended_chunk
                    .wav_guid
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
        ]);
    }

    print_rows(rows);
}

pub fn print_list_chunk(list_chunk: &ListInfoChunk) {
    let mut table = vec![
        ("chunk id".to_string(), "'LIST'".blue().to_string()),
        (
            "size of LIST chunk (in bytes)".to_string(),
            list_chunk.chunk_size.to_string().green().to_string(),
        ),
    ];

    let mut text_chunks: Vec<(String, String)> = list_chunk
        .data
        .iter()
        .map(|sub_chunk| (sub_chunk.info_id.clone(), sub_chunk.text.clone()))
        .collect();

    table.append(&mut text_chunks);

    print_rows(
        table
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect(),
    );
}

pub fn print_data_chunk(data_chunk: &DataChunk) {
    print_rows(vec![
        ("chunk id", &"'data'".blue().to_string()),
        (
            "size of data chunk (in bytes)",
            &data_chunk.chunk_size.to_string().green().to_string(),
        ),
        (
            "data... (minimized)",
            &format!("[ ...{} items ]", &data_chunk.sample_data.len()),
        ),
    ]);
}

pub fn print_id3_chunk(id3_chunk: &ID3v2Chunk) {
    let mut rows = vec![
        ("chunk id", "'id3'".blue().to_string()),
        (
            "size of id3 chunk (in bytes)",
            id3_chunk.chunk_size.to_string().green().to_string(),
        ),
        ("major version", id3_chunk.major_version.to_string()),
        ("minor version", id3_chunk.minor_version.to_string()),
        ("flags", format!("{:08b}", id3_chunk.flags)),
        (
            "size of id3v2 (self reported)",
            id3_chunk.id3v2_size.to_string(),
        ),
    ];

    if let Some(_xheader) = &id3_chunk.xheader {
        todo!();
    }

    for tag in &id3_chunk.tags {
        rows.push(("tag", format!("{}: {}", tag.frame_id, tag.data)));
    }

    print_rows(rows);
}

fn as_maybe_utf8(bytes: Vec<u8>) -> String {
    let mut result = Vec::new();
    let mut seen_null_byte = false;

    for byte in bytes {
        if byte == 0x00 {
            if !seen_null_byte {
                seen_null_byte = true;
                result.push(byte);
            }
        } else {
            seen_null_byte = false;
            result.push(byte);
        }
    }

    result
        .split(|x| *x == 0)
        .collect::<Vec<&[u8]>>()
        .into_iter()
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect::<Vec<String>>()
        .join(&" ...\\0 ".dimmed().to_string())
}

pub fn print_unknown_chunk(unknown_chunk: &UnknownChunk) {
    print_rows(vec![
        ("chunk id", format!("{} (unknown)", unknown_chunk.chunk_id)),
        (
            "size of file (in bytes)",
            unknown_chunk.chunk_size.green().to_string(),
        ),
        (
            "data (utf-8 parse attempt)",
            as_maybe_utf8(unknown_chunk.data.clone()),
        ),
    ]);
}
