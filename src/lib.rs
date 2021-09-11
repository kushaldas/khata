//! # Khata
//!
//! This is the internal library for the static blogging tool.
//!
//!
extern crate tera;

pub mod utils {

    extern crate chrono;
    extern crate regex;

    use chrono::prelude::*;
    use regex::Regex;
    use std::fs;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;
    use std::process::Command;
    use tera::Context;

    pub fn save_file(name: String, content: String) {
        let path = Path::new(&name);
        let mut file = match File::create(&path) {
            Err(why) => panic!("Error in creating {:?} {}", path, why),
            Ok(file) => file,
        };

        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("Failed to write to {:?} : {}", path, why),
            Ok(_) => (),
        };
    }

    pub fn save_rss(name: String, content: String) {
        let path = Path::new(&name);
        let mut file = match File::create(&path) {
            Err(why) => panic!("Error in creating RSS feed file {:?} {}", path, why),
            Ok(file) => file,
        };

        match file.write_all(br#"<?xml version="1.0" encoding="UTF-8"?>"#) {
            Err(_) => (),
            Ok(_) => (),
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(content.as_bytes()) {
            Err(why) => panic!("Failed to write to file: {}", why),
            Ok(_) => (),
        };

        match file.write_all(b"\n") {
            Err(_) => (),
            Ok(_) => (),
        };
    }

    pub fn read_file(name: String) -> String {
        let path = Path::new(&name);
        let mut file = match File::open(&path) {
            Err(why) => panic!("Error in opening {:?} {}", path, why),
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
        let tera = tera::Tera::new("templates/**/*").unwrap();
        let mut ctx = Context::new();

        ctx.insert("title", &inp);
        ctx.insert("slug", &slug);
        ctx.insert("date", &now);

        let content = tera
            .render("newpost.md", &ctx)
            .expect("Failed to render template");
        let filename = format!("./posts/{}.md", slug);
        save_file(filename.clone(), content);
        println!("Created new post: {}", filename);
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

    // Syncs any new blog post source
    pub fn sync_posts() {
        let _output = Command::new("/usr/bin/rsync")
            .arg("-avz")
            .arg("./posts/")
            .arg("./output/posts/")
            .output()
            .unwrap();
        //println!("{:#?}", output);
    }
}

pub mod libkhata {
    extern crate pulldown_cmark;
    use crate::utils::*;
    extern crate chrono;
    extern crate hex;
    extern crate rss;
    extern crate sha2;

    extern crate serde;
    extern crate serde_json;

    use chrono::prelude::*;
    use pulldown_cmark::{html, Options, Parser};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use sha2::{Digest, Sha256};
    use std::cmp::Reverse;
    use std::collections::HashMap;
    use std::path::Path;
    use std::str;
    use tera::{Context, Tera};

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
        filename: String,
        author: String,
        body: String,
        excerpt: String,
        hash: String,
        date: DateTime<Local>,
        sdate: String,
        tags: HashMap<String, String>,
        changed: bool,
        url: String,
        conf: &'a Configuration,
    }

    // This structure is only for template rendering
    #[derive(Debug, Clone, Serialize)]
    pub struct SerialPost<'a> {
        title: String,
        slug: String,
        filename: String,
        author: String,
        body: String,
        excerpt: String,
        hash: String,
        sdate: String,
        tags: HashMap<String, String>,
        changed: bool,
        url: String,
        conf: &'a Configuration,
    }

    impl SerialPost<'_> {
        pub fn new<'a>(post: &Post<'a>) -> SerialPost<'a> {
            SerialPost {
                title: post.title.clone(),
                slug: post.slug.clone(),
                body: post.body.clone(),
                excerpt: post.excerpt.clone(),
                filename: post.filename.clone(),
                hash: post.hash.clone(),
                sdate: post.sdate.clone(),
                tags: post.tags.clone(),
                changed: post.changed.clone(),
                author: post.author.clone(),
                url: post.url.clone(),
                conf: post.conf,
            }
        }
    }
    #[derive(Debug, Clone, Serialize)]
    pub struct Archiveyear<'a> {
        year: String,
        posts: Vec<SerialPost<'a>>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Catpage<'a> {
        cats: HashMap<String, String>,
        conf: &'a Configuration,
    }

    pub fn read_post(filename: String, conf: &Configuration) -> Post {
        let content = read_file(filename.clone());
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

            let slug = create_slug(temp_word.clone());
            // No empty tags
            if slug == "" {
                continue;
            }

            finaltags.insert(slug, temp_word);
        }
        //let mut tags: Vec<String> = tags_temp.iter().map(|x| x.to_string()).collect();
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&content, options).map(|event| match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image(_link_type, dest, title)) => {
                // If the image has a relative path, convert into absolute URL.
                if dest.starts_with("/") {
                    let mut chars = dest.chars();
                    chars.next();
                    let news = format!("{}{}", conf.url, chars.as_str());
                    pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image(
                        _link_type,
                        pulldown_cmark::CowStr::Boxed(Box::from(&news[..])),
                        title,
                    ))
                } else {
                    pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image(
                        _link_type, dest, title,
                    ))
                }
            }
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link(_link_type, dest, title)) => {
                // If the link has a relative path, convert into absolute URL.
                if dest.starts_with("/") {
                    let mut chars = dest.chars();
                    chars.next();
                    let news = format!("{}{}", conf.url, chars.as_str());
                    pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link(
                        _link_type,
                        pulldown_cmark::CowStr::Boxed(Box::from(&news[..])),
                        title,
                    ))
                } else {
                    pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link(_link_type, dest, title))
                }
            }
            _ => event,
        });

        // Write to String buffer.
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        // Let the current sha256sum of the source file
        let mut hasher = Sha256::new();
        hasher.input(content.as_bytes());
        let result = hasher.result();
        let hashs = hex::encode(&result[..]);

        // Find excerpt <!-- excerpt -->
        let excerpt_index = html_output.find("<!-- excerpt -->").unwrap_or(0);
        let excerpt: String = html_output.drain(..excerpt_index).collect();

        let post = Post {
            title,
            slug: slug.clone(),
            body: html_output,
            excerpt,
            filename,
            hash: hashs,
            date: dt,
            sdate: date,
            tags: finaltags,
            changed: false,
            author,
            url: format!("{}posts/{}.html", conf.url.clone(), slug),
            conf,
        };
        post
    }

    fn create_fdb() {
        let filename = ".scrrdkdss.json";
        let path = Path::new(&filename);
        if path.exists() == false {
            let mut map: HashMap<String, String> = HashMap::new();
            map.insert("file".to_string(), "filehash".to_string());
            let result = json!(map);
            save_file(filename.to_string(), result.to_string());
        }
    }

    fn get_fdb() -> HashMap<String, String> {
        let filename = ".scrrdkdss.json";
        let content = read_file(filename.to_string());
        let result: HashMap<String, String> = serde_json::from_str(content.as_str()).unwrap();
        result
    }

    fn save_fdb(map: HashMap<String, String>) {
        let filename = ".scrrdkdss.json";
        let result = json!(map);
        save_file(filename.to_string(), result.to_string());
    }

    pub fn get_conf() -> Configuration {
        let json_str = read_file("conf.json".to_string());
        let conf: Configuration = serde_json::from_str(&json_str).unwrap();
        conf
    }

    fn build_categories(tera: &Tera, catpage: Catpage) {
        let mut context = Context::new();
        context.insert("catpage", &catpage);
        context.insert("conf", &catpage.conf);
        let result = tera.render("category-index.html", &context).unwrap();
        let filename = "./output/categories/index.html".to_string();
        save_file(filename, result);
    }

    // Check if the indexfile exists on disk or not
    fn check_index(indexname: String, index: i32) -> bool {
        let name = match &indexname[..] {
            "index" => format!("./output/{}-{}.html", indexname, index),
            _ => format!("./output/categories/{}-{}.html", indexname, index),
        };
        let path = Path::new(&name);
        path.exists()
    }

    // Creates index files a type of indexname.
    // `index` is a valid indexname.
    fn create_index_files(tera: &Tera, mut lps: Vec<Post>, indexname: &str) {
        let posts_in_each_index = 10;
        let mut prev: i32;
        let mut next: i32;
        let mut index: i32 = 1;
        let mut index_page_flag = false;
        let mut num = 0;
        // length of the full list
        let length = (lps.len() as i32).into();
        // We start from the oldest post
        // That is why we are sorting here.
        lps.sort_by_key(|v| v.date);
        let mut sort_index: Vec<Post> = Vec::new();

        for post in lps {
            if post.changed == true {
                index_page_flag = true;
            }
            sort_index.push(post.clone());
            num = num + 1;

            // For each 10 posts, we create a new index page
            if num == posts_in_each_index {
                if check_index(String::from(indexname), index) == false {
                    index_page_flag = true;
                }

                // Only changed indexes will get rebuild
                if index_page_flag == true {
                    index_page_flag = false;
                    sort_index.sort_by_key(|v| Reverse(v.date));
                    //let lps: Vec<SerialPost> =
                    //    sort_index.iter().map(|p| SerialPost::new(p)).collect();
                    if index == 1 {
                        prev = 0;
                    } else {
                        prev = (index - 1) as i32;
                    }

                    // I don't remmeber the logic here.
                    // TODO: Add some comment to explain the logic please.
                    if (index * posts_in_each_index) < length
                        && (length - index * posts_in_each_index) > posts_in_each_index
                    {
                        next = ((index + 1) as i32).into();
                    } else if (index * posts_in_each_index) == length {
                        next = -1;
                    } else {
                        next = 0;
                    }
                    // TODO: call build_index
                    let lps: Vec<SerialPost> =
                        sort_index.iter().map(|p| SerialPost::new(p)).collect();
                    build_index(tera, lps, index, prev, next, indexname, sort_index[0].conf);
                }

                sort_index = Vec::new();
                index = index + 1;
                num = 0;
            }
        }
        if sort_index.len() > 0 {
            sort_index.sort_by_key(|v| Reverse(v.date));
            let lps: Vec<SerialPost> = sort_index.iter().map(|p| SerialPost::new(p)).collect();
            build_index(tera, lps, 0, index - 1, -1, indexname, sort_index[0].conf);
        }
    }

    fn build_index(
        tera: &Tera,
        pss: Vec<SerialPost>,
        index: i32,
        pre: i32,
        next: i32,
        indexname: &str,
        conf: &Configuration,
    ) {
        let result: String;
        let filename: String;
        let mut context = Context::new();
        context.insert("posts", &pss);
        context.insert("slug", indexname);
        context.insert("conf", conf);
        if pre != 0 {
            context.insert("PreviousF", &true);
            context.insert("Previous", &pre)
        } else {
            context.insert("PreviousF", &false);
        }

        if next > 0 {
            context.insert("NextF", &true);
            context.insert("Next", &next);
        } else if next == -1 {
            context.insert("NextF", &false);
        } else {
            context.insert("NextF", &true);
            context.insert("Next", &next);
        }
        if next == 0 {
            context.insert("NextLast", &true);
        } else {
            context.insert("NextLast", &false);
        }
        if indexname == "index" {
            context.insert("Main", &true);
        } else {
            context.insert("Main", &false);
        }

        if indexname == "index" {
            result = tera.render("index.html", &context).unwrap();
        } else {
            result = tera.render("cat-index.html", &context).unwrap();
        }

        if next == -1 {
            if indexname == "index" {
                filename = format!("./output/{}.html", indexname);
            } else {
                filename = format!("./output/categories/{}.html", indexname);
            }
        } else {
            if indexname == "index" {
                filename = format!("./output/{}-{}.html", indexname, index);
            } else {
                filename = format!("./output/categories/{}-{}.html", indexname, index);
            }
        }
        save_file(filename, result);
    }

    fn create_archive(tera: &Tera, pageyears: HashMap<String, Vec<Post>>, conf: &Configuration) {
        let mut context = Context::new();
        let mut result;
        let mut keys: Vec<String> = Vec::new();
        let mut years: Vec<Archiveyear> = vec![];
        for key in pageyears.keys() {
            keys.push(String::from(key));
        }
        keys.sort();
        keys.reverse();

        // Now for each year, add a sorted Vec
        for key in keys {
            let p = pageyears.get(&key).unwrap();
            let mut posts = p.clone();
            posts.sort_by_key(|v| Reverse(v.date));
            let lps: Vec<SerialPost> = posts.iter().map(|p| SerialPost::new(p)).collect();
            let a = Archiveyear {
                year: String::from(key),
                posts: lps.clone(),
            };
            years.push(a);
        }
        context.insert("archives", &years);
        context.insert("conf", &conf);

        // Now render and save archive.html
        result = tera.render("archive.html", &context).unwrap();
        let filename = "./output/archive.html".to_string();
        save_file(filename, result);

        // Now generate each year's posts.
        for ar in years {
            context = Context::new();
            context.insert("conf", &conf);
            context.insert("ar", &ar);

            result = tera.render("year.html", &context).unwrap();
            let filename = format!("./output/{}.html", ar.year);
            save_file(filename, result);
        }
    }

    fn build_post(tera: &Tera, post: &Post, ptype: String) {
        let mut filename = "".to_string();
        let mut result: String = "".to_string();
        let mut context = Context::new();
        // This struct can be passed to the template.
        let sp = SerialPost {
            title: post.title.clone(),
            slug: post.slug.clone(),
            body: post.body.clone(),
            excerpt: post.excerpt.clone(),
            filename: post.filename.clone(),
            hash: post.hash.clone(),
            sdate: post.sdate.clone(),
            tags: post.tags.clone(),
            changed: false,
            author: post.author.clone(),
            url: post.url.clone(),
            conf: post.conf,
        };
        context.insert("pdata", &sp);
        context.insert("conf", &sp.conf);
        match &ptype[..] {
            "post" => {
                result = tera.render("post.html", &context).unwrap();
                filename = format!("./output/posts/{}.html", post.slug);
            }
            "page" => {
                result = tera.render("page.html", &context).unwrap();
                filename = format!("./output/pages/{}.html", post.slug);
            }
            _ => (), // TODO: Make it for idiomatic
        }
        save_file(filename, result);
    }

    pub fn rebuild(rebuildall: bool, rbi: bool) {
        let mut rebuild_index = rbi;
        //let indexlist: Vec<Post>;
        let mut ps: Vec<Post> = vec![];
        let mut pageyears: HashMap<String, Vec<Post>> = HashMap::new();
        let mut catslinks: HashMap<String, Vec<Post>> = HashMap::new();
        let mut catnames: HashMap<String, String> = HashMap::new();
        let mut cat_needs_build: HashMap<String, bool> = HashMap::new();

        let conf = get_conf();
        create_fdb();
        let mut fdb = get_fdb();
        let post_files = ls("./posts/".to_string());
        let tera = tera::Tera::new("templates/**/*").unwrap();

        for filename in post_files {
            if &filename.ends_with(".md") != &true {
                continue;
            }
            let mut post = read_post(filename.clone(), &conf);
            let postdate = post.date.year().to_string();
            let page_posts = pageyears.get_mut(&postdate);
            match page_posts {
                Some(v) => v.push(post.clone()),
                None => {
                    let temp = vec![post.clone()];
                    pageyears.insert(postdate, temp);
                }
            }

            let hash: String = match fdb.get(&post.filename) {
                Some(val) => val.clone(),
                None => "".to_string(),
            };

            // TODO: check for hashes here.
            if rebuildall == true || post.hash != hash {
                println!("Building post: {}", filename);
                build_post(&tera, &post, "post".to_string());
                rebuild_index = true;
                post.changed = true;

                // update fdb
                fdb.insert(post.filename.clone(), post.hash.clone());

                // The following tags need rebuild
                for (key, _value) in &post.tags {
                    cat_needs_build.insert(String::from(key), true);
                }
            }
            // Now make it ready for tags (categories)
            let tp = post.clone();
            for (k, v) in &tp.tags {
                catnames.insert(k.clone(), v.clone());
                let key = k.clone();
                let cat_posts = catslinks.get_mut(&key);
                match cat_posts {
                    Some(v) => v.push(tp.clone()),
                    None => {
                        let temp = vec![tp.clone()];
                        catslinks.insert(key, temp);
                    }
                }
            }

            ps.push(post);
        }

        // Now rebuild the pages if required
        let page_files = ls("./pages/".to_string());

        for filename in page_files {
            if &filename.ends_with(".md") != &true {
                continue;
            }
            let post = read_post(filename.clone(), &conf);
            let hash: String = match fdb.get(&post.filename) {
                Some(val) => val.clone(),
                None => "".to_string(),
            };
            if rebuildall == true || post.hash != hash {
                println!("Building page: {}", filename);
                build_post(&tera, &post, "page".to_string());

                // update fdb
                fdb.insert(post.filename.clone(), post.hash.clone());
            }
        }

        save_fdb(fdb);

        let catpage = Catpage {
            cats: catnames.clone(),
            conf: &conf,
        };

        build_categories(&tera, catpage);

        // Now rebuild the category pages as required
        for (key, _) in &cat_needs_build {
            let final_lps: Vec<Post>;
            let localposts = catslinks.get(key).unwrap();
            let mut lps = localposts.clone();
            lps.sort_by_key(|v| Reverse(v.date));
            // let lps_template: Vec<SerialPost> = lps.iter().map(|p| SerialPost::new(p)).collect();
            // Let us create template for the category `key`.
            create_index_files(&tera, lps.clone(), key);

            if lps.len() >= 10 {
                final_lps = lps[..10].to_vec();
            } else {
                final_lps = lps;
            }
            // Now build the feed for that tag
            build_feeds(final_lps, &key, &conf, rebuildall);
        }

        create_archive(&tera, pageyears.clone(), &conf);

        create_index_files(&tera, ps.clone(), "index");
        if rebuild_index == true {
            // Time to check for any change in 10 posts at max and rebuild rss feed if required.

            let mut lps = ps.clone();
            lps.sort_by_key(|v| Reverse(v.date));
            let final_lps: Vec<Post>;
            if lps.len() >= 10 {
                final_lps = lps[..10].to_vec();
            } else {
                final_lps = lps;
            }
            build_feeds(final_lps, "cmain", &conf, rebuildall);
        }

        // Now sync the .md files
        sync_posts();
    }

    // Build the RSS feeds.
    // Use the time from the post normally
    // Use updated time only when it is a rebuild of the whole site
    fn build_feeds(lps: Vec<Post>, name: &str, conf: &Configuration, _rebuild: bool) {
        let filename = if name == "cmain" {
            "./output/rss.xml".to_string()
        } else {
            format!("./output/categories/{}.xml", name)
        };

        let mut items: Vec<rss::Item> = Vec::new();
        for post in lps {
            // date is now only for rebuilding the whole
            // site.
            let date = post.date.format("%a, %d %b %Y %T %z");

            let mut guid = rss::Guid::default();
            guid.set_value(post.url.clone());

            if post.changed == true {
                let item = rss::ItemBuilder::default()
                    .title(post.title.clone())
                    .link(post.url.clone())
                    .guid(Some(guid))
                    .pub_date(format!("{}", date))
                    .description(post.body.clone())
                    .build();
                match item {
                    Ok(i) => items.push(i),
                    Err(msg) => println!("{}", msg),
                }
            } else {
                let item = rss::ItemBuilder::default()
                    .title(post.title.clone())
                    .link(post.url.clone())
                    .guid(Some(guid))
                    .pub_date(format!("{}", date))
                    .description(post.body.clone())
                    .build();
                match item {
                    Ok(i) => items.push(i),
                    Err(msg) => println!("{}", msg),
                }
            }
        }

        let channel = rss::ChannelBuilder::default()
            .title(conf.title.clone())
            .link(conf.url.clone())
            .description(conf.title.clone())
            .items(items)
            .build();
        match channel {
            Ok(right) => {
                save_rss(filename, right.to_string());
            }
            Err(msg) => println!("{}", msg),
        }
    }
}
