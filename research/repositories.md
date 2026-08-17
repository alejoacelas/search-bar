# Repository survey

Snapshot: 2026-08-17. Stars, commits, contributors, and activity come from the GitHub API. Counts describe public activity, not code quality.

## Closest repositories

| Repository | What it already does | Public evidence | What to take |
| --- | --- | --- | --- |
| [MaxGhenis/openmessage](https://github.com/MaxGhenis/openmessage) | One local SQLite inbox for Google Messages, WhatsApp, and Signal, with full-text search and a native macOS wrapper | 50 stars; 245 default-branch commits; 5 contributors; 11 releases; pushed 2026-08-12; Unlicense | Evaluate its WhatsApp and Signal ingestion, normalized conversation model, media handling, health state, and backfill logic. It is the closest match for the hardest sources. |
| [asciimoo/hister](https://github.com/asciimoo/hister) | Indexes visited pages and watched local files; imports browser history, bookmarks, GitHub issues, and read-later archives; offers web, TUI, CLI, JSON, and MCP search | 1,898 stars; 1,878 commits; 45 contributors; 18 releases; pushed 2026-08-17; AGPL-3.0 | Benchmark its query language, stored previews, browser extension, file extractors, incremental imports, and opened-result history. It does not ingest email or private messaging. |
| [StarTrail-org/LEANN](https://github.com/StarTrail-org/LEANN) | Semantic retrieval examples for files, Apple Mail, browser history, WeChat, iMessage, ChatGPT, Claude, Slack, Twitter, and repositories | 12,784 stars; 614 commits; 45 contributors; 29 releases; pushed 2026-07-31; MIT | Reuse adapters and benchmark its compact vector index as an optional reranker. It is a retrieval toolkit and RAG surface, not a finished personal search bar. |
| [hanxiao/omni-macos](https://github.com/hanxiao/omni-macos) | Native on-device semantic search across text, PDFs, images, audio, and video on Apple silicon | 215 stars; 567 commits; 1 contributor; 110 releases; pushed 2026-08-13; Apache-2.0 | Benchmark extraction, changed-chunk indexing, memory-budgeted vectors, exact rescoring, and background-index responsiveness. Its Swift/MLX engine is not a drop-in Rust dependency. |
| [Xoshbin/asyar](https://github.com/Xoshbin/asyar) | Cross-platform Raycast-style launcher with a native file index, clipboard history, extensions, previews, actions, and latency benchmarks | 282 stars; 2,209 commits; 6 contributors; 43 releases; pushed 2026-08-17; GPL-3.0 | Copy interaction ideas and benchmark methods. Its source coverage stops well short of the requested personal archive; GPL affects direct code reuse. |
| [khoj-ai/khoj](https://github.com/khoj-ai/khoj) | Mature personal AI over PDFs, Markdown, Word, images, Notion, the web, Obsidian, desktop, phone, and WhatsApp | 36,531 stars; 5,180 commits; 73 contributors; 173 releases; pushed 2026-08-02; AGPL-3.0 | Inspect document parsing, Obsidian integration, source configuration, and hybrid semantic search. The product is organized around chat and agents rather than instant mixed-source retrieval. |

## Narrow but useful

| Repository | Evidence | Use or dismissal |
| --- | --- | --- |
| [screenpipe/screenpipe](https://github.com/screenpipe/screenpipe) | 20,999 stars; 12,418 commits; 148 contributors; pushed 2026-08-17; custom source-available commercial license | An optional searchable record of what appeared on screen or in audio. It loses native message metadata and exact source completeness, and its code is no longer open source under an OSI license. Integrate through its API, if at all. |
| [openrecall/openrecall](https://github.com/openrecall/openrecall) | 2,928 stars; 90 commits; 3 contributors; last push 2025-09-24; AGPL-3.0 | Screenshot/OCR memory only. Screenpipe is more active; neither replaces source connectors. |
| [samuelmeseret/memsearch](https://github.com/samuelmeseret/memsearch) | 1 star; 15 commits; 1 contributor; last push 2026-05-12; MIT | Concrete Electron, Raycast, ChromaDB, PhotoKit, PDF, and image pipeline. Useful as a small spike, not a dependency: it requires a Gemini key, is not notarized, and has little independent use evidence. |
| [moyangzhan/mango-finder](https://github.com/moyangzhan/mango-finder) | 250 stars; 71 commits; 1 contributor; last push 2026-05-28; no detected license | Natural-language file search across machines. No license means its code cannot be reused safely without permission. |
| [spyglass-search/spyglass](https://github.com/spyglass-search/spyglass) | 2,723 stars; 803 commits; 12 contributors; archived; last push 2025-06-27; AGPL-3.0 | Useful historical design for a personal search engine, but not an active base. Hister is the current comparison. |
| [can1357/notionfs](https://github.com/can1357/notionfs) | 17 stars; 4 commits; 1 contributor; last push 2026-01-19; MIT | A clean way to make selected Notion pages ordinary Markdown, but too new to trust as the sole copy or sync path. |

## The Oh My Pi lead

[can1357/obsidian-agent](https://github.com/can1357/obsidian-agent) is probably the Obsidian project that prompted this search. It is a fork of [logancyang/obsidian-copilot](https://github.com/logancyang/obsidian-copilot), not a general-purpose search bar. It removes the upstream cloud-service layer and feature gates, then provides vault and web search inside Obsidian. Its 875 commits and 42 contributors are largely inherited history, so they do not measure the fork's maturity.

Two other repositories from the same profile are informative but outside the product boundary:

- [can1357/smgrep](https://github.com/can1357/smgrep) is local semantic code search built for coding agents. It is a useful indexing experiment, not personal search.
- [can1357/notionfs](https://github.com/can1357/notionfs) could turn Notion into an ordinary file connector.

## Recommended system boundary

Keep the current normalized-document and local-replica design:

```text
files ───────────── existing watcher/extractors ─┐
browser history ── Hister-derived collector ─────┤
Gmail ──────────── official API collector ───────┤
WhatsApp/Signal ── audited OpenMessage adapter ──┼→ normalized records → local lexical index → Search Bar
other sources ──── LEANN-derived importers ──────┘                         └→ optional semantic reranker
```

This separates three decisions that the surveyed products often combine:

- Collection determines completeness and breakage risk per source.
- Retrieval determines literal, fuzzy, and semantic recall.
- The launcher determines latency, ranking, actions, and trust.

## Adoption gates

- Do not replace the current lexical index until a candidate beats it on a fixed corpus of real queries and stays under the existing 50 ms p95 target.
- Do not connect a write-capable messaging bridge until sending paths are removed or denied at the process boundary and disposable-account tests show no writes.
- Do not vendor AGPL or GPL code until the repository has an explicit compatible license. APIs, subprocess boundaries, and reimplementation from documented behavior still need legal review before distribution.
- Do not send private content to an embeddings API by default. Omni and LEANN offer local paths; Memsearch does not.
- Prefer projects with at least two active maintainers for a durable dependency. OpenMessage and Omni need a fork or replacement plan before they become critical infrastructure.

## First benchmark

Build a 100-query set from things actually searched for: exact filenames, remembered phrases, people plus dates, concepts without exact words, typos, and ambiguous cross-source terms. Record expected useful results, then compare:

1. current SQLite FTS5;
2. Hister lexical search;
3. FTS5 candidates reranked by LEANN;
4. Omni for files only.

Measure index time, incremental-update delay, disk, idle memory, warm and cold p50/p95 latency, top-1 success, and useful result in the top five. The recommendation changes only when a candidate improves top-five success without exceeding 50 ms warm p95 or adding an ingestion failure mode the source cannot recover from.
