[![tbl crate](https://img.shields.io/crates/v/tbl.svg)](https://crates.io/crates/tbl)
[![tbl documentation](https://docs.rs/tbl/badge.svg)](https://docs.rs/tbl)
[![GitHub license](https://img.shields.io/github/license/PicoJr/tbl)](https://github.com/PicoJr/tbl/blob/master/LICENSE)
# TBL

**T**erminal **B**ar (time)**L**ine (**WIP**)

`cargo run --example datetime`

![](img/timeline.png)

## Example

```rust
use tbl::Renderer;

let data = vec![(0., 2.), (3., 4.)];
let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>) // explicit type for Option<Label>
    .with_length(42)
    .render()
    .unwrap();
for line in rendered.iter().flatten() {
    assert_eq!(line, "=====================          ===========");
}
```

## Custom Data and Renderer

```rust
use tbl::{Block, RenderBlock, Renderer, TBLError, Bound};

struct CustomData {
   bounds: (usize, usize),
   label: String // must be Clone + Debug
}

fn bounds(cd: &CustomData)-> Bound {
    let (a, b) = cd.bounds;
    (a as f64, b as f64)
}

fn label(cd: &CustomData)-> Option<String> {
   Some(cd.label.clone())
}

fn render(b: &Block<String>) -> RenderBlock {
   match b {
       Block::Space(length) => RenderBlock::Space("\u{2606}".repeat(*length)),
       Block::Segment(length, label) => {
           let mut truncated = label.clone().unwrap_or_default();
           truncated.truncate(*length);
           RenderBlock::Block(format!(
               "{}{}",
               truncated,
               "\u{2605}".repeat(*length - truncated.len())
           ))
       }
   }
}

let data = vec![CustomData{bounds: (0, 2), label: "hello".to_string()}, CustomData{bounds: (3, 4), label: "world!".to_string()}];
let rendered = Renderer::new(data.as_slice(), &bounds, &label)
       .with_length(60)
       .with_renderer(&render)
       .render().unwrap();
for line in rendered.iter().flatten() {
    assert_eq!(line, "hello★★★★★★★★★★★★★★★★★★★★★★★★★☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆world!★★★★★★★★★");
}
```

See [examples](examples) folder for more examples.

## Changelog

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

## TODO

- [x] support overlapping intervals e.g. `[(0,2), (1,3)]`
- [x] prepare for release on crate.io
- [x] add doc
- [x] add test
- [ ] split `build_blocks` into several parts
