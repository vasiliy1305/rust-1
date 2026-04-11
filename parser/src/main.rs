use clap::{Parser, ValueEnum};
use parser::{FormatReader, FormatWriter};
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};

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
    output_format: FileFormat,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input = File::open(&args.input)?;
    let reader = BufReader::new(input);

    let data = match args.input_format {
        FileFormat::Csv => parser::csv_format::YPBankCsvFormat::load(reader),
        FileFormat::Txt => parser::txt_format::YPBankTxtFormat::load(reader),
        FileFormat::Binary => parser::bin_format::YPBankBinFormat::load(reader),
    };

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    match args.output_format {
        FileFormat::Csv => parser::csv_format::YPBankCsvFormat::save(&mut writer, &data?)?,
        FileFormat::Txt => parser::txt_format::YPBankTxtFormat::save(&mut writer, &data?)?,
        FileFormat::Binary => parser::bin_format::YPBankBinFormat::save(&mut writer, &data?)?,
    };
    Ok(())
}
