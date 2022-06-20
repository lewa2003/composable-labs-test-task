use clap::{App, Arg};
use interpreter_app::interpret;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("interpreter")
        .version("1.0")
        .author("Artem S. <sertem96@gmail.com>")
        .about(
            "Interpret ByteCode instructions",
        )
        .arg(
            Arg::with_name("FILE")
                .short('f')
                .long("file")
                .help("Sets file with ByteCode instructions")
                .takes_value(true),
        )
        .get_matches();

    let file = matches
        .value_of("FILE")
        .expect("File must be specified");
    let result = interpret(file)?;
    println!("{}", result.unwrap());
    Ok(())
}
