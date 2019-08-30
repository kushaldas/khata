# Khata: The writing pad

This is the Rust implementation of
[Shonku](https://shonku.readthedocs.io/en/latest/) project.

This is the very early stage.

## To build from the source

```bash
cargo build --release
```

## How to use?

You will the following directory structure.

```text
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

You can make them all as

```bash
mkdir -p pages posts output/{posts,pages,categories}
cp -r assests output/
```

### Create a new blog post

```bash
./khata --new
```

### Build the posts after any change

```bash
./khata
```

To build the updated/new posts.

### To rebuild the whole site

```bash
./khata --rebuild
```

To rebuild the whole site.

### To view the help

```bash
./khata -h
```

To view all the help options.

We have default templates and assets in the git repo.
