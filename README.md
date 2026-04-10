# rust-1

надо было делать все через макросы

ypbank_converter \
  --input <input_file> \
  --input-format <format> \
  --output-format <format> \
  > output_file.txt

  ypbank_compare --file1 records_example.bin --format1 binary --file2 records_example.csv --format2 csv
# Output: The transaction records in 'records_example.bin' and 'records_example.csv' are identical.


C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.bin
C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.csv
C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.txt


./parser   --input C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.bin  --input-format binary --output C:/Users/Admin/Desktop/RUST/rust-1/file_examples/COPY/records_example_copy.bin  --output-format binary

./parser   --input C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.csv  --input-format csv --output C:/Users/Admin/Desktop/RUST/rust-1/file_examples/COPY/records_example_copy.csv  --output-format csv

./parser   --input C:/Users/Admin/Desktop/RUST/rust-1/file_examples/records_example.txt  --input-format txt --output C:/Users/Admin/Desktop/RUST/rust-1/file_examples/COPY/records_example_copy.txt  --output-format txt