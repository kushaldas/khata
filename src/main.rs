use clap::{crate_authors, crate_version, App, Arg};
use khata::libkhata::rebuild;
use khata::utils::create_new_post;

#[cfg(feature = "shadow")]
use shadow_rs::shadow;
#[cfg(feature = "shadow")]
shadow!(build);

fn main() {
    let app = App::new("khata")
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
        );

    #[cfg(feature = "shadow")]
    let app = app.arg(
        Arg::with_name("exe")
            .short("e")
            .long("exe")
            .help("Gives the detials of the khata executable itself.")
            .takes_value(false),
    );
    let matches = app.get_matches();

    #[cfg(feature = "shadow")]
    if matches.is_present("exe") {
        println!("Debug build:{}", shadow_rs::is_debug());
        println!("git_clean: {}", shadow_rs::git_clean());

        println!("Branch: {}", build::BRANCH);
        println!("Short commit: {}", build::SHORT_COMMIT);
        println!("Commit HASH: {}", build::COMMIT_HASH);
        println!("Commit date: {}", build::COMMIT_DATE);
        println!("Commit author: {}", build::COMMIT_AUTHOR);
        println!("Commit email: {}", build::COMMIT_EMAIL);

        println!("Rust Version: {}", build::RUST_VERSION);
        println!("Rust channel: {}", build::RUST_CHANNEL);
        println!("Cargo version: {}", build::CARGO_VERSION);

        println!("Build time: {}", build::BUILD_TIME);
        return;
    }
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
