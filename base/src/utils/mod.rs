{%- if crate_type == "bin" -%}
mod cli;
{% endif -%}
mod error;
mod setup;

{% if crate_type == "bin" %}
// pub(crate) use cli::*;
{% endif -%}
pub use error::*;
pub(crate) use setup::*;
