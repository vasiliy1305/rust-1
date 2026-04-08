use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

// use parser::bin_format::BinFormat;
// use parser::txt_format::TxtFormat;
use parser::{FormatReader, FormatWriter, YPBankRecord};

fn main() {
    // let mut file =
    //     File::open("C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.csv").unwrap();
    // let mut reader = BufReader::new(file);
    // let data = CsvFormat::load(reader).unwrap();

    // let mut file =
    //     File::create("C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example_copy.csv")
    //         .unwrap();
    // let mut writer = BufWriter::new(file);
    // CsvFormat::save(&mut writer, &data);

    // let mut file =
    //     File::open("C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.txt").unwrap();
    // let mut reader = BufReader::new(file);
    // let data = TxtFormat::load(reader).unwrap();

    // let mut file = File::create(
    //     "C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example_txt_to_csv.csv",
    // )
    // .unwrap();
    // let mut writer = BufWriter::new(file);
    // CsvFormat::save(&mut writer, &data);

    // let mut file = File::create(
    //     "C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example_txt_copy.csv",
    // )
    // .unwrap();
    // let mut writer = BufWriter::new(file);
    // TxtFormat::save(&mut writer, &data);

    // let file =
    //     File::open("C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.bin").unwrap();
    // let mut reader = BufReader::new(file);
    // let data = BinFormat::load(&mut reader).unwrap();

    // let file =
    //     File::create("C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example_copy.bin")
    //         .unwrap();
    // let mut writer = BufWriter::new(file);
    // _ = BinFormat::save(&mut writer, &data);
}
