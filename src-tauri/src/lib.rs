use ignore::WalkBuilder;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
    sync::Mutex,
    time::Duration,
    time::Instant,
};
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

const MAX_FILE_BYTES: u64 = 2 * 1024 * 1024;
const TEXT_EXTENSIONS: &[&str] = &[
    "c", "cc", "cpp", "css", "csv", "go", "h", "hpp", "html", "ini", "java", "js", "json", "jsx",
    "log", "md", "mdx", "mjs", "py", "rb", "rs", "rtf", "sh", "sql", "swift", "toml", "ts", "tsx",
    "txt", "xml", "yaml", "yml", "zsh",
];

struct AppState {
    database: PathBuf,
    indexing: AtomicBool,
    syncing: AtomicBool,
    collector: Option<CollectorConfig>,
    collector_error: Mutex<Option<String>>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SearchResult {
    id: String,
    source: String,
    kind: String,
    title: String,
    subtitle: String,
    preview: Option<String>,
    open_target: Option<String>,
    copy_text: String,
    occurred_at: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IndexReport {
    files: usize,
    elapsed_ms: u128,
}

#[derive(Clone, Deserialize)]
struct CollectorConfig {
    url: String,
    token: String,
}

#[derive(Deserialize)]
struct ChangeFeed {
    cursor: i64,
    has_more: bool,
    changes: Vec<RemoteChange>,
}

#[derive(Deserialize)]
struct RemoteChange {
    operation: String,
    source: String,
    source_id: String,
    kind: String,
    title: String,
    body: String,
    people: String,
    occurred_at: i64,
    open_target: Option<String>,
    copy_text: String,
    metadata_json: String,
    updated_at: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SyncReport {
    configured: bool,
    imported: usize,
    cursor: i64,
    documents: usize,
    error: Option<String>,
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
             );
             CREATE TABLE IF NOT EXISTS documents (
               source TEXT NOT NULL,
               source_id TEXT NOT NULL,
               kind TEXT NOT NULL,
               title TEXT NOT NULL,
               body TEXT NOT NULL,
               people TEXT NOT NULL,
               occurred_at INTEGER NOT NULL,
               open_target TEXT,
               copy_text TEXT NOT NULL,
               metadata_json TEXT NOT NULL,
               remote_updated_at INTEGER NOT NULL,
               PRIMARY KEY(source, source_id)
             );
             CREATE VIRTUAL TABLE IF NOT EXISTS documents_fts USING fts5(
               title, body, people,
               content='documents', content_rowid='rowid',
               tokenize='unicode61 remove_diacritics 2'
             );
             CREATE TRIGGER IF NOT EXISTS documents_ai AFTER INSERT ON documents BEGIN
               INSERT INTO documents_fts(rowid, title, body, people)
               VALUES (new.rowid, new.title, new.body, new.people);
             END;
             CREATE TRIGGER IF NOT EXISTS documents_ad AFTER DELETE ON documents BEGIN
               INSERT INTO documents_fts(documents_fts, rowid, title, body, people)
               VALUES ('delete', old.rowid, old.title, old.body, old.people);
             END;
             CREATE TRIGGER IF NOT EXISTS documents_au AFTER UPDATE ON documents BEGIN
               INSERT INTO documents_fts(documents_fts, rowid, title, body, people)
               VALUES ('delete', old.rowid, old.title, old.body, old.people);
               INSERT INTO documents_fts(rowid, title, body, people)
               VALUES (new.rowid, new.title, new.body, new.people);
             END;",
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

struct RankedResult {
    result: SearchResult,
    title_match: bool,
    rank: f64,
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
    let mut file_statement = connection
        .prepare(
            "SELECT path, name, parent,
                    snippet(files, 2, '', '', ' … ', 18),
                    instr(lower(name), lower(?2)) > 0,
                    bm25(files, 0.0, 8.0, 1.0, 0.0)
             FROM files
             WHERE files MATCH ?1
             LIMIT ?3",
        )
        .map_err(|error| error.to_string())?;
    let file_rows = file_statement
        .query_map(params![expression, query, limit as i64], |row| {
            let path: String = row.get(0)?;
            let title_match: bool = row.get(4)?;
            Ok(RankedResult {
                result: SearchResult {
                    id: format!("file:{path}"),
                    source: "file".into(),
                    kind: "file".into(),
                    title: row.get(1)?,
                    subtitle: row.get(2)?,
                    preview: row.get(3)?,
                    open_target: Some(path.clone()),
                    copy_text: path,
                    occurred_at: None,
                },
                title_match,
                rank: row.get(5)?,
            })
        })
        .map_err(|error| error.to_string())?;
    let mut ranked = file_rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;

    let mut document_statement = connection
        .prepare(
            "SELECT d.source, d.source_id, d.kind, d.title, d.people, d.occurred_at,
                    d.open_target, d.copy_text,
                    snippet(documents_fts, 1, '', '', ' … ', 18),
                    instr(lower(d.title), lower(?2)) > 0,
                    bm25(documents_fts, 8.0, 1.0, 3.0)
             FROM documents_fts JOIN documents d ON d.rowid = documents_fts.rowid
             WHERE documents_fts MATCH ?1 LIMIT ?3",
        )
        .map_err(|error| error.to_string())?;
    let document_rows = document_statement
        .query_map(params![fts_query(query), query, limit as i64], |row| {
            let source: String = row.get(0)?;
            let source_id: String = row.get(1)?;
            let people: String = row.get(4)?;
            Ok(RankedResult {
                result: SearchResult {
                    id: format!("{source}:{source_id}"),
                    source,
                    kind: row.get(2)?,
                    title: row.get(3)?,
                    subtitle: people,
                    occurred_at: Some(row.get(5)?),
                    open_target: row.get(6)?,
                    copy_text: row.get(7)?,
                    preview: row.get(8)?,
                },
                title_match: row.get(9)?,
                rank: row.get(10)?,
            })
        })
        .map_err(|error| error.to_string())?;
    ranked.extend(
        document_rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?,
    );
    ranked.sort_by(|left, right| {
        right
            .title_match
            .cmp(&left.title_match)
            .then_with(|| left.rank.total_cmp(&right.rank))
            .then_with(|| right.result.occurred_at.cmp(&left.result.occurred_at))
    });
    Ok(ranked
        .into_iter()
        .take(limit)
        .map(|ranked| ranked.result)
        .collect())
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
fn open_result(target: String) -> Result<(), String> {
    open::that(target).map_err(|error| error.to_string())
}

fn collector_cursor(connection: &Connection) -> i64 {
    connection
        .query_row(
            "SELECT value FROM settings WHERE key = 'collector_cursor'",
            [],
            |row| row.get::<_, String>(0),
        )
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}

fn sync_from_collector(database: &Path, config: &CollectorConfig) -> Result<SyncReport, String> {
    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(8))
        .build()
        .map_err(|error| error.to_string())?;
    let mut connection = open_database(database)?;
    let mut cursor = collector_cursor(&connection);
    let mut imported = 0;
    loop {
        let endpoint = format!(
            "{}/v1/changes?after={cursor}&limit=500",
            config.url.trim_end_matches('/')
        );
        let response = client
            .get(endpoint)
            .bearer_auth(&config.token)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| format!("Collector sync failed: {error}"))?;
        let feed: ChangeFeed = response.json().map_err(|error| error.to_string())?;
        let transaction = connection
            .transaction()
            .map_err(|error| error.to_string())?;
        for change in feed.changes {
            if change.operation == "delete" {
                transaction
                    .execute(
                        "DELETE FROM documents WHERE source = ?1 AND source_id = ?2",
                        params![change.source, change.source_id],
                    )
                    .map_err(|error| error.to_string())?;
            } else {
                transaction
                    .execute(
                        "INSERT INTO documents(
                           source, source_id, kind, title, body, people, occurred_at,
                           open_target, copy_text, metadata_json, remote_updated_at
                         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                         ON CONFLICT(source, source_id) DO UPDATE SET
                           kind=excluded.kind, title=excluded.title, body=excluded.body,
                           people=excluded.people, occurred_at=excluded.occurred_at,
                           open_target=excluded.open_target, copy_text=excluded.copy_text,
                           metadata_json=excluded.metadata_json,
                           remote_updated_at=excluded.remote_updated_at",
                        params![
                            change.source,
                            change.source_id,
                            change.kind,
                            change.title,
                            change.body,
                            change.people,
                            change.occurred_at,
                            change.open_target,
                            change.copy_text,
                            change.metadata_json,
                            change.updated_at,
                        ],
                    )
                    .map_err(|error| error.to_string())?;
            }
            imported += 1;
        }
        cursor = feed.cursor;
        transaction
            .execute(
                "INSERT OR REPLACE INTO settings(key, value) VALUES ('collector_cursor', ?1)",
                [cursor.to_string()],
            )
            .map_err(|error| error.to_string())?;
        transaction.commit().map_err(|error| error.to_string())?;
        if !feed.has_more {
            break;
        }
    }
    let documents = connection
        .query_row("SELECT COUNT(*) FROM documents", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;
    Ok(SyncReport {
        configured: true,
        imported,
        cursor,
        documents,
        error: None,
    })
}

#[tauri::command]
async fn sync_collector(state: tauri::State<'_, AppState>) -> Result<SyncReport, String> {
    let Some(config) = state.collector.clone() else {
        return Ok(SyncReport {
            configured: false,
            imported: 0,
            cursor: 0,
            documents: 0,
            error: None,
        });
    };
    if state.syncing.swap(true, Ordering::SeqCst) {
        return Err("A collector sync is already running".into());
    }
    let database = state.database.clone();
    let result =
        tauri::async_runtime::spawn_blocking(move || sync_from_collector(&database, &config))
            .await
            .map_err(|error| error.to_string())?;
    state.syncing.store(false, Ordering::SeqCst);
    match &result {
        Ok(_) => *state.collector_error.lock().unwrap() = None,
        Err(error) => *state.collector_error.lock().unwrap() = Some(error.clone()),
    }
    result
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let data_directory = dirs::data_local_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("search-bar");
    let database = data_directory.join("files.sqlite3");
    let saved_collector = std::fs::read_to_string(data_directory.join("collector.json"))
        .ok()
        .and_then(|value| serde_json::from_str::<CollectorConfig>(&value).ok());
    let collector = match (
        std::env::var("SEARCH_BAR_COLLECTOR_URL").ok(),
        std::env::var("SEARCH_BAR_COLLECTOR_TOKEN").ok(),
    ) {
        (Some(url), Some(token)) if !url.is_empty() && !token.is_empty() => {
            Some(CollectorConfig { url, token })
        }
        _ => saved_collector,
    };
    tauri::Builder::default()
        .manage(AppState {
            database,
            indexing: AtomicBool::new(false),
            syncing: AtomicBool::new(false),
            collector,
            collector_error: Mutex::new(None),
        })
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    let Some(window) = app.get_webview_window("main") else {
                        return;
                    };
                    if window.is_visible().unwrap_or(false) && window.is_focused().unwrap_or(false)
                    {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.global_shortcut().register("CmdOrCtrl+Space")?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            home_directory,
            prepare_index,
            search_files,
            open_result,
            sync_collector
        ])
        .run(tauri::generate_context!())
        .expect("error while running Search Bar");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        io::{BufRead, BufReader, Write},
        net::TcpListener,
        thread,
    };

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
        assert_eq!(results[0].kind, "file");
        assert_eq!(results[1].kind, "file");
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

    #[test]
    fn searches_connected_documents_with_files() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join("index.sqlite3");
        fs::write(root.path().join("local.txt"), "local only").unwrap();
        rebuild_index(&database, root.path()).unwrap();
        let connection = open_database(&database).unwrap();
        connection
            .execute(
                "INSERT INTO documents(
               source, source_id, kind, title, body, people, occurred_at,
               open_target, copy_text, metadata_json, remote_updated_at
             ) VALUES ('whatsapp', 'chat:message', 'message', 'Ada',
                       'meeting at five', 'Ada', 1, 'whatsapp://',
                       'meeting at five', '{}', 1)",
                [],
            )
            .unwrap();
        let results =
            search_index(&database, &root.path().to_string_lossy(), "meeting", 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].source, "whatsapp");
        assert_eq!(results[0].title, "Ada");
    }

    #[test]
    fn synchronizes_authenticated_collector_feed() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut authorization = String::new();
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap();
                if line == "\r\n" || line.is_empty() {
                    break;
                }
                if line.to_ascii_lowercase().starts_with("authorization:") {
                    authorization = line.trim().to_string();
                }
            }
            assert_eq!(authorization, "authorization: Bearer test-token");
            let body = r#"{"cursor":1,"has_more":false,"changes":[{"operation":"upsert","source":"whatsapp","source_id":"chat:one","kind":"message","title":"Ada","body":"collector phrase","people":"Ada","occurred_at":1,"open_target":"whatsapp://","copy_text":"collector phrase","metadata_json":"{}","updated_at":1}]}"#;
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            )
            .unwrap();
        });
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join("index.sqlite3");
        rebuild_index(&database, root.path()).unwrap();
        let report = sync_from_collector(
            &database,
            &CollectorConfig {
                url: format!("http://{address}"),
                token: "test-token".into(),
            },
        )
        .unwrap();
        server.join().unwrap();
        assert_eq!(report.imported, 1);
        assert_eq!(report.cursor, 1);
        let results =
            search_index(&database, &root.path().to_string_lossy(), "collector", 10).unwrap();
        assert_eq!(results[0].source, "whatsapp");
    }
}
