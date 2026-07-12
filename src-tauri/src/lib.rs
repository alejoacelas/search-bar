use ignore::WalkBuilder;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

const MAX_FILE_BYTES: u64 = 2 * 1024 * 1024;
const TEXT_EXTENSIONS: &[&str] = &[
    "c", "cc", "cpp", "css", "csv", "go", "h", "hpp", "html", "ini", "java", "js", "json", "jsx",
    "log", "md", "mdx", "mjs", "py", "rb", "rs", "rtf", "sh", "sql", "swift", "toml", "ts", "tsx",
    "txt", "xml", "yaml", "yml", "zsh",
];

struct AppState {
    database: PathBuf,
    indexing: AtomicBool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    path: String,
    name: String,
    parent: String,
    line: Option<usize>,
    preview: Option<String>,
    kind: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IndexReport {
    files: usize,
    elapsed_ms: u128,
}

fn open_database(path: &Path) -> Result<Connection, String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let connection = Connection::open(path).map_err(|error| error.to_string())?;
    connection
        .execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
             CREATE VIRTUAL TABLE IF NOT EXISTS files USING fts5(
               path UNINDEXED, name, content, parent UNINDEXED,
               tokenize='unicode61 remove_diacritics 2'
             );",
        )
        .map_err(|error| error.to_string())?;
    Ok(connection)
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
        .and_then(|parent| parent.strip_prefix(root).ok())
        .map(|parent| {
            let value = parent.to_string_lossy();
            if value.is_empty() {
                ".".into()
            } else {
                value.into_owned()
            }
        })
        .unwrap_or_else(|| path.parent().unwrap_or(root).to_string_lossy().into_owned())
}

fn read_text(path: &Path) -> Option<String> {
    let metadata = path.metadata().ok()?;
    if metadata.len() > MAX_FILE_BYTES {
        return None;
    }
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();
    if !TEXT_EXTENSIONS.contains(&extension.as_str()) {
        return None;
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    File::open(path).ok()?.read_to_end(&mut bytes).ok()?;
    if bytes.iter().take(8_192).any(|byte| *byte == 0) {
        return None;
    }
    String::from_utf8(bytes).ok()
}

fn rebuild_index(database: &Path, root: &Path) -> Result<IndexReport, String> {
    let started = Instant::now();
    let mut connection = open_database(database)?;
    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    transaction
        .execute("DELETE FROM files", [])
        .map_err(|error| error.to_string())?;
    let mut statement = transaction
        .prepare("INSERT INTO files(path, name, content, parent) VALUES (?1, ?2, ?3, ?4)")
        .map_err(|error| error.to_string())?;
    let mut files = 0;
    let walker = WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .follow_links(false)
        .build();

    for entry in walker.filter_map(Result::ok) {
        if !entry.file_type().is_some_and(|kind| kind.is_file()) {
            continue;
        }
        let path = entry.path();
        let Some(content) = read_text(path) else {
            continue;
        };
        let name = entry.file_name().to_string_lossy();
        let parent = display_parent(path, root);
        statement
            .execute(params![path.to_string_lossy(), name, content, parent])
            .map_err(|error| error.to_string())?;
        files += 1;
    }
    drop(statement);
    transaction
        .execute(
            "INSERT OR REPLACE INTO settings(key, value) VALUES ('root', ?1)",
            [root.to_string_lossy()],
        )
        .map_err(|error| error.to_string())?;
    transaction.commit().map_err(|error| error.to_string())?;
    Ok(IndexReport {
        files,
        elapsed_ms: started.elapsed().as_millis(),
    })
}

#[tauri::command]
async fn prepare_index(
    root: String,
    state: tauri::State<'_, AppState>,
) -> Result<IndexReport, String> {
    if state.indexing.swap(true, Ordering::SeqCst) {
        return Err("An index refresh is already running".into());
    }
    let database = state.database.clone();
    let result =
        tauri::async_runtime::spawn_blocking(move || rebuild_index(&database, Path::new(&root)))
            .await
            .map_err(|error| error.to_string())?;
    state.indexing.store(false, Ordering::SeqCst);
    result
}

fn fts_query(query: &str) -> String {
    query
        .split(|character: char| !character.is_alphanumeric())
        .filter(|token| !token.is_empty())
        .map(|token| format!("\"{}\"*", token.replace('"', "\"\"")))
        .collect::<Vec<_>>()
        .join(" AND ")
}

fn search_index(
    database: &Path,
    root: &str,
    query: &str,
    limit: usize,
) -> Result<Vec<SearchResult>, String> {
    let expression = fts_query(query);
    if expression.is_empty() {
        return Ok(Vec::new());
    }
    let connection = open_database(database)?;
    let indexed_root =
        connection.query_row("SELECT value FROM settings WHERE key = 'root'", [], |row| {
            row.get::<_, String>(0)
        });
    if indexed_root.ok().as_deref() != Some(root) {
        return Ok(Vec::new());
    }
    let mut statement = connection
        .prepare(
            "SELECT path, name, parent,
                    snippet(files, 2, '', '', ' … ', 18),
                    CASE WHEN instr(lower(name), lower(?2)) > 0 THEN 'name' ELSE 'content' END
             FROM files
             WHERE files MATCH ?1
             ORDER BY CASE WHEN instr(lower(name), lower(?2)) > 0 THEN 0 ELSE 1 END,
                      bm25(files, 0.0, 8.0, 1.0, 0.0)
             LIMIT ?3",
        )
        .map_err(|error| error.to_string())?;
    let rows = statement
        .query_map(params![expression, query, limit as i64], |row| {
            let kind: String = row.get(4)?;
            Ok(SearchResult {
                path: row.get(0)?,
                name: row.get(1)?,
                parent: row.get(2)?,
                line: None,
                preview: row.get(3)?,
                kind: if kind == "name" { "name" } else { "content" },
            })
        })
        .map_err(|error| error.to_string())?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn search_files(
    root: String,
    query: String,
    limit: usize,
    state: tauri::State<'_, AppState>,
) -> Result<Vec<SearchResult>, String> {
    let database = state.database.clone();
    tauri::async_runtime::spawn_blocking(move || search_index(&database, &root, &query, limit))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
fn open_file(path: String) -> Result<(), String> {
    open::that(path).map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database = dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("search-bar")
        .join("files.sqlite3");
    tauri::Builder::default()
        .manage(AppState {
            database,
            indexing: AtomicBool::new(false),
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            home_directory,
            prepare_index,
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
    fn indexes_and_searches_names_and_contents() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join("index.sqlite3");
        fs::write(root.path().join("needle-notes.md"), "nothing here").unwrap();
        fs::write(root.path().join("other.md"), "the needle is here").unwrap();

        let report = rebuild_index(&database, root.path()).unwrap();
        let results =
            search_index(&database, &root.path().to_string_lossy(), "needle", 10).unwrap();
        assert_eq!(report.files, 2);
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].kind, "name");
        assert_eq!(results[1].kind, "content");
    }

    #[test]
    fn repeated_queries_are_fast() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join("index.sqlite3");
        for index in 0..2_000 {
            fs::write(
                root.path().join(format!("note-{index}.txt")),
                format!("a searchable document with token-{index}"),
            )
            .unwrap();
        }
        rebuild_index(&database, root.path()).unwrap();
        let started = Instant::now();
        for _ in 0..100 {
            assert_eq!(
                search_index(&database, &root.path().to_string_lossy(), "searchable", 80,)
                    .unwrap()
                    .len(),
                80
            );
        }
        assert!(started.elapsed().as_millis() < 1_000);
    }
}
