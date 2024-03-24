use binrw::{io::Cursor, BinReaderExt};
use color_eyre::eyre::{eyre, Result};
use std::{fs::File, io::Read};

mod args;
mod parse;
mod print_chunk;
mod print_utils;

use parse::{DataChunk, FmtChunk, ID3v2Chunk, ListInfoChunk, RiffChunk, UnknownChunk};
use print_utils::print_position;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::Constraint,
    prelude::{CrosstermBackend, Terminal},
    widgets::{Row, Table},
};
use std::io::stdout;

fn display_in_tui(cursor: &mut Cursor<&Vec<u8>>) -> Result<()> {
    let riff_chunk: RiffChunk = cursor.read_le().unwrap();
    let mut chunk_names = vec![("RIFF", riff_chunk.file_size)];

    // riff_chunk.file_size excludes the RIFF header and the u32 describing the size (4 + 4 bytes)
    while cursor.position() as u32 - 8 < riff_chunk.file_size {
        if let Ok(fmt_chunk) = cursor.read_le::<FmtChunk>() {
            chunk_names.push(("fmt", fmt_chunk.chunk_size));
        } else if let Ok(data_chunk) = cursor.read_le::<DataChunk>() {
            chunk_names.push(("data", data_chunk.chunk_size));
        } else if let Ok(list_chunk) = cursor.read_le::<ListInfoChunk>() {
            chunk_names.push(("list", list_chunk.chunk_size));
        } else if let Ok(id3_chunk) = cursor.read_le::<ID3v2Chunk>() {
            chunk_names.push(("id3", id3_chunk.chunk_size));
        } else if let Ok(unknown_chunk) = cursor.read_le::<UnknownChunk>() {
            chunk_names.push(("unknown", unknown_chunk.chunk_size));
        } else {
            let mut word_buff = [0u8; 4];
            cursor.read_exact(&mut word_buff).unwrap();
            return Err(eyre!(
                "Unknown chunk: {:?}",
                String::from_utf8(word_buff.to_vec()).unwrap()
            ));
        }
    }

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let rows = chunk_names
                .iter()
                .map(|(name, size)| Row::new(vec![name.to_string(), size.to_string()]));
            frame.render_widget(
                Table::new(rows, [Constraint::from(10), Constraint::from(10)]),
                area,
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn display_stdout(cursor: &mut Cursor<&Vec<u8>>) -> Result<()> {
    println!();

    let riff_chunk: RiffChunk = cursor.read_le().unwrap();
    println!("{}", riff_chunk);
    print_position(cursor);

    let mut chunk_names = vec![("RIFF", riff_chunk.file_size)];

    // riff_chunk.file_size excludes the RIFF header and the u32 describing the size (4 + 4 bytes)
    while cursor.position() as u32 - 8 < riff_chunk.file_size {
        if let Ok(fmt_chunk) = cursor.read_le::<FmtChunk>() {
            println!("{}", fmt_chunk);
            chunk_names.push(("fmt", fmt_chunk.chunk_size));
        } else if let Ok(data_chunk) = cursor.read_le::<DataChunk>() {
            println!("{}", data_chunk);
            chunk_names.push(("data", data_chunk.chunk_size));
        } else if let Ok(list_chunk) = cursor.read_le::<ListInfoChunk>() {
            println!("{}", list_chunk);
            chunk_names.push(("list", list_chunk.chunk_size));
        } else if let Ok(id3_chunk) = cursor.read_le::<ID3v2Chunk>() {
            println!("{}", id3_chunk);
            chunk_names.push(("id3", id3_chunk.chunk_size));
        } else if let Ok(unknown_chunk) = cursor.read_le::<UnknownChunk>() {
            println!("{}", unknown_chunk);
            chunk_names.push(("unknown", unknown_chunk.chunk_size));
        } else {
            let mut word_buff = [0u8; 4];
            cursor.read_exact(&mut word_buff).unwrap();
            return Err(eyre!(
                "Unknown chunk: {:?}",
                String::from_utf8(word_buff.to_vec()).unwrap()
            ));
        }
        print_position(cursor);
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = args::get_args()?;
    let mut file = File::open(&args.file).unwrap();

    let mut buffer: Vec<u8> = vec![];
    file.read_to_end(&mut buffer).unwrap();

    let mut cursor = Cursor::new(&buffer);

    if args.tui {
        display_in_tui(&mut cursor)?;
    } else {
        display_stdout(&mut cursor)?;
    }

    // debug assertion that we actually parsed the whole file
    assert!(cursor.position() as usize == buffer.len());

    Ok(())
}
