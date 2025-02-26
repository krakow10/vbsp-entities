VBSP Entities For Team Fortress 2
=================================

[![Latest version](https://img.shields.io/crates/v/vbsp-entities-tf2.svg)](https://crates.io/crates/vbsp-entities-tf2)
![License](https://img.shields.io/crates/l/vbsp-entities-tf2.svg)

## TF2 Entities (Generated)

Generated by [vbsp-entities-codegen](https://github.com/krakow10/vbsp-entities-codegen).
Intended for use with the [vbsp crate](https://crates.io/crates/vbsp).

### Usage:
```rust
use vbsp::Bsp;
use vbsp_entities_tf2::Entity;

let data = std::fs::read("tf/maps/ctf_2fort.bsp")?;
let bsp = Bsp::read(&data)?;

for entity in &bsp.entities {
	match entity.parse() {
		// print Medkit locations
		Ok(Entity::ItemHealthkitFull(item_healthkit_full)) => dbg!(item_healthkit_full.origin),
		_ => (),
	}
}
```
