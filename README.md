# TBL

**T**erminal **B**ar (time)**L**ine (**WIP**)

`cargo run --example datetime`

![](img/timeline.png)

## Example

``` rust
use tbl::{Renderer, TBCError};

fn main() -> Result<(), TBCError> {
    let data = vec![(0., 2.), (3., 4.)];
    let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
        .with_length(42)
        .render()?;
    println!("{}", rendered);
    Ok(())
}
```

Output:

```text
=====================          ===========
```

See [examples](examples) folder for more examples.

## TODO

- [ ] support or forbid joint intervals e.g. `[(0,2), (1,3)]`
- [ ] prepare for release on crate.io
- [ ] add doc
- [ ] add test
- [ ] split `build_blocks` into several parts