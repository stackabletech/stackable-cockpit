{% if pre_hints | length != 0 -%}
{% for pre_hint in pre_hints -%}
{{ pre_hint }}
{% endfor %}
{% endif -%}

{%- if output | length != 0 %}
{{ output }}
{%- endif %}

{% if post_hints | length != 0 -%}
{% for post_hint in post_hints -%}
{{ post_hint }}
{% endfor -%}
{% endif -%}
