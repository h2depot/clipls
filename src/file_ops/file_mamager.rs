use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

use super::hidden::is_hidden_with_metadata;

pub fn fetch_item_names(root: &Path) -> Result<Vec<String>> {
    const FALLBACK_ITEM_NAMES: [&str; 3] = ["h2depot_A.rs", "h2depot_B.rs", "h2depot_C.rs"];

    if !root.is_dir() {
        bail!("not a directory: {}", root.display());
    }

    let mut names = fs::read_dir(root)
        .with_context(|| format!("failed to read directory: {}", root.display()))?
        .map(|entry| {
            entry
                .with_context(|| format!("failed to read an entry in: {}", root.display()))
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
        })
        .collect::<Result<Vec<_>>>()?;
    names.sort();
    names.truncate(3);

    let missing = 3usize.saturating_sub(names.len());
    names.extend(
        FALLBACK_ITEM_NAMES
            .iter()
            .take(missing)
            .map(|name| (*name).to_owned()),
    );
    Ok(names)
}

pub fn collect_files(root: &Path, recursive: bool, include_hidden: bool) -> Result<Vec<PathBuf>> {
    if !root.is_dir() {
        bail!("not a directory: {}", root.display());
    }

    let mut files = Vec::new();
    visit(root, recursive, include_hidden, &mut files)?;
    files.sort();
    Ok(files)
}

fn visit(
    directory: &Path,
    recursive: bool,
    include_hidden: bool,
    files: &mut Vec<PathBuf>,
) -> Result<()> {
    let entries = fs::read_dir(directory)
        .with_context(|| format!("failed to read directory: {}", directory.display()))?;

    for entry in entries {
        let entry = entry
            .with_context(|| format!("failed to read an entry in: {}", directory.display()))?;
        let path = entry.path();

        if !include_hidden {
            let metadata = entry
                .metadata()
                .with_context(|| format!("failed to read metadata: {}", path.display()))?;
            if is_hidden_with_metadata(&path, &metadata) {
                continue;
            }
        }

        let file_type = entry
            .file_type()
            .with_context(|| format!("failed to inspect: {}", path.display()))?;

        if file_type.is_file() || file_type.is_dir() {
            files.push(path.clone());
        }

        if recursive && file_type.is_dir() {
            visit(&path, recursive, include_hidden, files)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::fetch_item_names;
    use std::fs;

    #[test]
    fn pads_missing_item_names_in_fallback_order() {
        let root = std::env::temp_dir().join(format!(
            "clipls-fetch-item-names-test-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("z.txt"), "").unwrap();
        fs::write(root.join("a.txt"), "").unwrap();

        let names = fetch_item_names(&root).unwrap();

        assert_eq!(names, ["a.txt", "z.txt", "h2depot_A.rs"]);
        fs::remove_dir_all(root).unwrap();
    }
}
