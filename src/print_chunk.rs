use crate::parse_chunk::{
    get_compression_code_str, FmtChunk, ListInfoChunk, RiffChunk,
};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Row, Table};

pub fn print_riff_chunk(riff_chunk: &RiffChunk) {
    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);

    table.add_rows(vec![
        Row::from(vec!["chunk id", "'RIFF'"]),
        Row::from(vec![
            "size of file (in bytes)",
            &riff_chunk.file_size.to_string(),
        ]),
        Row::from(vec!["wave identifier", &riff_chunk.wave_ident]),
    ]);

    println!("{table}");
}

pub fn print_fmt_chunk(fmt_chunk: &FmtChunk) {
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
            &format!(
                "{} ({})",
                fmt_chunk.compression_code,
                get_compression_code_str(fmt_chunk.compression_code),
            ),
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

    if let Some(extended_chunk) = &fmt_chunk.extended_fmt_sub_chunk {
        table.add_rows(vec![
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
                &format!(
                    "{} ({})",
                    extended_chunk.compression_code,
                    get_compression_code_str(extended_chunk.compression_code),
                ),
            ]),
            Row::from(vec![
                "WAV GUID",
                &extended_chunk
                    .wav_guid
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<String>>()
                    .join(" "),
            ]),
        ]);
    }

    println!("{table}");
}

pub fn print_list_chunk(list_chunk: &ListInfoChunk) {
    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);

    table.add_rows(vec![
        Row::from(vec!["chunk id", "'LIST'"]),
        Row::from(vec![
            "size of LIST chunk (in bytes)",
            &list_chunk.chunk_size.to_string(),
        ]),
    ]);
    table.add_rows(
        list_chunk.data
            .iter()
            .map(|sub_chunk| Row::from(vec![sub_chunk.info_id.clone(), sub_chunk.text.clone()])),
    );

    println!("{table}");
}
