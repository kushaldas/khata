# Khata: The writing pad

This is the Rust implementation of
[Shonku](https://shonku.readthedocs.io/en/latest/) project.

This is the very early stage.

### To build from the source

```bash
cargo build --release
```

### How to use?

You will the following directory structure.

```
|-> templates
|-> pages
|-> posts
|-> output
    |
    |-> posts
    |-> pages
    |-> categories
    |-> assets
```

```bash
./khata --new
```

Will help you to create a new post.

```bash
./khata
```

To build the updated/new posts.

```bash
./khata --rebuild
```

To rebuild the whole site.

```bash
./khata -h
```

To view all the help options.

We have default templates and assets in the git repo.
