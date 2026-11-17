mod activity;
mod details;
mod viewer;

pub use activity::view as activity_view;
pub use details::view as details_view;
pub use viewer::view as viewer_view;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Viewer,
    Details,
    Activity,
}
