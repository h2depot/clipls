use std::{fs::Metadata, os::windows::fs::MetadataExt, path::Path};

const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

pub(super) fn is_hidden(path: &Path, metadata: &Metadata) -> bool {
    has_hidden_attribute(metadata.file_attributes()) || has_dot_prefix(path)
}

fn has_hidden_attribute(attributes: u32) -> bool {
    attributes & FILE_ATTRIBUTE_HIDDEN != 0
}

fn has_dot_prefix(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.'))
}

#[cfg(test)]
mod tests {
    use super::{FILE_ATTRIBUTE_HIDDEN, has_hidden_attribute};

    #[test]
    fn recognizes_the_windows_hidden_attribute() {
        assert!(has_hidden_attribute(FILE_ATTRIBUTE_HIDDEN));
        assert!(has_hidden_attribute(FILE_ATTRIBUTE_HIDDEN | 0x20));
        assert!(!has_hidden_attribute(0x20));
    }
}
