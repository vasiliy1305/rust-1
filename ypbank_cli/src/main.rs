use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::io::{BufReader, BufWriter};
use ypbank_parser::{FormatReader, FormatWriter};

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
    input: Option<String>,

    #[arg(long)]
    output: Option<String>,

    #[arg(long = "in-format")]
    in_format: FileFormat,

    #[arg(long = "out-format")]
    out_format: FileFormat,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let data = match args.input {
        Some(path) => {
            let input = File::open(path)?;
            let reader = BufReader::new(input);
            load_by_format(reader, &args.in_format)?
        }
        None => {
            let input = io::stdin();
            let reader = input.lock();
            load_by_format(reader, &args.in_format)?
        }
    };

    match args.output {
        Some(path) => {
            let output = File::create(path)?;
            let mut writer = BufWriter::new(output);
            save_by_format(&mut writer, &args.out_format, &data)?
        }
        None => {
            let stdout = io::stdout();
            let mut writer = BufWriter::new(stdout.lock());
            save_by_format(&mut writer, &args.out_format, &data)?
        }
    }

    Ok(())
}

fn load_by_format<R: Read>(
    reader: R,
    format: &FileFormat,
) -> Result<Vec<ypbank_parser::YPBankRecord>, ypbank_parser::error::ParserError> {
    match format {
        FileFormat::Csv => ypbank_parser::csv_format::YPBankCsvFormat::load(reader),
        FileFormat::Txt => ypbank_parser::txt_format::YPBankTxtFormat::load(reader),
        FileFormat::Binary => ypbank_parser::bin_format::YPBankBinFormat::load(reader),
    }
}

fn save_by_format<W: Write>(
    writer: &mut W,
    format: &FileFormat,
    data: &[ypbank_parser::YPBankRecord],
) -> Result<(), ypbank_parser::error::ParserError> {
    match format {
        FileFormat::Csv => ypbank_parser::csv_format::YPBankCsvFormat::save(writer, data),
        FileFormat::Txt => ypbank_parser::txt_format::YPBankTxtFormat::save(writer, data),
        FileFormat::Binary => ypbank_parser::bin_format::YPBankBinFormat::save(writer, data),
    }
}
