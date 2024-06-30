use clap::{Command, Arg, ArgAction};

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
                .action(ArgAction::Append)
                .num_args(1..)
        )
        .arg(
            Arg::new("omit_newline")
            .short('n')
            .action(ArgAction::SetTrue)
            .help("Do not print newline")
        )
        .get_matches();

    let text = matches.get_many::<String>("text").unwrap();
    let omit_newline = matches.get_flag("omit_newline");

    println!("{:#?}{}", text, if omit_newline { "" } else { "\n" });
    println!("{:#?}", omit_newline);
}
