{% if tm == "bin" -%}
mod cli;
{% endif -%}
mod error;
mod setup;

{%- if tm == "bin" %}
pub use cli::*;
{%- endif %}
pub use error::*;
pub use setup::*;
