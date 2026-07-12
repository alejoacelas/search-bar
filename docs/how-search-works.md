<!--ai-->
# How search works

Search Bar builds private SQLite FTS5 indexes on this computer, then searches them as I type. It does not rescan every file, contact the collector, or send contents anywhere for a query.

Connected sources use a second FTS table backed by normalized documents. The collector supplies incremental changes in the background; Search Bar applies them to its local replica every ten seconds. If the collector is offline, the last replica remains searchable.

## What gets indexed

On launch or after changing the search folder, the app walks that folder using ripgrep's `ignore` library. It respects `.gitignore`, global Git excludes, and repository excludes. It skips hidden folders, symlinks, unreadable files, files over 2 MiB, and formats outside this plain-text list:

`c cc cpp css csv go h hpp html ini java js json jsx log md mdx mjs py rb rs rtf sh sql swift toml ts tsx txt xml yaml yml zsh`

For each file, it stores four fields:

- Full path, used to open the result but not to rank it.
- File name, indexed with high ranking weight.
- Text content, indexed with normal ranking weight.
- Parent path, displayed but not searched or ranked.

The refresh runs in one database transaction. Existing results remain searchable while a same-folder refresh runs; the new index appears all at once when the transaction commits. A first-time folder has no results until its first index finishes.

Each connected item stores its source identifier, kind, title, body, people, timestamp, native open target, default copy text, and source-specific metadata. The first connector is WhatsApp; the same schema is intended for Gmail, Slack, Telegram, notes, and later connectors.

## What a query means

The app splits the query on punctuation and whitespace, removes empty pieces, and requires every remaining token. Each token is a prefix.

For example:

```text
project meet
```

becomes approximately:

```text
"project"* AND "meet"*
```

This matches `project meeting`, `projects meetup`, and `project-meeting.md`. It does not match a typo such as `proejct`, a synonym such as `work`, or a fragment from the middle of a token such as `oject`.

SQLite's `unicode61` tokenizer makes matching case-insensitive, splits on Unicode punctuation and spaces, and treats most Latin diacritics as equivalent. `Résumé`, `résumé`, and `resume` therefore match one another. See the [SQLite FTS5 tokenizer and prefix-query documentation](https://www.sqlite.org/fts5.html).

## How results rank

File ranking has two stages:

1. A case-insensitive substring match in the file name always goes above a content-only match.
2. Within those groups, SQLite's BM25 relevance score orders the results. The file-name field has weight `8`; content has weight `1`; paths have weight `0`.

BM25 generally favors files where:

- More query terms occur.
- Terms occur more often without the document being disproportionately long.
- A matched term is rarer across the indexed files.
- The match is in the more heavily weighted file-name field.

The app retrieves file and connected-item candidates, puts title matches first, then compares each index's BM25 score and timestamp. It returns at most 80 results. Each content result shows an FTS5-generated excerpt of roughly 18 tokens around a strong match. It currently returns one result per file and one result per connected message, not one result per matching line or conversation.

## How good is it?

It is fast and predictable for finding a plain-text file when I remember part of its name or wording. It is not yet forgiving or semantic.

| System | Knows | Better than Search Bar at | Search Bar's advantage |
| --- | --- | --- | --- |
| Search Bar today | File names plus normalized connected-source titles, people, and literal text tokens | — | Local queries remain private and fast after indexing |
| [Typesense](https://typesense.org/docs/latest/api/search.html) | Indexed fields, token similarity, configurable relevance | Typo tolerance, token dropping, field rules, infix options, and relevance tuning | No separate search-server process, API key, or service lifecycle |
| [VS Code IntelliSense](https://code.visualstudio.com/docs/editing/intellisense) | Programming-language syntax, types, symbols, imports, and cursor context | Code completion and semantic navigation | Searches arbitrary supported text files instead of understanding one code workspace |
| ripgrep | Current bytes on disk | Exact live results, regex, no index delay | Search-as-you-type ranking without rescanning the folder |
| macOS Spotlight | OS metadata and extracted content across many formats | PDFs, Office files, email, and system-wide metadata | Explicit search scope and behavior we control end to end |

IntelliSense is not a stronger version of this search. It is a different category: a language service analyzes code semantics to suggest valid functions, variables, and properties at the cursor. Search Bar retrieves documents from remembered words. Typesense is the closer comparison.

## Important gaps

- No typo tolerance, synonyms, stemming, semantic/vector search, recency boost, or usage-based ranking.
- No PDF, Word, image OCR, archive, or files over 2 MiB.
- No live filesystem watcher. The file index refreshes when the app launches or the folder changes.
- Deleted or changed files can remain stale until that refresh commits.
- Cross-index BM25 values are only approximately comparable. Ranking needs fixtures based on real mixed-source queries.
- Connected sources are only as complete as their adapters. Search Bar cannot infer records a provider never supplied.
- Prefix matching works from the start of a token, not the middle.
- Ranking has not yet been tuned against a real set of searches and clicked results.

The next useful ranking improvement is likely a small local history of opened results. It could boost files I open frequently or recently without changing the search backend. If misspellings become the main failure mode, Typesense or a lightweight fuzzy fallback over file names would be the next comparison to test.
<!--/ai-->
