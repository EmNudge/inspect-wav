use crate::parse_chunk::{ExtendedFmtChunk, FmtChunk, RiffChunk};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Row, Table};

pub fn print_riff_chunk(riff_chunk: RiffChunk) {
    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);

    table.add_rows(vec![
        Row::from(vec!["chunk id", "'RIFF'"]),
        Row::from(vec![
            "size of file (in bytes)",
            &riff_chunk.file_size.to_string(),
        ]),
        Row::from(vec!["wave identifier", &riff_chunk.get_wave_ident()]),
    ]);

    println!("{table}");
}
pub fn print_fmt_chunk(fmt_chunk: FmtChunk, extended_chunk: Option<ExtendedFmtChunk>) {
    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);

    table.add_rows(vec![
        Row::from(vec!["chunk id", "'fmt '"]),
        Row::from(vec![
            "size of fmt chunk (in bytes)",
            &fmt_chunk.chunk_size.to_string(),
        ]),
        Row::from(vec![
            "compression code",
            &fmt_chunk.get_compression_code_str(),
        ]),
        Row::from(vec![
            "number of channels",
            &fmt_chunk.number_of_channels.to_string(),
        ]),
        Row::from(vec!["sampling rate", &fmt_chunk.sample_rate.to_string()]),
        Row::from(vec!["byte rate", &fmt_chunk.byte_rate.to_string()]),
        Row::from(vec!["block align", &fmt_chunk.block_align.to_string()]),
        Row::from(vec![
            "bits per sample",
            &fmt_chunk.bits_per_sample.to_string(),
        ]),
    ]);

    if let Some(extended_chunk) = extended_chunk {
        table.add_rows(vec![
            Row::from(vec![
                "Extra Format Bytes",
                &extended_chunk.extra_fmt_bytes_num.to_string(),
            ]),
            Row::from(vec![
                "Number of valid bits",
                &extended_chunk.num_valid_bits.to_string(),
            ]),
            Row::from(vec![
                "Speaker position mask",
                &extended_chunk.channel_mask.to_string(),
            ]),
            Row::from(vec![
                "Actual compression code",
                &extended_chunk.get_compression_code_str(),
            ]),
            Row::from(vec!["GUID", &extended_chunk.get_guid()]),
        ]);
    }

    println!("{table}");
}
