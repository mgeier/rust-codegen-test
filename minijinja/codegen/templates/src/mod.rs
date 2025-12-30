// This file has been auto-generated ...
// arc = {{ arc }}
// bip = {{ bip }}
// pow2 = {{ pow2 }}

{% include "definitions.jinja" %}

{% if N %}
pub struct RingBuffer<const N: usize, T> {
}
{% else %}
pub struct RingBuffer<T> {
}
{% endif %}

impl<T> RingBuffer<T> {
    fn new() -> Self { Self {} }
}

{% if arc %}
impl<{{ params }}> RingBuffer<{{ args }}> {
    pub fn producer(&self) -> Option<Producer<{{ blank_lifetime }}>> {
    }
}
{% endif %}
