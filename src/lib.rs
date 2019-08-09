#[macro_use]
extern crate tera;

pub mod utils {

    extern crate chrono;
    extern crate regex;

    use chrono::prelude::*;
    use regex::Regex;
    use std::error::Error;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;
    use tera::Context;

    fn save_file(name: String, content: String) {
        let path = Path::new(&name);
        let mut file = match File::create(&path) {
            Err(why) => panic!("Error in creating file {}", why.description()),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("Failed to write to file: {}", why),
            Ok(_) => (),
        };
    }

    pub fn get_input() -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim_end();
                let input = input.to_lowercase();
                return input;
            }
            Err(error) => println!("error: {}", error),
        }
        "".to_string()
    }

    pub fn create_slug(input: String) -> String {
        let re = Regex::new(r"[[:alnum:]]+").unwrap();
        let mut output = String::new();
        for cap in re.captures_iter(&input) {
            let data = &cap[0];
            output.push_str(data);
            output.push_str("-");
        }
        return output.trim_end_matches("-").to_string();
    }

    pub fn create_new_post() {
        println!("Enter the title of the post:");
        let inp = get_input();
        let slug = create_slug(inp.clone());
        let local: DateTime<Local> = Local::now();
        let now = local.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
        // Now, let us work on the tempalte
        let tera = compile_templates!("templates/**/*");
        let mut ctx = Context::new();

        ctx.insert("title", &inp);
        ctx.insert("slug", &slug);
        ctx.insert("date", &now);

        let content = tera
            .render("newpost.md", &ctx)
            .expect("Failed to render template");
        let filename = format!("./posts/{}.md", slug);
        save_file(filename, content);
    }
}
