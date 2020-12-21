use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    let f = File::open("data/to_format.csv")?;
    let f = BufReader::new(f);
    let iter = f.lines().skip(5);

    let f2 = File::open("data/to_format_2.csv")?;
    let f2 = BufReader::new(f2);
    let iter = iter.chain(f2.lines().skip(5));

    let mut new_file = File::create("data/result_all_v2.csv")?;
    new_file.write_all(b"DATE;JOURNAL;GENERAL;AUXILIAIRE;LIBELLE;SENS;MONTANT\r\n")?;

    for line in iter {
        let v: Vec<String> = line?
            .split(';')
            .map(String::from)
            //.inspect(|s| println!("{}", s))
            .collect();

        let data: Vec<Entry> = v
            .chunks_exact(5)
            .map(|input| {
                let debit = input[3].replace(',', ".").parse::<f32>();
                let credit = input[4].replace(',', ".").parse::<f32>();

                let mut sens = "D";
                let mut amount = 0f32;

                if let Ok(value) = debit {
                    sens = "D";
                    amount = value * -1f32;
                } else if let Ok(value) = credit {
                    sens = "C";
                    amount = value;
                } else {
                    panic!("not debit nor credit")
                };

                Entry {
                    date: input[0].clone(),
                    journal: String::from("BNP"),
                    libelle: input[2].clone(),
                    sens: String::from(sens),
                    amount: amount.to_string().replace('.', ","),
                }
            })
            .collect();

        for (i, e) in data.iter().enumerate() {
            let i = i + 1;

            new_file.write_all(std::format!("{};", e.date.replace('/', "")).as_bytes())?;
            new_file.write_all(b"BNP;")?;
            new_file.write_all(b"471000;")?;
            new_file.write_all(b";")?;
            new_file.write_all(std::format!("{};", e.libelle).as_bytes())?;

            if e.sens == "C" {
                new_file.write_all(b"C;")?;
            } else {
                new_file.write_all(b"D;")?;
            }

            new_file.write_all(std::format!("{}", e.amount).as_bytes())?;

            new_file.write_all(b"\r\n")?;

            // next line
            new_file.write_all(std::format!("{};", e.date.replace('/', "")).as_bytes())?;
            new_file.write_all(b"BNP;")?;
            new_file.write_all(b"512000;")?;
            new_file.write_all(b";")?;
            new_file.write_all(std::format!("{};", e.libelle).as_bytes())?;

            if e.sens == "C" {
                new_file.write_all(b"D;")?;
            } else {
                new_file.write_all(b"C;")?;
            }

            new_file.write_all(std::format!("{}", e.amount).as_bytes())?;

            new_file.write_all(b"\r\n")?;
        }
    }

    Ok(())
}

// fn map_data_to_entry(input: &[&str]) {
//     let (sens, amount) = if let Ok(debit) = i32::from_str_radix(&input[3], 10) {
//         ("D", debit * -1)
//     } else if let Ok(credit) = i32::from_str_radix(&input[4], 10) {
//         ("C", credit)
//     } else {
//         panic!("not debit nor credit");
//     };

//     Entry {
//         date: input[0].clone(),
//         journal: "BNP",
//         sens: sens,
//         amount: amount,
//     }
// }

#[derive(Debug)]
struct Entry {
    date: String,
    journal: String,
    //general: String,
    //auxiliaire: String,
    sens: String,
    amount: String,
    libelle: String,
}
