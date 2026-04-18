# rust-1


Сборка
cargo build
---
Запуск конвертера
rust-1> cargo run --manifest-path parser/Cargo.toml --bin ypbank_converter -- --input file_examples/records_example.csv --input-format csv --output-format csv

---
Запуск сравнения форматов
cargo run --manifest-path parser/Cargo.toml --bin ypbank_compare -- --file1 file_examples/records_example.csv --format1 csv --file2 file_examples/records_example.csv --format2 csv

---
C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.bin
C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.csv
C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.txt
---

Проверки

csv -> txt
cmd /c "cargo run --manifest-path .\parser\Cargo.toml --quiet --bin ypbank_converter -- --input .\file_examples\records_example.csv --input-format csv --output-format txt > .\file_examples\temp\out.txt"

cargo run --manifest-path .\parser\Cargo.toml --quiet --bin ypbank_compare -- `
  --file1 .\file_examples\records_example.txt `
  --format1 txt `
  --file2 .\file_examples\temp\out.txt `
  --format2 txt


csv -> csv
cmd /c "cargo run --manifest-path .\parser\Cargo.toml --quiet --bin ypbank_converter -- --input .\file_examples\records_example.csv --input-format csv --output-format csv > .\file_examples\temp\out.csv"

cargo run --manifest-path .\parser\Cargo.toml --quiet --bin ypbank_compare -- `
  --file1 .\file_examples\records_example.csv `
  --format1 csv `
  --file2 .\file_examples\temp\out.csv `
  --format2 csv


txt -> txt
cmd /c "cargo run --manifest-path .\parser\Cargo.toml --quiet --bin ypbank_converter -- --input .\file_examples\records_example.csv --input-format csv --output-format csv > .\file_examples\temp\out.csv"

cargo run --manifest-path .\parser\Cargo.toml --quiet --bin ypbank_compare -- `
  --file1 .\file_examples\records_example.csv `
  --format1 csv `
  --file2 .\file_examples\temp\out.csv `
  --format2 csv


txt -> csv
cmd /c "cargo run --manifest-path .\parser\Cargo.toml --quiet --bin ypbank_converter -- --input .\file_examples\records_example.txt --input-format txt --output-format csv > .\file_examples\temp\out.csv"

cargo run --manifest-path .\parser\Cargo.toml --quiet --bin ypbank_compare -- `
  --file1 .\file_examples\records_example.csv `
  --format1 csv `
  --file2 .\file_examples\temp\out.csv `
  --format2 csv