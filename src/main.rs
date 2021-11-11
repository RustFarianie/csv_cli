use csv::{self, ReaderBuilder};
use std::error::Error;
use std::process;

fn csv(
    row_start_number: usize,
    row_end_number: usize,
    record_start_number: usize,
    record_end_number: usize,
    seprator: char,
    path: &str,
) -> Result<(), Box<dyn Error>> {
    let seprator = seprator as u8;
    let mut rdr = ReaderBuilder::new().delimiter(seprator).from_path(path)?;

    for (i, result) in rdr.records().enumerate() {
        if record_start_number <= i && record_end_number >= i {
            let record = result?;

            println!("Row {}", i);
            for elem in row_start_number..row_end_number {
                println!("{:?}", &record[elem]);
            }
        }
    }

    Ok(())
}

fn main() {
    let row_start_number = 0;
    let row_end_number = 5;

    let record_start_number = 3;
    let record_end_number = 5;

    let seprator = '|';
    let path = "input[2].csv";

    if let Err(err) = csv(
        row_start_number,
        row_end_number,
        record_start_number,
        record_end_number,
        seprator,
        path,
    ) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}