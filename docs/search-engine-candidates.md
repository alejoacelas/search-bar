<!--ai-->
# Search engine candidates

## Decision

Start index-free with ripgrep's filesystem traversal rules, implemented through its `ignore` Rust crate, and a small literal line matcher.

This gives the first version the useful parts of ripgrep's behavior: recursive walking, `.gitignore` support, hidden-file control, and no index to configure or keep fresh. The app searches file names first, then readable text content, stops after 80 results, and skips unreadable or binary-looking files.

The matcher is intentionally behind one Tauri command. We can replace it with an index without changing the interface.

## Candidates

| Candidate | Best at | Costs | Verdict |
| --- | --- | --- | --- |
| [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md) | Immediate literal or regex search over live files; respects common ignore rules and skips binary files | Re-reads files for every query; relevance and typo tolerance must be added | Pick now. It is local, dependable, and needs no indexing lifecycle. |
| [SQLite FTS5](https://www.sqlite.org/fts5.html) | Embedded indexed search with ranking, phrase queries, and one portable database | We must build and maintain a file watcher, extraction pipeline, schema, and index recovery | Best likely upgrade when repeated searches over a large stable corpus become slow. |
| [Tantivy](https://docs.rs/tantivy/latest/tantivy/) | High-performance Lucene-style indexing in native Rust | More index machinery and schema design than this MVP needs | Strong option if ranking and corpus size outgrow SQLite. |
| [Meilisearch](https://www.meilisearch.com/docs/capabilities/full_text_search/overview) | Typo tolerance, prefix search, and polished relevance defaults | Adds a separate service and its operational lifecycle to a personal desktop utility | Excellent search UX, wrong deployment shape for the first version. |

## When to revisit

Measure before replacing the backend. Move to SQLite FTS5 when a representative home-folder query cannot reliably return useful results within 200 ms, or when typo tolerance, phrase ranking, document metadata, or non-plain-text extraction becomes more valuable than zero-maintenance freshness.

The next backend should preserve the current `search_files(root, query, limit)` command contract, add cancellation for superseded queries, and index only user-approved roots.
<!--/ai-->
