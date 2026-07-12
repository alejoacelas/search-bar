use ignore::WalkBuilder;
use serde::Serialize;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    path: String,
    name: String,
    parent: String,
    line: Option<usize>,
    preview: Option<String>,
    kind: &'static str,
}

#[tauri::command]
fn home_directory() -> String {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .to_string_lossy()
        .into_owned()
}

fn display_parent(path: &Path, root: &Path) -> String {
    path.parent()
        .and_then(|p| p.strip_prefix(root).ok())
        .map(|p| {
            let value = p.to_string_lossy();
            if value.is_empty() {
                ".".into()
            } else {
                value.into_owned()
            }
        })
        .unwrap_or_else(|| path.parent().unwrap_or(root).to_string_lossy().into_owned())
}

#[tauri::command]
async fn search_files(
    root: String,
    query: String,
    limit: usize,
) -> Result<Vec<SearchResult>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        search_files_sync(PathBuf::from(root), &query, limit)
    })
    .await
    .map_err(|error| error.to_string())?
}

fn search_files_sync(
    root_path: PathBuf,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, String> {
    let needle = query.to_lowercase();
    let mut name_hits = Vec::new();
    let mut content_hits = Vec::new();
    let walker = WalkBuilder::new(&root_path)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .follow_links(false)
        .build();

    for entry in walker.filter_map(Result::ok) {
        if name_hits.len() + content_hits.len() >= limit {
            break;
        }
        let path = entry.path();
        if !entry.file_type().is_some_and(|kind| kind.is_file()) {
            continue;
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let parent = display_parent(path, &root_path);
        let path_string = path.to_string_lossy().into_owned();

        if name.to_lowercase().contains(&needle) {
            name_hits.push(SearchResult {
                path: path_string.clone(),
                name: name.clone(),
                parent: parent.clone(),
                line: None,
                preview: None,
                kind: "name",
            });
        }

        let Ok(file) = File::open(path) else { continue };
        for (index, line) in BufReader::new(file).lines().enumerate() {
            let Ok(line) = line else { break };
            if line.contains('\0') {
                break;
            }
            if line.to_lowercase().contains(&needle) {
                let preview: String = line.trim().chars().take(180).collect();
                content_hits.push(SearchResult {
                    path: path_string,
                    name,
                    parent,
                    line: Some(index + 1),
                    preview: Some(preview),
                    kind: "content",
                });
                break;
            }
            if index > 20_000 {
                break;
            }
        }
    }
    name_hits.extend(content_hits);
    name_hits.truncate(limit);
    Ok(name_hits)
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    open::that(path).map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            home_directory,
            search_files,
            open_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running Search Bar");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn finds_names_before_contents() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("needle-notes.md"), "nothing here").unwrap();
        fs::write(root.path().join("other.md"), "the needle is here").unwrap();

        let results = search_files_sync(root.path().into(), "needle", 10).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].kind, "name");
        assert_eq!(results[1].kind, "content");
        assert_eq!(results[1].line, Some(1));
    }

    #[test]
    fn respects_the_result_limit() {
        let root = tempfile::tempdir().unwrap();
        for index in 0..5 {
            fs::write(root.path().join(format!("match-{index}.txt")), "match").unwrap();
        }
        assert_eq!(
            search_files_sync(root.path().into(), "match", 3)
                .unwrap()
                .len(),
            3
        );
    }
}
