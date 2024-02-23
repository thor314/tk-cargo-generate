<h1 align="center">
  {{project-name}}
</h1>

<p align="center">
  {{description}}
</p>

<div align="center">
  <a href="https://x.com/cryptograthor">
    <img src="https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink" />
    <!-- ![](https://img.shields.io/badge/made_by_cryptograthor-black?style=flat&logo=undertale&logoColor=hotpink) -->
  </a>
  {% if ci -%}
  <a href="https://github.com/thor314/{{project-name}}/actions">
    <!-- ![](https://github.com/thor314/{{project-name}}/actions/workflows/ci.yml/badge.svg) -->
    <img src="https://github.com/thor314/{{project-name}}/actions/workflows/ci.yml/badge.svg" />
  </a>
  <!-- [![crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}}) -->
  <!-- [![Documentation](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}}) -->
  {% endif -%}
</div>

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

{% if server -%}
## Deploy
```sh
# run locally
cargo shuttle run
# deploy
cargo shuttle project start # only needed the first time
cargo shuttle deploy
```
{% endif -%}

## Project created with flags:
- project-name: {{project-name}}
- description:  {{description}}
- authors:      {{authors}}
- crate_name:   {{crate_name}}
- crate_type:   {{crate_type}}
- os-arch:      {{os-arch}}
- username:     {{username}}
- within_cargo: {{within_cargo_project}}
- is_init:      {{is_init}}
- now:          {{ "now" | date: "%Y-%m-%d" }}
- bin or lib:  {% if crate_type == "bin" %} bin {% else %} lib {% endif %}
- advanced:    {% if advanced %} advanced {% endif %}
- cli:         {% if cli %} cli {% endif %}
- license:     {% if license %} license {% endif %}
- ci:          {% if ci %} ci {% endif %}
- itests:      {% if itests %} itests {% endif %}
- benches:     {% if benches %} benches {% endif %}
- async:       {% if async %} async {% endif %}
- server:      {% if server %} server {% endif %}
