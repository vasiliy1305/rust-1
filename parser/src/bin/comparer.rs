use clap::{Parser, ValueEnum};
use parser::FormatReader;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone, ValueEnum)]
enum FileFormat {
    Binary,
    Csv,
    Txt,
}

#[derive(Debug, Parser)]
#[command(name = "ypbank_compare")]
#[command(about = "Compare two files")]
struct Args {
    #[arg(long)]
    file1: String,

    #[arg(long)]
    format1: FileFormat,

    #[arg(long)]
    file2: String,

    #[arg(long)]
    format2: FileFormat,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file1 = File::open(args.file1.to_string())?;
    let reader1 = BufReader::new(file1);

    let data1 = match args.format1 {
        FileFormat::Csv => parser::csv_format::YPBankCsvFormat::load(reader1)?,
        FileFormat::Txt => parser::txt_format::YPBankTxtFormat::load(reader1)?,
        FileFormat::Binary => parser::bin_format::YPBankBinFormat::load(reader1)?,
    };

    let file2 = File::open(args.file2.to_string())?;
    let reader2 = BufReader::new(file2);

    let data2 = match args.format2 {
        FileFormat::Csv => parser::csv_format::YPBankCsvFormat::load(reader2)?,
        FileFormat::Txt => parser::txt_format::YPBankTxtFormat::load(reader2)?,
        FileFormat::Binary => parser::bin_format::YPBankBinFormat::load(reader2)?,
    };

    if data1.len() != data2.len() {
        println!(
            "Different number of records: '{}' has {}, '{}' has {}",
            args.file1,
            data1.len(),
            args.file2,
            data2.len()
        );
        return Ok(());
    }

    for (index, (left, right)) in data1.iter().zip(data2.iter()).enumerate() {
        if left != right {
            println!("Records differ at index {}", index);
            println!("file1: {:?}", left);
            println!("file2: {:?}", right);
            return Ok(());
        }
    }

    println!(
        "The transaction records in '{}' and '{}' are identical.",
        args.file1, args.file2
    );

    Ok(())
}
