use crate::parse_chunk::{get_compression_code_str, DataChunk, FmtChunk, ListInfoChunk, RiffChunk};
use owo_colors::OwoColorize;

fn print_rows(rows: Vec<(&str, &str)>) {
    let max_width = rows.iter().map(|(k, _v)| k.len()).max().unwrap();
    let line = "-".repeat(max_width);

    println!("{}", format!("╭--{}", &line).dimmed());
    let table = rows
        .iter()
        .map(|(k, v)| {
            format!(
                "{} {k}{}{v}",
                "|".dimmed(),
                " ".repeat(max_width - k.len() + 4)
            )
        })
        .collect::<Vec<String>>()
        .join(&format!("\n|---{}\n", &line).dimmed().to_string());

    println!("{}", table);
    println!("{}", format!("╰--{}", &line).dimmed());
}

pub fn print_riff_chunk(riff_chunk: &RiffChunk) {
    print_rows(vec![
        ("chunk id", &"'RIFF'".blue().to_string()),
        (
            "size of file (in bytes)",
            &riff_chunk.file_size.to_string().green().to_string(),
        ),
        ("wave identifier", &riff_chunk.wave_ident),
    ]);
}

pub fn print_fmt_chunk(fmt_chunk: &FmtChunk) {
    print_rows(vec![
        ("chunk id", &"'fmt '".blue().to_string()),
        (
            "size of fmt chunk (in bytes)",
            &fmt_chunk.chunk_size.to_string().green().to_string(),
        ),
        (
            "compression code",
            &format!(
                "{} ({})",
                fmt_chunk.compression_code,
                get_compression_code_str(fmt_chunk.compression_code),
            ),
        ),
        (
            "number of channels",
            &fmt_chunk.number_of_channels.to_string(),
        ),
        ("sampling rate", &fmt_chunk.sample_rate.to_string()),
        ("byte rate", &fmt_chunk.byte_rate.to_string()),
        ("block align", &fmt_chunk.block_align.to_string()),
        ("bits per sample", &fmt_chunk.bits_per_sample.to_string()),
    ]);

    if let Some(extended_chunk) = &fmt_chunk.extended_fmt_sub_chunk {
        print_rows(vec![
            (
                "Number of valid bits",
                &extended_chunk.num_valid_bits.to_string(),
            ),
            (
                "Speaker position mask",
                &extended_chunk.channel_mask.to_string(),
            ),
            (
                "Actual compression code",
                &format!(
                    "{} ({})",
                    extended_chunk.compression_code,
                    get_compression_code_str(extended_chunk.compression_code),
                ),
            ),
            (
                "WAV GUID",
                &extended_chunk
                    .wav_guid
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
        ]);
    }
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
