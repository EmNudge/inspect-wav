use binrw::io::Cursor;
use owo_colors::OwoColorize;

pub fn print_position<T>(cursor: &Cursor<T>) {
    println!(
        "{}",
        format!("| parsed {} bytes", cursor.position()).dimmed()
    );
}

pub fn print_rows(rows: Vec<(impl ToString, impl ToString)>) {
    let string_rows: Vec<(String, String)> = rows
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let max_key_width = string_rows.iter().map(|(k, _v)| k.len()).max().unwrap();
    let max_value_width = string_rows.iter().map(|(_k, v)| v.len()).max().unwrap();

    println!(
        "{}",
        format!("╭{}", "-".repeat(max_key_width + max_value_width + 4)).dimmed()
    );
    let table = string_rows
        .iter()
        .map(|(k, v)| {
            format!(
                "{} {k}{}{v}",
                "|".dimmed(),
                " ".repeat(max_key_width - k.len() + 4),
            )
        })
        .collect::<Vec<String>>()
        .join(
            &format!(
                "\n|{}+{}\n",
                "-".repeat(max_key_width + 2),
                "-".repeat(max_value_width)
            )
            .dimmed()
            .to_string(),
        );

    println!("{}", table);
    println!(
        "{}",
        format!("╰{}", "-".repeat(max_key_width + max_value_width + 4)).dimmed()
    );
}
