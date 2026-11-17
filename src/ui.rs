use iced::{
    Application, Command, Element, Length, Settings, Theme, executor,
    widget::{button, column, container, row, scrollable, text, text_input},
};

use std::fs;
use std::path::Path;

use crate::data;
use sqlx::MySqlPool;

// Screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Screen {
    #[default]
    Viewer,
    Details,
    Activity,
}

// FileInfo
#[derive(Debug, Clone)]
struct FileInfo {
    name: String,
    path: String,
    extension: Option<String>,
    size: u64,
    total_lines: usize,
}

// ActivityRecord
#[derive(Debug, Clone)]
struct ActivityRecord {
    file_name: String,
    keyword: String,
    match_count: usize,
}

// LoadedFile
#[derive(Debug, Clone)]
pub(crate) struct LoadedFile {
    content: String,
    info: FileInfo,
}

// Message
#[derive(Debug, Clone)]
pub enum Message {
    // Input
    FilePathChanged(String),
    KeywordChanged(String),

    // File operations
    Analyze,
    Refresh,
    FileLoaded {
        keyword: String,
        result: Result<LoadedFile, String>,
    },

    // Navigation
    ViewerScreen,
    DetailsScreen,
    ActivityScreen,

    // Report confirmation
    AskSave,
    ConfirmSave,
    CancelSave,

    // Activity
    ActivitySearchChanged(String),
    SortByName,
    SortByKeyword,

    // MySQL
    DbConnected(Result<MySqlPool, String>),
    DbError(String),
}

// Application State
#[derive(Default)]
pub struct App {
    // MySQL
    db_pool: Option<MySqlPool>,

    // Input
    file_path: String,
    keyword: String,

    // Navigation
    current_screen: Screen,

    // File data
    file_info: Option<FileInfo>,
    filtered_lines: Vec<(usize, String)>,
    total_lines: usize,
    match_count: usize,

    // Status
    loading: bool,
    error_message: Option<String>,
    notice_message: Option<String>,

    // Save report
    confirm_save: bool,

    // Activity
    activity: Vec<ActivityRecord>,
    activity_search: String,
}

