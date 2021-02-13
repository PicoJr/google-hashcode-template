use clap::{App, Arg};

pub fn get_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Google Hashcode Template")
        .arg(
            Arg::with_name("input")
                .help("input file paths")
                .multiple(true)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dry")
                .help("do not write to disk")
                .long("--dry")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("out")
                .help("output directory")
                .long("--out")
                .required(false)
                .default_value(".")
                .takes_value(true),
        )
}
