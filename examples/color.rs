use tbl::{Block, RenderBlock, Renderer, TBLError};
use termion::color;

fn render(b: &Block<String>) -> RenderBlock {
    match b {
        Block::Space(length) => RenderBlock::Space(format!(
            "{}{}{}",
            color::Bg(color::Black),
            " ".repeat(*length),
            color::Bg(color::Reset)
        )),
        Block::Segment(length, label) => {
            let mut truncated = label.clone().unwrap_or_default();
            truncated.truncate(*length);
            RenderBlock::Block(format!(
                "{}{}{}{}",
                color::Bg(color::LightGreen),
                truncated,
                " ".repeat(*length - truncated.len()),
                color::Bg(color::Reset),
            ))
        }
    }
}

fn main() -> Result<(), TBLError<String>> {
    let data = vec![(0., 2.), (3., 4.)];
    let rendered = Renderer::new(data.as_slice(), &|&e| e, &|e| {
        Some(format!("label for {:?}", e))
    })
    .with_length(90)
    .with_renderer(&render)
    .render()?;
    for line in rendered {
        println!("{}", line);
    }
    Ok(())
}
