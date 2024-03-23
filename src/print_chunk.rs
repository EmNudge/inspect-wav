use std::fmt::Display;

use crate::parse::{
    get_compression_code_str, DataChunk, FmtChunk, ID3v2Chunk, ListInfoChunk, RiffChunk,
    UnknownChunk,
};
use crate::print_utils::get_rows_string;
use owo_colors::OwoColorize;

impl Display for RiffChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = vec![
            ("chunk id", "'RIFF'".blue().to_string()),
            (
                "size of file (in bytes)",
                self.file_size.green().to_string(),
            ),
            ("wave identifier", self.wave_ident.to_string()),
        ];
        write!(f, "{}", get_rows_string(rows))?;
        Ok(())
    }
}

impl Display for DataChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = vec![
            ("chunk id", "'data'".blue().to_string()),
            (
                "size of data chunk (in bytes)",
                self.chunk_size.to_string().green().to_string(),
            ),
            (
                "data... (minimized)",
                format!("[ ...{} items ]", &self.sample_data.len()),
            ),
        ];
        write!(f, "{}", get_rows_string(rows))?;
        Ok(())
    }
}

impl Display for FmtChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = vec![
            ("chunk id", "'fmt '".blue().to_string()),
            (
                "size of fmt chunk (in bytes)",
                self.chunk_size.to_string().green().to_string(),
            ),
            (
                "compression code",
                format!(
                    "{} ({})",
                    self.compression_code,
                    get_compression_code_str(self.compression_code),
                ),
            ),
            ("number of channels", self.number_of_channels.to_string()),
            ("sampling rate", self.sample_rate.to_string()),
            ("byte rate", self.byte_rate.to_string()),
            ("block align", self.block_align.to_string()),
            ("bits per sample", self.bits_per_sample.to_string()),
        ];

        if let Some(extra_bytes) = &self.extra_bytes {
            rows.push(("extra format bytes", extra_bytes.to_string()));
        }

        if let Some(extended_chunk) = &self.extended_fmt_sub_chunk {
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

        write!(f, "{}", get_rows_string(rows))?;
        Ok(())
    }
}

impl Display for ListInfoChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = vec![
            ("chunk id".to_string(), "'LIST'".blue().to_string()),
            (
                "size of LIST chunk (in bytes)".to_string(),
                self.chunk_size.to_string().green().to_string(),
            ),
        ];
        let mut text_chunks: Vec<(String, String)> = self
            .data
            .iter()
            .map(|sub_chunk| (sub_chunk.info_id.clone(), sub_chunk.text.clone()))
            .collect();

        rows.append(&mut text_chunks);

        write!(f, "{}", get_rows_string(rows))?;

        Ok(())
    }
}

impl Display for ID3v2Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = vec![
            ("chunk id", "'id3 '".blue().to_string()),
            (
                "size of id3 chunk (in bytes)",
                self.chunk_size.to_string().green().to_string(),
            ),
            ("major version", self.major_version.to_string()),
            ("minor version", self.minor_version.to_string()),
            ("flags", format!("{:08b}", self.flags)),
            ("size of id3v2 (self reported)", self.id3v2_size.to_string()),
        ];

        if let Some(_xheader) = &self.xheader {
            todo!();
        }

        for tag in &self.tags {
            rows.push(("tag", format!("{}: {}", tag.frame_id, tag.data)));
        }

        write!(f, "{}", get_rows_string(rows))?;
        Ok(())
    }
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

impl Display for UnknownChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = vec![
            ("chunk id", format!("{} (unknown)", self.chunk_id)),
            (
                "size of file (in bytes)",
                self.chunk_size.green().to_string(),
            ),
            (
                "data (utf-8 parse attempt)",
                as_maybe_utf8(self.data.clone()),
            ),
        ];
        write!(f, "{}", get_rows_string(rows))?;
        Ok(())
    }
}