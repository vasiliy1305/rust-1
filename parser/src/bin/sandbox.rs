use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

use parser::{Transaction, LoadData, SaveData, csv_format::CsvFormat};

fn main(){
    let mut file = File::open("C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.csv").unwrap();
    let mut reader = BufReader::new(file);
    let data = CsvFormat::load(reader).unwrap();

    let mut file = File::create("C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example_copy.csv").unwrap();
    let mut writer = BufWriter::new(file);
    CsvFormat::save(&mut writer, &data);
}