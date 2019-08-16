extern crate chrono;
extern crate clap;
extern crate khata;

use clap::{crate_authors, crate_version, App, Arg};
use khata::libkhata::rebuild;
use khata::utils::*;

fn main() {
    let matches = App::new("khata")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Static blogging tool inspired from Shonku.")
        .arg(
            Arg::with_name("new")
                .short("n")
                .long("new")
                .help("Creates a new blog post under posts directory.")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("rebuild")
                .short("r")
                .long("rebuild")
                .help("Rebuilds the whole site")
                .takes_value(false),
        )
        .get_matches();
    if matches.is_present("new") {
        create_new_post();
        return;
    }

    if matches.is_present("rebuild") {
        rebuild(true, true);
    } else {
        rebuild(false, false);
    }


}
