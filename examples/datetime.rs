use chrono::{DateTime, Local, TimeZone, Utc};
use tbl::{Block, Bound, RenderBlock, Renderer, TBLError};
use termion::color;

type RGB = (u8, u8, u8);

struct Activity {
    start: DateTime<Local>,
    end: DateTime<Local>,
    label: Option<(String, RGB)>,
}

fn fbounds(activity: &Activity) -> Bound {
    (
        activity.start.timestamp() as f64,
        activity.end.timestamp() as f64,
    )
}

fn label_activity(activity: &Activity) -> Option<(String, RGB)> {
    activity.label.clone()
}

fn label_legend(activity: &Activity) -> Option<(String, RGB)> {
    Some((
        format!(
            "{}-{}",
            activity.start.format("%H:%M").to_string(),
            activity.end.format("%H:%M")
        ),
        (96, 125, 139),
    ))
}

fn render(b: &Block<(String, RGB)>) -> RenderBlock {
    match b {
        Block::Space(length) => RenderBlock::Space(format!(
            "{}{}{}",
            color::Bg(color::Reset),
            " ".repeat(*length),
            color::Bg(color::Reset)
        )),
        Block::Segment(length, label) => {
            let (label, (r, g, b)) = label.clone().unwrap_or_else(|| ("".to_string(), (0, 0, 0)));
            let mut truncated = label.clone();
            truncated.truncate(*length);
            RenderBlock::Block(format!(
                "{}{}{}{}",
                color::Bg(color::Rgb(r, g, b)),
                truncated,
                " ".repeat(*length - truncated.len()),
                color::Bg(color::Reset),
            ))
        }
    }
}

fn main() -> Result<(), TBLError> {
    // this isn't the real Apollo 11 timeline, it's just an example ;-)
    let data = vec![
        Activity {
            start: Utc.ymd(1969, 7, 20).and_hms(8, 0, 0).into(),
            end: Utc.ymd(1969, 7, 20).and_hms(9, 20, 0).into(),
            label: Some(("breakfast".to_string(), (139, 195, 74))),
        },
        Activity {
            start: Utc.ymd(1969, 7, 20).and_hms(9, 30, 0).into(),
            end: Utc.ymd(1969, 7, 20).and_hms(11, 0, 0).into(),
            label: Some(("launch".to_string(), (255, 152, 0))),
        },
        Activity {
            start: Utc.ymd(1969, 7, 20).and_hms(12, 0, 0).into(),
            end: Utc.ymd(1969, 7, 20).and_hms(19, 0, 0).into(),
            label: Some((
                "orbit the moon and count craters".to_string(),
                (3, 169, 244),
            )),
        },
        Activity {
            start: Utc.ymd(1969, 7, 20).and_hms(20, 17, 0).into(),
            end: Utc.ymd(1969, 7, 20).and_hms(22, 0, 0).into(),
            label: Some(("moon walk".to_string(), (96, 125, 139))),
        },
    ];
    let legend = Renderer::new(data.as_slice(), &fbounds, &label_legend)
        .with_length(120)
        .with_renderer(&render)
        .render()?;
    println!("{}", legend);
    let rendered = Renderer::new(data.as_slice(), &fbounds, &label_activity)
        .with_length(120)
        .with_renderer(&render)
        .render()?;
    println!("{}", rendered);
    Ok(())
}
