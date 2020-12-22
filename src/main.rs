use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};
use std::path::Path;

fn main() -> Result<()> {
    let config = get_global_config();
    let iter = config
        .inputs
        .iter()
        .filter_map(|input| File::open(input).ok())
        .map(BufReader::new)
        .flat_map(|f| f.lines().skip(5));

    let mut new_file = File::create(config.output)?;
    new_file.write_all(b"DATE;JOURNAL;GENERAL;AUXILIAIRE;LIBELLE;SENS;MONTANT\r\n")?;

    for line in iter {
        let v: Vec<String> = line?.split(';').map(String::from).collect();

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

fn get_global_config() -> GlobalConfig {
    let mut args_iter = env::args().skip(1);
    let mut curr = args_iter.next();
    let mut inputs = vec![];
    let mut output = String::from("output.csv");

    while let Some(value) = curr {
        if value.starts_with('-') && value != "-o" {
            panic!("Invalide argument: {}", value);
        } else if value == "-o" {
            curr = args_iter.next();

            if let Some(custom_output_str) = curr {
                output = custom_output_str;
            }
        } else {
            inputs.push(value);
        }

        curr = args_iter.next();
    }

    let config = GlobalConfig {
        inputs: inputs,
        output: output,
    };

    validate_config(&config);

    config
}

fn validate_config(config: &GlobalConfig) {
    for input in &config.inputs {
        let input_path = Path::new(input);

        if !input_path.exists() || !input_path.is_file() {
            panic!("Invalid input path: {}", input);
        }
    }

    if Path::new(&config.output).exists() {
        panic!("Output already exits: {}", config.output);
    }
}

#[derive(Debug)]
struct GlobalConfig {
    inputs: Vec<String>,
    output: String,
}

#[derive(Debug)]
struct Entry {
    date: String,
    libelle: String,
    amount: f32,
}
