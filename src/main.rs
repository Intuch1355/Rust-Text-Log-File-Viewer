mod data;
mod ui;

fn main() -> iced::Result {
    ui::run()
}

#[cfg(test)]
mod unit_test;