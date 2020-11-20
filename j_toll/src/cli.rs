use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(parse(from_os_str))]
    pub input_file_path: std::path::PathBuf,
}

pub fn parse() -> Args {
    Args::from_args()
}
