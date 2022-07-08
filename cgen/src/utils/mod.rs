{%- if crate_type == "bin" -%}
mod cli;
{% endif -%}
mod error;
mod setup;

{% if crate_type == "bin" %}
pub use cli::*;
{% endif -%}
pub use error::*;
pub use setup::*;
