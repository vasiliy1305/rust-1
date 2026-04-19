# rust-1

Workspace из двух crate:

* `ypbank_parser` — библиотека (CSV, TXT, BIN)
* `ypbank_cli` — CLI

## Сборка

```bash
cargo build
```

## Конвертация

### файл → stdout

```bash
cargo run -p ypbank_cli --bin ypbank_converter -- \
  --input file_examples/records_example.csv \
  --in-format csv \
  --out-format txt
```

### файл → файл

```bash
cargo run -p ypbank_cli --bin ypbank_converter -- \
  --input file_examples/records_example.csv \
  --in-format csv \
  --out-format txt \
  --output out.txt
```

### stdin → stdout

```bash
Get-Content .\file_examples\records_example.csv -Raw | \
cargo run -p ypbank_cli --bin ypbank_converter -- \
  --in-format csv \
  --out-format txt
```

## Сравнение

```bash
cargo run -p ypbank_cli --bin ypbank_compare -- \
  --file1 file_examples/records_example.csv \
  --format1 csv \
  --file2 file_examples/records_example.txt \
  --format2 txt
```

## Форматы

`csv`, `txt`, `binary`
 