// Iced app
impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::default(),
            Command::perform(
                async move {
                    data::connect_db()
                        .await
                        .map_err(|error| format!("MySQL connection failed: {}", error))
                },
                Message::DbConnected,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Text / Log File Viewer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // mysql
            Message::DbConnected(result) => match result {
                Ok(pool) => {
                    self.db_pool = Some(pool);
                    self.notice_message = Some(String::from("MySQL connected."));
                    self.error_message = None;
                }
                Err(error) => {
                    self.error_message = Some(error);
                }
            },

            Message::DbError(error) => {
                self.error_message = Some(error);
            }

            // input
            Message::FilePathChanged(path) => {
                self.file_path = path;
            }

            Message::KeywordChanged(keyword) => {
                self.keyword = keyword;
            }

            // Analyze
            Message::Analyze | Message::Refresh => {
                if self.file_path.trim().is_empty() {
                    self.error_message = Some(String::from("Please enter a file path."));
                    return Command::none();
                }

                let path = self.file_path.clone();
                let keyword = self.keyword.clone();

                self.loading = true;
                self.error_message = None;
                self.notice_message = None;

                return Command::perform(
                    async move {
                        let content = data::read_file(&path).map_err(|error| error.to_string())?;

                        let size = data::get_file_size(&path).map_err(|error| error.to_string())?;

                        let extension = data::get_file_extension(&path);

                        let name = Path::new(&path)
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("Unknown")
                            .to_string();

                        let total_lines = data::count_lines(&content);

                        Ok(LoadedFile {
                            content,
                            info: FileInfo {
                                name,
                                path,
                                extension,
                                size,
                                total_lines,
                            },
                        })
                    },
                    move |result| Message::FileLoaded { keyword, result },
                );
            }

            // file load
            Message::FileLoaded { keyword, result } => {
                self.loading = false;

                match result {
                    Ok(file) => {
                        let matches = data::count_matches(&file.content, &keyword);

                        let lines: Vec<(usize, String)> = file
                            .content
                            .lines()
                            .enumerate()
                            .filter(|(_, line)| keyword.is_empty() || line.contains(&keyword))
                            .map(|(index, line)| (index + 1, line.to_string()))
                            .collect();

                        self.total_lines = file.info.total_lines;
                        self.match_count = matches;
                        self.filtered_lines = lines;

                        self.activity.insert(
                            0,
                            ActivityRecord {
                                file_name: file.info.name.clone(),
                                keyword: keyword.clone(),
                                match_count: matches,
                            },
                        );

                        self.file_info = Some(file.info.clone());
                        self.error_message = None;

                        // Save to MySQL
                        if let Some(pool) = &self.db_pool {
                            let pool = pool.clone();
                            let info = data::FileInfo {
                                name: file.info.name,
                                path: file.info.path,
                                extension: file.info.extension,
                                size: file.info.size,
                                total_lines: file.info.total_lines,
                            };
                            let keyword = keyword.clone();

                            return Command::perform(
                                async move {
                                    match data::save_file_info(&pool, &info).await {
                                        Ok(file_id) => {
                                            match data::save_search_log(
                                                &pool,
                                                file_id,
                                                &keyword,
                                                matches,
                                            )
                                            .await
                                            {
                                                Ok(()) => Ok(()),
                                                Err(error) => Err(format!(
                                                    "MySQL search log error: {}",
                                                    error
                                                )),
                                            }
                                        }
                                        Err(error) => Err(format!(
                                            "MySQL file info error: {}",
                                            error
                                        )),
                                    }
                                },
                                |res| match res {
                                    Ok(()) => Message::DbError(String::from(
                                        "Saved to MySQL.",
                                    )),
                                    Err(error) => Message::DbError(error),
                                },
                            );
                        }
                    }

                    Err(error) => {
                        self.error_message = Some(error);
                    }
                }
            }

            // navigation
            Message::ViewerScreen => {
                self.current_screen = Screen::Viewer;
            }

            Message::DetailsScreen => {
                self.current_screen = Screen::Details;
            }

            Message::ActivityScreen => {
                self.current_screen = Screen::Activity;
            }

            // save repot
            Message::AskSave => {
                if self.file_info.is_some() {
                    self.confirm_save = true;
                    self.notice_message = None;
                }
            }

            Message::CancelSave => {
                self.confirm_save = false;
            }

            Message::ConfirmSave => {
                self.confirm_save = false;

                if let Some(info) = &self.file_info {
                    let mut report = String::new();

                    report.push_str("TEXT / LOG FILE VIEWER REPORT\n");
                    report.push_str("=============================\n\n");

                    report.push_str(&format!("File: {}\n", info.name));
                    report.push_str(&format!("Path: {}\n", info.path));
                    report.push_str(&format!(
                        "Extension: {}\n",
                        info.extension.as_deref().unwrap_or("None")
                    ));
                    report.push_str(&format!("Size: {} bytes\n", info.size));
                    report.push_str(&format!("Total lines: {}\n", self.total_lines));
                    report.push_str(&format!("Keyword: {}\n", self.keyword));
                    report.push_str(&format!("Matches: {}\n\n", self.match_count));

                    report.push_str("MATCHING LINES\n");
                    report.push_str("--------------\n");

                    for (number, line) in &self.filtered_lines {
                        report.push_str(&format!("{}: {}\n", number, line));
                    }

                    match fs::write("log_viewer_report.txt", report) {
                        Ok(()) => {
                            self.notice_message =
                                Some(String::from("Report saved as log_viewer_report.txt"));
                            self.error_message = None;
                        }

                        Err(error) => {
                            self.error_message = Some(error.to_string());
                        }
                    }
                }
            }

            // activity_search
            Message::ActivitySearchChanged(search) => {
                self.activity_search = search;
            }

            // activity_sort
            Message::SortByName => {
                self.activity.sort_by(|a, b| a.file_name.cmp(&b.file_name));
            }

            Message::SortByKeyword => {
                self.activity.sort_by(|a, b| a.keyword.cmp(&b.keyword));
            }
        }

        Command::none()
    }

    // main view
    fn view(&self) -> Element<'_, Message> {
        let navigation = row![
            button("File Viewer").on_press(Message::ViewerScreen),
            button("File Details").on_press(Message::DetailsScreen),
            button("Activity").on_press(Message::ActivityScreen),
        ]
        .spacing(10);

        let content = match self.current_screen {
            Screen::Viewer => self.viewer_view(),
            Screen::Details => self.details_view(),
            Screen::Activity => self.activity_view(),
        };

        container(
            column![text("TEXT / LOG FILE VIEWER").size(28), navigation, content,]
                .spacing(20)
                .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

// screen 1
impl App {
    fn viewer_view(&self) -> Element<'_, Message> {
        let path_input = text_input("Enter file path...", &self.file_path)
            .on_input(Message::FilePathChanged)
            .width(Length::FillPortion(3));

        let keyword_input = text_input("Enter keyword...", &self.keyword)
            .on_input(Message::KeywordChanged)
            .width(Length::FillPortion(2));

        let analyze_button = button("Analyze").on_press(Message::Analyze);

        let refresh_button = button("Refresh").on_press(Message::Refresh);

        let inputs = row![path_input, keyword_input, analyze_button, refresh_button,].spacing(10);

        let status = if self.loading {
            text("Loading...")
        } else if let Some(error) = &self.error_message {
            text(format!("Error: {}", error))
        } else if self.file_info.is_none() {
            text("No file opened yet.")
        } else {
            text(format!(
                "Total lines: {} | Keyword occurrences: {}",
                self.total_lines, self.match_count
            ))
        };

        let mut lines = column![text("File Contents / Matching Lines").size(18)].spacing(8);

        for (number, line) in &self.filtered_lines {
            lines = lines.push(
                row![
                    text(format!("{}:", number)).width(Length::Fixed(60.0)),
                    text(line),
                ]
                .spacing(10),
            );
        }

        column![inputs, status, scrollable(lines).height(Length::Fill),]
            .spacing(15)
            .into()
    }
}

