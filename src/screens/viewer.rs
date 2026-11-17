use iced::widget::{Container, Row, column, scrollable, text, text_input};
use iced::{Element, Length};

use super::Screen;

pub fn view<'a, Message>(
    screen: Screen,
    path: &str,
    keyword: &str,
    loading: bool,
    error: &Option<String>,
    lines: &'a [(usize, String)],
    matched_count: usize,
) -> Element<'a, Message>
where
    Message: 'a + From<PathChanged> + From<KeywordChanged> + From<Analyze> + From<Refresh>,
{
    let _ = screen;

    let path_input = text_input("File path", path)
        .on_input(|v| Message::from(PathChanged(v)))
        .width(Length::FillPortion(3));

    let keyword_input = text_input("Keyword", keyword)
        .on_input(|v| Message::from(KeywordChanged(v)))
        .width(Length::FillPortion(2));

    let analyze_button = iced::widget::button("Analyze")
        .on_press(Message::from(Analyze))
        .width(Length::Fixed(120.0));

    let refresh_button = iced::widget::button("Refresh")
        .on_press(Message::from(Refresh))
        .width(Length::Fixed(120.0));

    let top_bar: Row<'a, Message> = Row::new()
        .push(path_input)
        .push(iced::widget::Space::with_width(8))
        .push(keyword_input)
        .push(iced::widget::Space::with_width(8))
        .push(analyze_button)
        .push(iced::widget::Space::with_width(8))
        .push(refresh_button);

    let status = if loading {
        text("Loading...").size(14)
    } else if let Some(err) = error {
        text(format!("Error: {}", err)).size(14)
    } else if lines.is_empty() {
        text("No file opened yet.").size(14)
    } else {
        text(format!(
            "Total lines: {} | Matches \"{}\": {}",
            lines.len(),
            keyword,
            matched_count
        ))
        .size(14)
    };

    let mut list = scrollable(column::new().push(text("Lines").size(16)));
    for (num, line) in lines {
        list = list.push(
            Row::new()
                .push(text(format!("{}:", num)).width(Length::Fixed(60.0)))
                .push(text(line)),
        );
    }

    Container::new(
        column::new()
            .push(top_bar)
            .push(iced::widget::Space::with_height(8))
            .push(status)
            .push(iced::widget::Space::with_height(8))
            .push(list),
    )
    .padding(8)
    .into()
}

#[derive(Debug, Clone)]
pub enum Message {
    PathChanged(String),
    KeywordChanged(String),
    Analyze,
    Refresh,
}

#[derive(Debug, Clone)]
pub struct PathChanged(pub String);
#[derive(Debug, Clone)]
pub struct KeywordChanged(pub String);
#[derive(Debug, Clone)]
pub struct Analyze;
#[derive(Debug, Clone)]
pub struct Refresh;
