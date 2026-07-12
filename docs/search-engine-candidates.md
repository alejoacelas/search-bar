<!--ai-->
# Search engine candidates

## Decision

Use embedded SQLite FTS5 for persistent ranked search, with ripgrep's filesystem traversal rules implemented through its `ignore` Rust crate for indexing.

The original index-free implementation rescanned the selected folder for every query and was too slow over a home directory. FTS5 moves that work to a background refresh and makes each typed query an indexed lookup. The app boosts file-name matches, ranks content matches, caps results at 80, and skips unreadable, binary, and oversized files.

Typesense has better typo-tolerance defaults, but requires installing or bundling and supervising a separate server. FTS5 stays inside the app process and keeps the deployment simple. Typesense remains a candidate if typo tolerance and more sophisticated ranking become central.

## Candidates

| Candidate | Best at | Costs | Verdict |
| --- | --- | --- | --- |
| [ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md) | Immediate literal or regex search over live files; respects common ignore rules and skips binary files | Re-reads files for every query; relevance and typo tolerance must be added | Useful traversal layer; too slow as the per-keystroke backend over a home folder. |
| [SQLite FTS5](https://www.sqlite.org/fts5.html) | Embedded indexed search with ranking, phrase queries, and one portable database | We must maintain an extraction and refresh lifecycle | Pick now. Fast repeated queries without another process or service. |
| [Tantivy](https://docs.rs/tantivy/latest/tantivy/) | High-performance Lucene-style indexing in native Rust | More index machinery and schema design than this MVP needs | Strong option if ranking and corpus size outgrow SQLite. |
| [Typesense](https://typesense.org/docs/latest/api/search.html) | Typo tolerance, prefix search, snippets, and sophisticated relevance controls | Adds a separately installed or bundled HTTP server, API key, and process lifecycle | Strong future option; too much deployment machinery for the current local utility. |

## When to revisit

Measure before replacing the backend again. Consider Typesense or Tantivy when typo tolerance, richer ranking, or corpus size outweighs the cost of a more complex search service.

The next backend should preserve the current `search_files(root, query, limit)` command contract, add cancellation for superseded queries, and index only user-approved roots.
<!--/ai-->
