# {{project-name}}
![](https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink)
{% if ci -%}
![](https://github.com/thor314/{{project-name}}/actions/workflows/ci.yml/badge.svg)
<!-- [![crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}}) -->
<!-- [![Documentation](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}}) -->
{% endif -%}

{% if crate_type == "bin" -%}
## Installation
### Cargo
- Install the rust toolchain in order to have cargo installed by following [this](https://www.rust-lang.org/tools/install) guide.
- run `cargo install {{project-name}}`
{% endif -%}

{% if license -%}
## License
Licensed under your option of either:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
{% endif -%}
