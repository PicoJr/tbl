use tbl::{Block, BlockRenderer, RenderBlock, Renderer, TBCError};

struct CustomRenderer {}

impl BlockRenderer<String> for CustomRenderer {
    fn render(&self, b: &Block<String>) -> RenderBlock {
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
}

fn main() -> Result<(), TBCError> {
    let data = vec![(0., 2.), (3., 4.)];
    let rendered = Renderer::new(data.as_slice(), &|&e| e, &|e| {
        Some(format!("label for {:?}", e))
    })
    .with_length(90)
    .with_renderer(&CustomRenderer {})
    .render()?;
    println!("{}", rendered);
    Ok(())
}
