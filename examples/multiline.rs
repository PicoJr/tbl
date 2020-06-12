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
        Block::Segment(length, _) => RenderBlock::MultiLineBlock(vec![
            format!(
                "{}{}{}",
                color::Bg(color::Blue),
                " ".repeat(*length),
                color::Bg(color::Reset)
            ),
            format!(
                "{}{}{}",
                color::Bg(color::Blue),
                " ".repeat(*length),
                color::Bg(color::Reset)
            ),
        ]),
    }
}

fn main() -> Result<(), TBLError> {
    let data = vec![(0., 2.), (3., 4.)];
    let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
        .with_length(42)
        .with_renderer(&render)
        .render()?;
    for line in rendered {
        println!("{}", line);
    }
    Ok(())
}
