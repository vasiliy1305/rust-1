use clap::{Parser, ValueEnum};
use parser::{FormatReader, FormatWriter};
use std::fs::File;
use std::io::{BufReader, BufWriter};

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
    file1_format: FileFormat,

    #[arg(long)]
    file2: String,

    #[arg(long)]
    file2_format: FileFormat,
}

fn main() {
    let args = Args::parse();

    println!("file1   = {}", args.file1);
    println!("file1_format = {:?}", args.file1_format);
    println!("file2   = {}", args.file2);
    println!("file2_format = {:?}", args.file2_format);

    let file1 = File::open(args.file1.to_string()).unwrap();
    let reader1 = BufReader::new(file1);

    let data1 = match args.file1_format {
        FileFormat::Csv => parser::csv_format::YPBankCsvFormat::load(reader1).unwrap(),
        FileFormat::Txt => parser::txt_format::YPBankTxtFormat::load(reader1).unwrap(),
        FileFormat::Binary => parser::bin_format::YPBankBinFormat::load(reader1).unwrap(),
    };

    let file2 = File::open(args.file2.to_string()).unwrap();
    let reader2 = BufReader::new(file2);

    let data2 = match args.file2_format {
        FileFormat::Csv => parser::csv_format::YPBankCsvFormat::load(reader2).unwrap(),
        FileFormat::Txt => parser::txt_format::YPBankTxtFormat::load(reader2).unwrap(),
        FileFormat::Binary => parser::bin_format::YPBankBinFormat::load(reader2).unwrap(),
    };

    // compire size
    if data1.len() != data2.len(){
        println!("Diferent size of file, file1.len = {}, file2.len = {}", data1.len(), data2.len());
    }

}
