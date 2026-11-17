use iced::widget::{Container, Row, column, scrollable, text, text_input};
use iced::{Element, Length};

use super::Screen;

#[derive(Debug, Clone)]
pub struct Record {
    pub file_name: String,
    pub keyword: String,
    pub match_count: usize,
    pub searched_at: String,
}

pub fn view<'a, Message>(
    screen: Screen,
    records: &'a [Record],
    search: &str,
    loading: bool,
    error: &Option<String>,
    sort_by: SortBy,
) -> Element<'a, Message>
where
    Message: 'a + From<SearchChanged> + From<Refresh> + From<SortByName>,
{
    let _ = screen;

    let search_box = text_input("Search...", search)
        .on_input(|v| Message::from(SearchChanged(v)))
        .width(Length::FillPortion(3));

    let sort_name = iced::widget::button("Sort by name")
        .on_press(Message::from(SortByName))
        .width(Length::Fixed(120.0));

    let sort_keyword = iced::widget::button("Sort by keyword")
        .on_press(Message::from(SortByName))
        .width(Length::Fixed(140.0));

    let refresh = iced::widget::button("Refresh")
        .on_press(Message::from(Refresh))
        .width(Length::Fixed(120.0));

    let top_bar: Row<'a, Message> = Row::new()
        .push(search_box)
        .push(iced::widget::Space::with_width(8))
        .push(sort_name)
        .push(iced::widget::Space::with_width(8))
        .push(sort_keyword)
        .push(iced::widget::Space::with_width(8))
        .push(refresh);

    let status = if loading {
        text("Loading...").size(14)
    } else if let Some(err) = error {
        text(format!("Error: {}", err)).size(14)
    } else if records.is_empty() {
        text("No activity yet.").size(14)
    } else {
        text(format!("{} searches", records.len())).size(14)
    };

    let header = Row::new()
        .push(text("File name").width(Length::Fixed(180.0)))
        .push(text("Keyword").width(Length::Fixed(140.0)))
        .push(text("Matches").width(Length::Fixed(100.0)))
        .push(text("Searched at").width(Length::Fixed(180.0)));

    let mut list = scrollable(column::new().push(header));
    for rec in records {
        list = list.push(
            Row::new()
                .push(text(&rec.file_name).width(Length::Fixed(180.0)))
                .push(text(&rec.keyword).width(Length::Fixed(140.0)))
                .push(text(rec.match_count.to_string()).width(Length::Fixed(100.0)))
                .push(text(&rec.searched_at).width(Length::Fixed(180.0))),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Name,
    Keyword,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    Refresh,
    SortByName,
}

#[derive(Debug, Clone)]
pub struct SearchChanged(pub String);
#[derive(Debug, Clone)]
pub struct Refresh;
#[derive(Debug, Clone)]
pub struct SortByName;
