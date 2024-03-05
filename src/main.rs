use std::{env, fs::File, io::Read, path::Path};
use std::io::Cursor;
use std::collections::BTreeMap;
use byteorder::{LittleEndian, ReadBytesExt};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Table, Row};

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

fn parse_riff_chunk(buffer: &[u8]) -> Result<Vec<Row>, String> {
    let mut rows = vec![];

    let mut cursor = Cursor::new(buffer);

    let mut riff_buffer = [0; 4];
    cursor.read_exact(&mut riff_buffer).unwrap();

    // RIFF chunk
    if b"RIFF" == &riff_buffer {
        rows.push(Row::from(vec!["chunk id", "'RIFF'"]));
    } else {
        return Err("Does not have 'RIFF' header".to_string());
    }

    let file_size = cursor.read_u32::<LittleEndian>().unwrap();
    rows.push(Row::from(vec![
        "size of file (in bytes)",
        &file_size.to_string(),
    ]));

    let mut wave_buffer = [0; 4];
    cursor.read_exact(&mut wave_buffer).unwrap();
    if b"WAVE" == &wave_buffer {
        rows.push(Row::from(vec!["wave identifier", "WAVE"]));
    } else {
        return Err("Does not have 'WAVE' identifier".to_string());
    }

    Ok(rows)
}

const COMPRESSION_CODES_STR: &'static str = include_str!("compression_codes.json");

fn parse_fmt_chunk(buffer: &[u8]) -> Result<Vec<Row>, String> {
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

    let mut cursor = Cursor::new(&buffer[12..]);

    let mut rows = vec![];

    let mut id_buffer = [0; 4];
    cursor.read_exact(&mut id_buffer).unwrap();
    if b"fmt " == &id_buffer {
        rows.push(Row::from(vec!["chunk id", "'fmt '"]));
    } else {
        return Err("Does not have 'fmt ' chunk".to_string());
    }

    let chunk_size = cursor.read_u32::<LittleEndian>().unwrap();
    rows.push(Row::from(vec![
        "size of fmt chunk (in bytes)",
        &chunk_size.to_string(),
    ]));

    let compression_code = cursor.read_u16::<LittleEndian>().unwrap();
    let compression_code_str = get_compression_code_str(compression_code);

    rows.push(Row::from(vec![
        "compression code",
        &format!("{} ({})", compression_code, compression_code_str),
    ]));

    rows.push(Row::from(vec![
        "number of channels",
        &cursor.read_u16::<LittleEndian>().unwrap().to_string(),
    ]));
    rows.push(Row::from(vec![
        "sampling rate",
        &cursor.read_u32::<LittleEndian>().unwrap().to_string(),
    ]));
    rows.push(Row::from(vec![
        "byte rate",
        &cursor.read_u32::<LittleEndian>().unwrap().to_string(),
    ]));
    rows.push(Row::from(vec![
        "block align",
        &cursor.read_u16::<LittleEndian>().unwrap().to_string(),
    ]));
    rows.push(Row::from(vec![
        "bits per sample",
        &cursor.read_u16::<LittleEndian>().unwrap().to_string(),
    ]));

    if chunk_size > 16 {
        // this should be 0 or 22
        let _extra_fmt_bytes_num = cursor.read_u16::<LittleEndian>().unwrap();

        if chunk_size > 18 {
            rows.push(Row::from(vec![
                "Number of valid bits",
                &cursor.read_u16::<LittleEndian>().unwrap().to_string(),
            ]));

            rows.push(Row::from(vec![
                "Speaker position mask",
                &cursor.read_u32::<LittleEndian>().unwrap().to_string(),
            ]));

            let compression_code = cursor.read_u16::<LittleEndian>().unwrap();
            let compression_code_str = get_compression_code_str(compression_code);

            rows.push(Row::from(vec![
                "Actual compression code",
                &format!("{} ({})", compression_code, compression_code_str),
            ]));

            // the GUID is 16 bytes, but the first 2 is the compression code 
            // (which we just read)
            let mut guid_string = [0; 14];
            cursor.read_exact(&mut guid_string).unwrap();

            let sub_format = guid_string
                .into_iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join("");

            rows.push(Row::from(vec!["GUID", &sub_format]));
        }
    }

    Ok(rows)
}
