// Keep the existing filename for now; expose it through the correctly named module API.
mod file_mamager;
mod hidden;

pub use file_mamager::collect_files;
pub(crate) use file_mamager::fetch_item_names;
pub use hidden::is_hidden;
