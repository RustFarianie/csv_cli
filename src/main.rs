use structopt::StructOpt;

mod cli;
mod csv_fn;

use cli::*;
use csv_fn::*;

use crate::cli::Read;

fn main() {
    match Cli::from_args().cmd {
		//  cargo run write --separator '|' --record "Country" "Name" "Town" --path "test" 
        Command::Write(Write {
            separator,
            record,
            path
        }) => {

            if let Err(err) = csv_write(separator, record, path) {
                panic!("error running example: {}", err);
            }

        }
        // cargo run read --row-start-number 0 --row-end-number 10 --record-start-number 0 --record-end-number 0 --seprator  '|' --column "room_code" --path "input[2].csv"
        Command::Read(Read {
            row_start_number,
            record_start_number,
            row_end_number,
            record_end_number,
            seprator,
            path,
            column,
            query,
        }) => {
            if let Err(err) = csv_read(
                row_start_number,
                row_end_number,
                record_start_number,
                record_end_number,
                seprator,
                path.as_str(),
                column.as_str(),
                query,
            ) {
                panic!("error running example: {}", err);
            }
        },
		// cargo run add --separator '|' --string-record "Test" "Test2" "Test3" --path "test"
		Command::Add(Add{
			separator,
			string_record,
			path,
		}) => {
			let mut record = Vec::new();
			record.push(string_record);

			if let Err(err) = csv_add_record(separator, record, path) {
				panic!("error running example: {}", err);
			}
		}
    };
}
