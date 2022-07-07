# Templates
My "batteries-and-more-included" templates with `cargo-generate`.

## Why bother with templates? Two reasons:
- Save time, keep tools organized and close to hand. Countless hours have gone into reducing developer friction.
  In production, this is called "reproducible builds". In practice, this means making it easy to set up a playground
  with all your tools.
- Don't forget (how) to use your tools. A good template puts yours tools in reasonable, unobtrusive places, with short
  reminders of how to use them, so that you don't have to page through documentation to grab a tool you haven't used in awhile. 

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