// screen 2
impl App {
    fn details_view(&self) -> Element<'_, Message> {
        let metadata = match &self.file_info {
            Some(info) => column![
                text("FILE DETAILS").size(22),
                row![text("Name:").width(Length::Fixed(120.0)), text(&info.name),],
                row![text("Path:").width(Length::Fixed(120.0)), text(&info.path),],
                row![
                    text("Extension:").width(Length::Fixed(120.0)),
                    text(info.extension.as_deref().unwrap_or("(none)")),
                ],
                row![
                    text("Size:").width(Length::Fixed(120.0)),
                    text(format!("{} bytes", info.size)),
                ],
                row![
                    text("Total lines:").width(Length::Fixed(120.0)),
                    text(info.total_lines.to_string()),
                ],
            ]
            .spacing(12),

            None => {
                column![text("No file loaded yet.").size(18)]
            }
        };

        let save_button = button("Save report").on_press(Message::AskSave);

        let db_status = match &self.db_pool {
            Some(_) => text("MySQL: connected").size(14),
            None => text("MySQL: not connected").size(14),
        };

        let mut content = column![metadata, save_button, db_status,].spacing(20);

        if self.confirm_save {
            let confirmation = container(
                column![
                    text("Save report to log_viewer_report.txt?").size(16),
                    row![
                        button("Yes").on_press(Message::ConfirmSave),
                        button("Cancel").on_press(Message::CancelSave),
                    ]
                    .spacing(10),
                ]
                .spacing(10),
            )
            .padding(15);

            content = content.push(confirmation);
        }

        if let Some(notice) = &self.notice_message {
            content = content.push(text(notice));
        }

        if let Some(error) = &self.error_message {
            content = content.push(text(format!("Error: {}", error)));
        }

        content.into()
    }
}

// screen 3
impl App {
    fn activity_view(&self) -> Element<'_, Message> {
        let search_input = text_input("Search activity...", &self.activity_search)
            .on_input(Message::ActivitySearchChanged)
            .width(Length::Fill);

        let controls = row![
            search_input,
            button("Sort by name").on_press(Message::SortByName),
            button("Sort by keyword").on_press(Message::SortByKeyword),
        ]
        .spacing(10);

        let header = row![
            text("File name").width(Length::FillPortion(3)),
            text("Keyword").width(Length::FillPortion(2)),
            text("Matches").width(Length::FillPortion(1)),
        ];

        let search = self.activity_search.to_lowercase();

        let mut records = column![header].spacing(10);

        let mut found = false;

        for record in &self.activity {
            let matches_search = record.file_name.to_lowercase().contains(&search)
                || record.keyword.to_lowercase().contains(&search);

            if matches_search {
                found = true;

                records = records.push(
                    row![
                        text(&record.file_name).width(Length::FillPortion(3)),
                        text(&record.keyword).width(Length::FillPortion(2)),
                        text(record.match_count.to_string()).width(Length::FillPortion(1)),
                    ]
                    .spacing(10),
                );
            }
        }

        let status = if self.activity.is_empty() {
            text("No activity yet.")
        } else if !found {
            text("No matching activity found.")
        } else {
            text(format!("{} activity records", self.activity.len()))
        };

        column![
            text("ACTIVITY HISTORY").size(22),
            controls,
            status,
            scrollable(records).height(Length::Fill),
            text("Note: Activity is stored in memory and saved to MySQL on each analysis."),
        ]
        .spacing(15)
        .into()
    }
}

// run app
pub fn run() -> iced::Result {
    App::run(Settings::default())
}
