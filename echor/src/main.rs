use clap::{Command, Arg};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..)
                // .action(ArgAction::Set),
        )
        .arg(
            Arg::new("omit_newline")
            .short('n')
            .num_args(0)
            .help("Do not print newline")
        )
        .get_matches();

    println!("{:#?}", matches);
}
