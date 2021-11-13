use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)] pub struct Read{
    #[structopt()]
    #[structopt(long)]
    pub row_start_number: usize,
    #[structopt(long)]
    pub row_end_number: usize,
    #[structopt(long)]
    pub record_start_number: usize,
    #[structopt(long)]
    pub record_end_number: usize,
    #[structopt(long)]
    pub seprator: char,
    #[structopt(long)]
    pub path: String,
    #[structopt(long)]
    pub column: String,
    #[structopt(long)]
    pub query: Option<String>,
}

#[derive(StructOpt, Debug)] pub struct Write{
    #[structopt()]
    #[structopt(long)]
    pub separator: char,
    #[structopt(long)]
    pub record: Vec<String>,
    #[structopt(long)]
    pub path: String,
}

#[derive(StructOpt, Debug)] pub struct Add{
    #[structopt()]
    #[structopt(long)]
    pub separator: char,
    #[structopt(long)]
    pub string_record: Vec<String>,
    #[structopt(long)]
    pub path: String,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Read(Read),
    Write(Write),
    Add(Add)
}