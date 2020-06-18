use tbl::{Renderer, TBLError};

fn main() -> Result<(), TBLError<String>> {
    let data = vec![(0., 2.), (3., 4.)];
    let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
        .with_length(42)
        .render()?;
    for line in rendered {
        println!("{}", line);
    }
    Ok(())
}
