# RSS Feed Fetcher

**v0.0.9**: "rust_rss"

*A simple CLI tool for fetching and displaying rss feeds.*
---
## Getting started:

### Setup:

*If needed, install Rust.*

`curl https://sh.rustup.rs -sSf | sh`

1. Fork the repository:

`gh repo fork https://github.com/cursebreakers/rust_rss.git --clone`

2. Compile and run with:

`cargo run`

The program will gather rss content from the urls in **feeds.json** and return them to the terminal. 

By default, only the current date is returned. This can be bypassed to return all available rss by passing the `-a` flag. 

If using `cargo run`, you must add a `--` to the command. 

eg: `cargo run -- -a`

If you are using the binary after installing to your path, you would use

`rust_rss -a`

3. Make tweaks to the program, if desired.

Examples:
- Edit the feeds.json to customize your rss sources.
- Add flags/args to create filter options like ranges of dates, topics, etc.

### Installing to /bin & PATH:

This repository includes a bash script (install.sh) for building and installing the binary and adding the run command to your environment.

Run `./install.sh` from the **/rust_rss** directory.

You may have to run `chmod +x install.sh` to make it executable.

Once installed, you can run the program with `rust_rss`

### Flags and other options:

**The -a/--all flag**

`-a` removes the default filter, fetching any available content from the urls in feeds.json.

**Output**

Piping and routing output to files or other functions supported by default.

Use `rust_rss > todays_news.md` to save the output to a markdown file.

Or `rust_rss | grep "tech"` to filter for tech related news.

---
## WORKING/NEXT

*This is the planning section. These features are (99% likely) not yet implemented, nor may they ever be.*

Interface/menu
- man pages and `-h`/`--help` flag/argument
- display greeting/completion, program/function statuses, present options, etc
- increase readability and appearance/appeal

### Future:

Flag/arg controls
- `-t`/`--today` 
  - *(this is the default setting)*
- `-d x`/`-date x` 
  - *(apply pub date filter to output. x = date)*
- `-s f`/`--save f` 
  - *(Save output to file. Allow format spec with f var. f = md, json or txt. Default to markdown.)*

Feed collections/genre options
- curate and select different sets of feeds

---

## Credits & Acknowledgements:

### Author: 

Esau @ [Cursebreakers LLC](https://cursebreakers.net)

### Built with:

**Crates used:**

| Dependency | Version |
|------------|------|
| reqwest    | 0.11 |
| serde_json | 1.0  |
| serde      | 1.0  |
| tokio      | 1    |
| chrono     | 0.4  |

Honorable mention to [cargo-mommy](https://github.com/Gankra/cargo-mommy), for making programming with Rust much more fun.

---
### License:

This project is to be released under either MIT, Apache 2.0 or both.

---
### Contributions:

Contributions are welcome! Feel free to [submit issues, pull requests, or suggestions for improvement](mailto:hello@cursebreakers.net).


