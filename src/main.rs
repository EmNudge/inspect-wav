use comfy_table::Row;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Table};
use std::collections::BTreeMap;
use std::{env, fs::File, io::Read, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(path) = args.get(1) else {
        println!("Usage: inspect_wav <path to wav file>");
        return;
    };

    if !Path::new(path).exists() {
        println!("ERR: file does not exist");
        return;
    }

    let mut file = File::open(path).unwrap();

    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec!["Property", "Value"]);

    let mut buffer = [0; 300];
    file.read(&mut buffer).unwrap();

    table.add_rows(parse_riff_chunk(&buffer).unwrap());
    table.add_rows(parse_fmt_chunk(&buffer).unwrap());

    println!("{table}");
}


fn buf_to_str(buffer: &[u8]) -> String {
    if buffer.len() == 2 {
        u16::from_le_bytes(buffer.try_into().unwrap()).to_string()
    } else {
        u32::from_le_bytes(buffer.try_into().unwrap()).to_string()
    }
}

fn parse_riff_chunk(buffer: &[u8]) -> Result<Vec<Row>, String> {
    let mut rows = vec![];

    // RIFF chunk
    if b"RIFF" == &buffer[..4] {
        rows.push(Row::from(vec!["chunk id", "'RIFF'"]));
    } else {
        return Err("Does not have 'RIFF' header".to_string());
    }

    rows.push(Row::from(vec![
        "size of file (in bytes)",
        &buf_to_str(&buffer[4..8]),
    ]));

    if b"WAVE" == &buffer[8..12] {
        rows.push(Row::from(vec!["wave identifier", "WAVE"]));
    } else {
        return Err("Does not have 'WAVE' identifier".to_string());
    }

    Ok(rows)
}

fn parse_fmt_chunk(buffer: &[u8]) -> Result<Vec<Row>, String> {
    let compression_codes_map = {
        let compression_codes_str = include_str!("compression_codes.json");
        let compression_codes: Vec<(u16, String)> =
            serde_json::from_str(compression_codes_str).unwrap();
        compression_codes
            .into_iter()
            .collect::<BTreeMap<u16, String>>()
    };

    let mut rows = vec![];
    // fmt chunk
    if b"fmt " == &buffer[12..16] {
        rows.push(Row::from(vec!["chunk id", "'fmt '"]));
    } else {
        return Err("Does not have 'fmt ' chunk".to_string());
    }

    let chunk_size = u32::from_le_bytes(buffer[16..20].try_into().unwrap());
    rows.push(Row::from(vec![
        "size of fmt chunk (in bytes)",
        &chunk_size.to_string(),
    ]));

    let compression_code = u16::from_le_bytes(buffer[20..22].try_into().unwrap());
    let compression_code_str_def = "UNKNOWN".to_string();
    let compression_code_str = compression_codes_map
        .get(&compression_code)
        .unwrap_or(&compression_code_str_def);

    rows.push(Row::from(vec![
        "compression code",
        &format!("{} ({})", compression_code, compression_code_str),
    ]));

    rows.push(Row::from(vec![
        "number of channels",
        &buf_to_str(&buffer[22..24]),
    ]));
    rows.push(Row::from(vec![
        "sampling rate",
        &buf_to_str(&buffer[24..28]),
    ]));
    rows.push(Row::from(vec!["byte rate", &buf_to_str(&buffer[28..32])]));
    rows.push(Row::from(vec!["block align", &buf_to_str(&buffer[32..34])]));
    rows.push(Row::from(vec![
        "bits per sample",
        &buf_to_str(&buffer[34..36]),
    ]));

    if chunk_size > 16 {
        let extra_fmt_bytes_num = u16::from_le_bytes(buffer[36..38].try_into().unwrap());
        rows.push(Row::from(vec![
            "number of extra format bytes",
            &extra_fmt_bytes_num.to_string(),
        ]));

        if chunk_size > 18 {
            let buffer_as_string = &buffer[40..(40 + extra_fmt_bytes_num as usize)]
                .into_iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join(" ");
            rows.push(Row::from(vec![
                "extra format bytes",
                &buffer_as_string,
            ]));
        }
    }

    Ok(rows)
}