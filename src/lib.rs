//! # Khata
//!
//! This is the internal library for the static blogging tool.
//!
//!
#[macro_use]
extern crate tera;

pub mod utils {

    extern crate chrono;
    extern crate regex;

    use chrono::prelude::*;
    use regex::Regex;
    use std::error::Error;
    use std::fs;
    use std::fs::DirEntry;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;
    use tera::Context;

    pub fn save_file(name: String, content: String) {
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

    pub fn read_file(name: String) -> String {
        let path = Path::new(&name);
        let mut file = match File::open(&path) {
            Err(why) => panic!("Error in opening file {}", why.description()),
            Ok(file) => file,
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
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
        let yet_to_be_small = output.trim_end_matches("-");
        return yet_to_be_small.to_lowercase();
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
    pub fn ls(dirname: String) -> Vec<String> {
        let mut names: Vec<String> = vec![];
        for entry in fs::read_dir(dirname).unwrap() {
            let value = entry.unwrap();
            let finalpath = value.path();
            let finalname = finalpath.to_str();
            match finalname {
                Some(name) => names.push(name.to_string()),
                None => (),
            }
        }
        names
    }
}

pub mod libkhata {
    extern crate pulldown_cmark;
    use crate::utils::*;
    extern crate chrono;
    extern crate hex;
    extern crate sha2;

    extern crate serde;
    extern crate serde_json;

    use chrono::prelude::*;
    use pulldown_cmark::{html, Options, Parser};
    use serde::{Deserialize, Serialize};
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::str;
    use tera::{Tera, Context};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct PageLink {
        link: String,
        text: String,
    }

    #[derive(Deserialize, Serialize, Debug, Default)]
    pub struct Configuration {
        author: String,
        title: String,
        url: String,
        content_footer: String,
        disqus: String,
        email: String,
        description: String,
        logo: String,
        links: Vec<PageLink>,
        withamp: bool,
    }

    #[derive(Debug, Clone)]
    pub struct Post<'a> {
        title: String,
        slug: String,
        author: String,
        body: String,
        hash: String,
        date: DateTime<Local>,
        sdate: String,
        tags: HashMap<String, String>,
        changed: bool,
        url: String,
        conf: &'a Configuration,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Catpage<'a> {
        cats: HashMap<String, String>,
        conf: &'a Configuration,
    }

    pub fn read_post(filename: String, conf: &Configuration) -> Post {
        let content = read_file(filename);
        let tmp_content = content.clone();
        let lines: Vec<&str> = tmp_content.split("\n").collect();

        let mut title: String = String::from("");
        let mut author: String = String::from("");
        let mut temp_author: String = String::from("");
        let mut date: String = String::from("");
        let mut tagline: String = String::from("");
        let mut slug: String = String::from("");
        let mut finaltags: HashMap<String, String> = HashMap::new();
        let mut dt: DateTime<Local> = Local::now();

        for line in lines {
            // Means we have all the metadata
            if line.starts_with("-->") {
                break;
            } else if line.starts_with(".. title:") {
                // We have the title of the post
                title = String::from(&line[10..]);
            } else if line.starts_with(".. date:") {
                // We have the date of the post
                date = String::from(&line[9..]);
                let d = DateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M:%S%:z").unwrap();
                dt = d.with_timezone(&dt.timezone());
            } else if line.starts_with(".. author:") {
                temp_author = String::from(&line[11..])
            } else if line.starts_with(".. slug:") {
                slug = String::from(&line[9..])
            } else if line.starts_with(".. tags:") {
                let l = &line[9..];
                let trimmed_line = l.trim();
                tagline = String::from(trimmed_line);
                if tagline == "" {
                    tagline = "Uncategorized".to_string();
                }
            }
        }
        // Find the actual author for the post
        // This can be per post or from the configuration file
        if temp_author == String::from("") {
            author = conf.author.clone();
        }

        let tags_temp: Vec<&str> = tagline.split(",").collect();
        for word in tags_temp {
            let trimmped_word = word.trim();
            let temp_word = trimmped_word.to_string();

            finaltags.insert(create_slug(temp_word.clone()), temp_word);
        }
        //let mut tags: Vec<String> = tags_temp.iter().map(|x| x.to_string()).collect();
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&content, options);

        // Write to String buffer.
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Let the current sha256sum of the source file
        let mut hasher = Sha256::new();
        hasher.input(content.as_bytes());
        let result = hasher.result();
        let hashs = hex::encode(&result[..]);

        let post = Post {
            title: title,
            slug: slug.clone(),
            body: html_output,
            hash: hashs,
            date: dt,
            sdate: date,
            tags: finaltags,
            changed: false,
            author: author,
            url: format!("{}posts/{}.html", conf.url.clone(), slug),
            conf: conf,
        };
        post
    }

    pub fn get_conf() -> Configuration {
        let json_str = read_file("conf.json".to_string());
        let conf: Configuration = serde_json::from_str(&json_str).unwrap();
        conf
    }

    fn build_categories(tera: Tera, catpage: Catpage) {
                let mut context = Context::new();
        context.insert("catpage", &catpage);
        let result = tera.render("category-index.html", &context).unwrap();
        println!("{}", result);
    }

    pub fn rebuild() {
        let mut indexlist: Vec<Post> = vec![];
        let mut ps: Vec<Post> = vec![];
        let mut pageyears: HashMap<String, Vec<Post>> = HashMap::new();
        let mut catslinks: HashMap<String, Vec<Post>> = HashMap::new();
        let mut catnames: HashMap<String, String> = HashMap::new();

        let conf = get_conf();
        let post_files = ls("./posts/".to_string());

        for filename in post_files {
            let post = read_post(filename, &conf);
            let postdate = post.date.year().to_string();
            let page_posts = pageyears.get_mut(&postdate);
            match page_posts {
                Some(v) => v.push(post.clone()),
                None => {
                    let temp = vec![post.clone()];
                    pageyears.insert(postdate, temp);
                }
            }
            // Now make it ready for tags (categories)
            let tp = post.clone();
            for (k, v) in &tp.tags {
                catnames.insert(k.clone(), v.clone());
                let key = k.clone();
                let cat_posts = catslinks.get_mut(&key);
                match cat_posts {
                    Some(v) => v.push(post.clone()),
                    None => {
                        let temp = vec![post.clone()];
                        catslinks.insert(key, temp);
                    }
                }
            }
            ps.push(post);
        }

        let catpage = Catpage {
            cats: catnames.clone(),
            conf: &conf,
        };
        //println!("{:?}", catpage);
        let tera = compile_templates!("templates/**/*");

        build_categories(tera, catpage);
    }

}
