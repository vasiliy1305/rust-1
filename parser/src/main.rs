use clap::{Parser, ValueEnum};
use parser::{FormatReader, FormatWriter};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};
use std::process::Output;

#[derive(Debug, Clone, ValueEnum)]
enum FileFormat {
    Binary,
    Csv,
    Txt,
}

#[derive(Debug, Parser)]
#[command(name = "ypbank_converter")]
#[command(about = "Convert one format to other")]
struct Args {
    #[arg(long)]
    input: String,

    #[arg(long)]
    input_format: FileFormat,

    #[arg(long)]
    output: String,

    #[arg(long)]
    output_format: FileFormat,
}

fn main() {
    let args = Args::parse();

    println!("input   = {}", args.input);
    println!("input-format = {:?}", args.input_format);
    println!("output   = {}", args.output);
    println!("output-format = {:?}", args.output_format);

    let mut input = File::open(args.input.to_string()).unwrap();
    let mut reader = BufReader::new(input);

    let data = match args.input_format {
        FileFormat::Csv => parser::csv_format::YPBankCsvFormat::load(reader),
        FileFormat::Txt => parser::txt_format::YPBankTxtFormat::load(reader),
        FileFormat::Binary => parser::bin_format::YPBankBinFormat::load(reader),
    };

    let mut output = File::create(args.output.to_string()).unwrap();
    let mut writer = BufWriter::new(output);

    let status = match args.output_format {
        FileFormat::Csv => parser::csv_format::YPBankCsvFormat::save(&mut writer, &data.unwrap()),

        FileFormat::Txt => parser::txt_format::YPBankTxtFormat::save(&mut writer, &data.unwrap()),

        FileFormat::Binary => {
            parser::bin_format::YPBankBinFormat::save(&mut writer, &data.unwrap())
        }
    };

    if status.is_ok(){
        println!("Succssess saved!")
    }else{

    }
}
