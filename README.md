# rust-1
---
C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.bin
C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.csv
C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.txt
---

./parser --input C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.bin --input-format binary --output-format binary > C:/Users/Admin/Desktop/RUST/rust-1/file_examples/COPY/records_example_copy.bin

./parser --input C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.csv --input-format csv --output-format csv > C:/Users/Admin/Desktop/RUST/rust-1/file_examples/COPY/records_example_copy.csv 

./parser --input C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.txt --input-format txt  --output-format txt > C:/Users/Admin/Desktop/RUST/rust-1/file_examples/COPY/records_example_copy.txt 
---

./comparer --file1 C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.csv --format1 csv --file2 C:/Users/Admin/Desktop/RUST/rust-1/file_examples/COPY/records_example_copy.csv  --format2 csv

./comparer --file1 C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.bin --format1 binary --file2 C:/Users/Admin/Desktop/RUST/rust-1/file_examples/COPY/records_example_copy.bin  --format2 binary

./comparer --file1 C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.txy --format1 txt --file2 C:/Users/Admin/Desktop/RUST/rust-1/file_examples/COPY/records_example_copy.txt  --format2 txt