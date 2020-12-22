use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    let f = File::open("data/to_format.csv")?;
    let f = BufReader::new(f);
    let iter = f.lines().skip(5);

    let f2 = File::open("data/to_format_2.csv")?;
    let f2 = BufReader::new(f2);
    let iter = iter.chain(f2.lines().skip(5));

    let mut new_file = File::create("data/result_all_v3.csv")?;
    new_file.write_all(b"DATE;JOURNAL;GENERAL;AUXILIAIRE;LIBELLE;SENS;MONTANT\r\n")?;

    for line in iter {
        let v: Vec<String> = line?
            .split(';')
            .map(String::from)
            //.inspect(|s| println!("{}", s))
            .collect();

        v.chunks_exact(5)
            .map(map_input_data_to_entry)
            .map(map_entry_to_output_format)
            .for_each(|lines| new_file.write_all(lines.as_bytes()).unwrap());
    }

    Ok(())
}

fn map_input_data_to_entry(input: &[String]) -> Entry {
    Entry {
        date: input[0].clone(),
        libelle: input[2].clone(),
        amount: input[3].replace(',', ".").parse::<f32>().unwrap_or(0f32)
            + input[4].replace(',', ".").parse::<f32>().unwrap_or(0f32),
    }
}

fn map_entry_to_output_format(entry: Entry) -> String {
    let first_line: String = format!(
        "{DATE};{JOURNAL};{GENERAL};{AUXILIAIRE};{LIBELLE};{SENS};{MONTANT}",
        DATE = entry.date.replace('/', ""),
        JOURNAL = "BNP",
        GENERAL = "471000",
        AUXILIAIRE = "",
        LIBELLE = entry.libelle,
        SENS = if entry.amount > 0f32 { "C" } else { "D" },
        MONTANT = entry.amount.abs().to_string().replace('.', ",")
    );

    let second_line: String = format!(
        "{DATE};{JOURNAL};{GENERAL};{AUXILIAIRE};{LIBELLE};{SENS};{MONTANT}",
        DATE = entry.date.replace('/', ""),
        JOURNAL = "BNP",
        GENERAL = "512000",
        AUXILIAIRE = "",
        LIBELLE = entry.libelle,
        SENS = if entry.amount > 0f32 { "D" } else { "C" },
        MONTANT = entry.amount.abs().to_string().replace('.', ",")
    );

    format!("{}\r\n{}\r\n", first_line, second_line)
}

#[derive(Debug)]
struct Entry {
    date: String,
    libelle: String,
    amount: f32,
}
