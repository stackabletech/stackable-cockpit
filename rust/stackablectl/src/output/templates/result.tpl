{% if pre_hints | length != 0 -%}
{% for pre_hint in pre_hints -%}
{{ pre_hint }}
{% endfor %}

{% endif -%}
{%- if output | length != 0 %}{{ output }}{% endif -%}
{% if command_hints | length != 0 %}

{% for command_hint in command_hints -%}
{{ command_hint }}
{% endfor -%}
{% endif -%}
{% if post_hints | length != 0 %}

{% for post_hint in post_hints -%}
{{ post_hint }}
{% endfor -%}
{% endif -%}
