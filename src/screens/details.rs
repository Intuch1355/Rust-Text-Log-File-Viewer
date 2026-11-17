use iced::widget::{column, text, Container, Row};
use iced::{Length, Element};

use super::Screen;

pub fn view<'a, Message>(
    screen: Screen,
    file_info: &Option<FileInfo>,
    confirm: &Option<String>,
) -> Element<'a, Message>
where
    Message: 'a + From<AskSave> + From<ConfirmSave> + From<CancelSave>,
{
    let _ = screen;

    let dialog = if let Some(report_path) = confirm {
        Some(
            Container::new(
                column::new()
                    .push(text(format!("Save report to {}?", report_path)).size(16))
                    .push(iced::widget::Space::with_height(8))
                    .push(
                        Row::new()
                            .push(
                                iced::widget::button("Yes")
                                    .on_press(Message::from(ConfirmSave))
                                    .width(Length::Fixed(80.0)),
                            )
                            .push(iced::widget::Space::with_width(8))
                            .push(
                                iced::widget::button("Cancel")
                                    .on_press(Message::from(CancelSave))
                                    .width(Length::Fixed(80.0)),
                            ),
                    ),
            )
            .padding(8),
        )
    } else {
        None
    };

    let metadata = match file_info {
        Some(info) => column::new()
            .push(text("File Details").size(20))
            .push(iced::widget::Space::with_height(8))
            .push(Row::new()
                .push(text("Name:").width(Length::Fixed(120.0)))
                .push(text(&info.name)))
            .push(Row::new()
                .push(text("Path:").width(Length::Fixed(120.0)))
                .push(text(&info.path)))
            .push(Row::new()
                .push(text("Extension:").width(Length::Fixed(120.0)))
                .push(text(info.extension.as_deref().unwrap_or("(none)")))
            .push(Row::new()
                .push(text("Size:").width(Length::Fixed(120.0)))
                .push(text(format!("{} bytes", info.size))))
            .push(Row::new()
                .push(text("Total lines:").width(Length::Fixed(120.0)))
                .push(text(info.total_lines.to_string()))),
        None => column::new()
            .push(text("No file loaded yet.").size(16)),
    };

    let save_button = iced::widget::button("Save report")
        .on_press(Message::from(AskSave))
        .width(Length::Fixed(160.0));

    let mut content = column::new()
        .push(metadata)
        .push(iced::widget::Space::with_height(16))
        .push(save_button);

    if let Some(d) = dialog {
        content = content.push(iced::widget::Space::with_height(8)).push(d);
    }

    Container::new(content)
        .padding(16)
        .into()
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub extension: Option<String>,
    pub size: u64,
    pub total_lines: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    AskSave,
    ConfirmSave,
    CancelSave,
}

#[derive(Debug, Clone)]
pub struct AskSave;
#[derive(Debug, Clone)]
pub struct ConfirmSave;
#[derive(Debug, Clone)]
pub struct CancelSave;
