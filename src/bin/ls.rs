use std::fs;
use structopt::StructOpt;
use colored::Colorize;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ls",
    version = "0.1.0",
    about = "List information about the FILEs (the current directory by default).
Sort entries alphabetically if none of -cftuvSUX nor --sort is specified.",
)]
struct Opt {
    /// do not ignore entries starting with .
    #[structopt(short, long)]
    all: bool,
}

fn main() {
    let opt : Opt = Opt::from_args();

    println!("{:?}", opt);

    let mut path_vec : Vec<PathBuf> = Vec::new();
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        path_vec.push(path.unwrap().path());
    }

    path_vec.sort();

    for pb in path_vec {
        let is_dir = pb.is_dir();
        let file_name = pb.file_name().unwrap().to_str().unwrap();
        let prefix_dot = file_name.starts_with(".");

        if prefix_dot && !opt.all {
            continue;
        }

        if is_dir {
            print!("{} ", file_name.bright_cyan());
        } else {
            print!("{} ", file_name);
        }
    }
}