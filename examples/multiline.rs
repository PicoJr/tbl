use itertools::Itertools;
use std::iter::FromIterator;
use tbl::{Block, RenderBlock, Renderer, TBLError};
use termion::color;

fn chunkify(s: &String, size: usize) -> Vec<String> {
    let inter = s.chars().collect_vec();
    let chunks = inter.chunks_exact(size);
    let remainder = chunks.remainder().to_vec();
    let padding = std::iter::repeat(' ')
        .take(size - remainder.len())
        .collect_vec();
    let padded_remainder: Vec<char> = remainder
        .iter()
        .chain(padding.iter())
        .cloned()
        .collect_vec();
    chunks
        .chain(std::iter::once(padded_remainder.as_slice()))
        .map(|s| String::from_iter(s.iter()))
        .collect_vec()
}

fn render(b: &Block<String>) -> RenderBlock {
    match b {
        Block::Space(length) => RenderBlock::Space(format!(
            "{}{}{}",
            color::Bg(color::Black),
            " ".repeat(*length),
            color::Bg(color::Reset)
        )),
        Block::Segment(length, label) => {
            let label = label.clone().unwrap_or_default();
            let chunks = chunkify(&label, *length);
            let color_chunks = chunks
                .iter()
                .map(|s| format!("{}{}{}", color::Bg(color::Blue), s, color::Bg(color::Reset)))
                .collect_vec();
            RenderBlock::MultiLineBlock(color_chunks)
        }
    }
}

fn main() -> Result<(), TBLError> {
    let data = vec![(0., 2.), (3., 4.), (5., 6.)];
    let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| {
        Some("hello world".to_string())
    })
    .with_length(42)
    .with_renderer(&render)
    .render()?;
    for line in rendered {
        println!("{}", line);
    }
    Ok(())
}