# Templates
My "batteries-and-more-included" templates with `cargo-generate`.

## Why bother with templates? 
- Sanity testing is great. Templates like this make it easy to try out new libraries and test features. 
- Save time setting up tools. Tools are great. I want them available with minimal fiddling.
- How do I use $TOOL again? Templates remind me how to use my tools, even when I haven't picked them up recently.

## Current Features:
- CI on pull-requests: verifies lints, tests, and formatting (lint/test over build)
- CI with dependabot to update dependencies
- Automerge dependabot pull requests (this is insecure, but convenient as heck)
- Options for library or binary templates
- A template `clap` CLI tools
- Logging with `log` and `env_logger`
- Error handling with `anyhow` and `this-error`
- Environment variables with `dotenv`
- Input validation with `validator`
- Benching with `criterion` and `iai`
- Test fixtures with `rstest`
- Property testing (fuzzing) with `quickcheck` and `arbitrary`
- Opinionated formatting preferences in `rustfmt.toml`
- Sensible default `.gitignore` settings
- Licensing: permissive dual Unlicense/MIT licensing
- Several optional, common dependencies, that won't be built unless used

## To do:
More templates:
- Fix bin/lib feature
- Async template
- Rocket HTTP server template
- TUI template, similar to: https://github.com/orhun/rust-tui-template
- Rhai-script examples
Options:
- Option to turn off CI, Licensing 

## Usage
```sh
cargo install cargo-generate
cargo generate thor314/cgen --{bin|lib}
```

## Alternatives:
- [rust-github/template](https://github.com/rust-github/template) - a lighter template for `cargo-generate`, with APACHE-MIT dual licensing
