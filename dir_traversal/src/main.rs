use clap::{App, Arg};
use linecount::count_lines;
use std::fs::File;
use walkdir::{DirEntry, WalkDir};

fn main() {
    let matches = App::new("myapp")
        .version("1.0")
        .author("Artem S. <sertem96@gmail.com>")
        .about(
            "Traverse a directory and prints the number of lines in files with a given extension",
        )
        .arg(
            Arg::with_name("EXTENSION")
                .short('e')
                .long("extension")
                .help("Sets file extension")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DIRECTORY")
                .short('d')
                .long("directory")
                .help("Sets directory")
                .takes_value(true),
        )
        .get_matches();

    let extension = matches
        .value_of("EXTENSION")
        .expect("Extension must be specified");

    let directory = matches
        .value_of("DIRECTORY")
        .expect("Directory must be specified")
        .to_owned();

    for entry in WalkDir::new(directory) {
        let path = entry
            .as_ref()
            .expect("Unable to open directory entry")
            .path();
        if let Some(ext) = path.extension() {
            if extension == ext {
                let lines: usize = count_lines(File::open(path).expect("Unable to open file"))
                    .expect("Unable to count lines");
                println!(
                    "{}: {}",
                    path.to_str().expect("Unable to convert path to str"),
                    lines
                );
            }
        }
    }
}