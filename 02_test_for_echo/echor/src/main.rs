use clap::{Command, Arg};

fn main() {
    // println!(std::env::args()); // This will not work

    // println!("{}", std::env::args()); // This will not work

    // println!("{:?}", std::env::args());

    let matches = Command::new("echor")
        .author("Me, vhgcuong95@gmail.com")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
        )
        .get_matches();

    println!("{:#?}", matches);
}
