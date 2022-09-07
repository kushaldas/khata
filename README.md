# Khata: The writing pad

This is the Rust implementation of
[Shonku](https://shonku.readthedocs.io/en/latest/) project.

This is the very early stage.

## To build from the source for normal systems

```bash
cargo build --features shadow --release
```

The `shadow` feature will enable `-e/--exe` to get details about the executable.
But, in case you want to run it under [WASI](https://wasi.dev/), then build it using the following command.

```bash
cargo build --target wasm32-wasi --release
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
cp -r assets output/
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

### To view which binary you are using (when you enable shadow feature)

```bash
./khata -e
```


