use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

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

        if !include_hidden && is_hidden(&path) {
            continue;
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

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.'))
}

#[cfg(test)]
mod tests {
    use super::collect_files;
    use std::{fs, path::PathBuf};

    #[test]
    fn includes_hidden_files_and_directories() {
        let root = std::env::temp_dir().join(format!("clipls-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("target")).unwrap();
        fs::write(root.join(".gitignore"), "target\n").unwrap();

        let entries = collect_files(&root, false, true).unwrap();
        let relative: Vec<PathBuf> = entries
            .iter()
            .map(|path| path.strip_prefix(&root).unwrap().to_owned())
            .collect();

        assert_eq!(
            relative,
            [
                PathBuf::from(".gitignore"),
                PathBuf::from("src"),
                PathBuf::from("target")
            ]
        );

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn excludes_hidden_files_by_default() {
        let root = std::env::temp_dir().join(format!("clipls-hidden-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(root.join(".gitignore"), "target\n").unwrap();

        let entries = collect_files(&root, false, false).unwrap();
        let relative: Vec<PathBuf> = entries
            .iter()
            .map(|path| path.strip_prefix(&root).unwrap().to_owned())
            .collect();

        assert_eq!(relative, [PathBuf::from("src")]);

        fs::remove_dir_all(root).unwrap();
    }
}
