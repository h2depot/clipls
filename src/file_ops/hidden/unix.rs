use std::{fs::Metadata, path::Path};

pub(super) fn is_hidden(path: &Path, _metadata: &Metadata) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.'))
}
