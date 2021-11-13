use csv::WriterBuilder;
use csv::{self, ReaderBuilder};
use std::error::Error;
use std::fs::OpenOptions;

pub fn csv_read(
    row_start_number: usize,
    row_end_number: usize,
    record_start_number: usize,
    record_end_number: usize,
    separator: char,
    path: &str,
    column: &str,
    query: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let separator = separator as u8;
    let mut rdr = ReaderBuilder::new().delimiter(separator).from_path(path)?;
    let header = rdr.byte_headers()?;
    let mut column_index: usize = 0;

    if !&query.is_none() {
        column_index = header.iter().position(|r| r == column.as_bytes()).unwrap();
    }

    for (i, result) in rdr.records().enumerate() {
        if record_start_number <= i && record_end_number >= i {
            let record = result?;

            for elem in row_start_number..row_end_number {
                if !query.is_none() {
                    if record.get(column_index).unwrap() == query.as_ref().unwrap() {
                        print!("{} ", &record[elem]);
                        add_new_line(elem, row_end_number)
                    }
                } else {
                    print!("{} ", &record[elem]);
                    add_new_line(elem, row_end_number)
                }
            }
        }
    }

    Ok(())
}

fn add_new_line(elem: usize, row_end_number: usize) {
    if elem == row_end_number - 1 {
        print!("\n")
    }
}

pub fn csv_write(separator: char, record: Vec<String>, path: String) -> Result<(), Box<dyn Error>> {
    let separator = separator as u8;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(false)
        .open(format!("{}.csv", path))?;

    let mut wtr = WriterBuilder::new().delimiter(separator).from_writer(file);

    wtr.write_record(record)?;

    wtr.flush()?;

    Ok(())
}

pub fn csv_add_record(
    separator: char,
    record: Vec<Vec<String>>,
    path: String,
) -> Result<(), Box<dyn Error>> {
    let separator = separator as u8;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(format!("{}.csv", path))?;

    let mut wtr = WriterBuilder::new().delimiter(separator).from_writer(file);

    for srialize_record in record {
        wtr.serialize(srialize_record)?;
    }

    wtr.flush()?;

    Ok(())
}
