# Repository survey

Snapshot: 2026-08-17. The expanded account scan changed the recommendation: keep Search Bar as the product shell, but stop building every collector and extractor ourselves. Mature source-specific projects now cover most inputs.

## Recommended experiment queue

1. **Email:** compare [Google Workspace CLI](https://github.com/googleworkspace/cli) for Gmail and Drive against [Himalaya](https://github.com/pimalaya/himalaya) plus [Neverest](https://github.com/pimalaya/neverest) for generic IMAP and local mailboxes.
2. **Apple data:** prototype read-only collectors with [macnotesapp](https://github.com/RhetTbull/macnotesapp), [osxphotos](https://github.com/RhetTbull/osxphotos), and [macos_mditem_metadata](https://github.com/RhetTbull/macos_mditem_metadata).
3. **Messages:** audit [OpenMessage](https://github.com/MaxGhenis/openmessage) against its underlying WhatsApp and Signal options, then compare [imsg](https://github.com/openclaw/imsg) for local Messages.app access. Remove sending at the process boundary before using any of them.
4. **Documents:** run [MarkItDown](https://github.com/microsoft/markitdown) first; add [Docling](https://github.com/docling-project/docling) only for formats or layout it handles materially better; preprocess scanned PDFs with [OCRmyPDF](https://github.com/ocrmypdf/OCRmyPDF).
5. **Browser history and saved pages:** run [Hister](https://github.com/asciimoo/hister) as a sidecar and compare its imports, query language, and previews with a small native collector.
6. **Retrieval:** keep SQLite FTS5 as the baseline. Benchmark [Tantivy](https://github.com/quickwit-oss/tantivy) for richer lexical search, [LEANN](https://github.com/StarTrail-org/LEANN) for compact semantic reranking, and [Omni](https://github.com/hanxiao/omni-macos) for multimodal files.
7. **Launcher:** compare the current window and keyboard behavior with [Sol](https://github.com/ospfranco/sol) and [Asyar](https://github.com/Xoshbin/asyar). Search Bar already has the right product shape; this is interaction research, not a rewrite.

This order tests source completeness before search sophistication. Better ranking cannot recover data a connector never collected.

## Closest integrated systems

| Repository | Existing coverage | Evidence | Recommendation |
| --- | --- | --- | --- |
| [MaxGhenis/openmessage](https://github.com/MaxGhenis/openmessage) | Local SQLite inbox for Google Messages, WhatsApp, and Signal; native macOS and web surfaces | 50 stars; pushed 2026-08-12; Unlicense | Best messaging reference. Audit carefully: it is young, has few maintainers, and includes write paths. |
| [asciimoo/hister](https://github.com/asciimoo/hister) | Visited pages, browser imports, watched local files, stored previews, web/TUI/CLI/JSON/MCP search | 1,898 stars; pushed 2026-08-17; AGPL-3.0 | Best browser-history and saved-page comparison. Prefer a sidecar or clean-room collector unless this project adopts AGPL. |
| [StarTrail-org/LEANN](https://github.com/StarTrail-org/LEANN) | Examples for files, Apple Mail, browser history, WeChat, iMessage, ChatGPT, Claude, Slack, and Twitter | 12,784 stars; pushed 2026-07-31; MIT | Best source of import patterns and compact semantic retrieval. It is a toolkit, not a search-bar product. |
| [hanxiao/omni-macos](https://github.com/hanxiao/omni-macos) | Native semantic search over text, PDFs, images, audio, and video on Apple silicon | 215 stars; pushed 2026-08-13; Apache-2.0 | Best multimodal file benchmark. Its Swift/MLX engine is not a drop-in Rust library and currently has one primary contributor. |
| [khoj-ai/khoj](https://github.com/khoj-ai/khoj) | Documents, Notion, web, Obsidian, desktop, phone, WhatsApp, semantic retrieval, and automation | 36,531 stars; pushed 2026-08-02; AGPL-3.0 | Inspect parsing and source configuration. Its core experience is chat and agents rather than instant mixed-source retrieval. |
| [deta/surf](https://github.com/deta/surf) | Local notebooks organizing files and webpages with open model choice | 3,527 stars; pushed 2026-08-17; Apache-2.0 | Useful product reference for mixing local files and web material without hiding the source objects. |
| [Mintplex-Labs/anything-llm](https://github.com/Mintplex-Labs/anything-llm) | Local-first document workspace, models, agents, and integrations | 64,834 stars; pushed 2026-08-17; MIT | Mine ingestion and setup flows; the application is much broader and heavier than a launcher. |

No integrated system in this table covers Gmail, WhatsApp, Signal, Apple data, arbitrary files, and a sub-100 ms launcher together.

## Source collectors and sync

| Source | Repository | What it contributes | Decision |
| --- | --- | --- | --- |
| Gmail, Drive, Calendar, Docs, Chat | [googleworkspace/cli](https://github.com/googleworkspace/cli) | Official Google APIs generated from Discovery documents; 30,414 stars; Apache-2.0 | First Gmail/Drive spike. Restrict the credential scopes and commands to reads. |
| Generic email | [pimalaya/himalaya](https://github.com/pimalaya/himalaya) | Rust CLI over IMAP, JMAP, Maildir, Gmail REST, Microsoft Graph, and other backends; 7,037 stars; Apache-2.0 | Best generic mail interface. Evaluate its libraries before writing another IMAP stack. |
| Generic email sync | [pimalaya/neverest](https://github.com/pimalaya/neverest) | Rust mailbox synchronization; 314 stars; AGPL-3.0 | Useful behavior and testing reference; license complicates embedding. |
| Gmail only | [badlogic/gmcli](https://github.com/badlogic/gmcli) | Minimal Gmail CLI | Excellent small code-reading reference; too small to make a critical dependency. |
| Gmail and iCloud | [kitlangton/mail-control](https://github.com/kitlangton/mail-control) | Multi-account search and actions | Useful interface ideas; small and currently lacks detected license metadata. |
| Mailbox snapshots | [simonw/mbox-to-sqlite](https://github.com/simonw/mbox-to-sqlite) | Loads `.mbox` archives into SQLite; Apache-2.0 | Good import path for exports and test fixtures, not live sync. |
| Apple Notes | [RhetTbull/macnotesapp](https://github.com/RhetTbull/macnotesapp) | CLI and Python interface over Notes.app; 270 stars; MIT | First Apple Notes spike. It uses supported app automation rather than private database parsing. |
| Apple Notes | [RhetTbull/apple-notes-parser](https://github.com/RhetTbull/apple-notes-parser) | Reads the private Notes database | Code-reading reference only: no detected license and private schemas can change. |
| Apple Photos | [RhetTbull/osxphotos](https://github.com/RhetTbull/osxphotos) | Mature Python API and CLI for Photos metadata and exports; 3,763 stars; MIT | Strong collector candidate for photos, people, albums, dates, and locations. |
| Spotlight metadata | [RhetTbull/macos_mditem_metadata](https://github.com/RhetTbull/macos_mditem_metadata) | Python access and key catalog for `MDItem` metadata; Unlicense | Borrow the metadata map; call native APIs directly from the app where practical. |
| iMessage/SMS | [openclaw/imsg](https://github.com/openclaw/imsg) | Messages.app CLI; 1,281 stars; MIT | Audit as the main local Messages candidate. Disable its sending commands. |
| iMessage/SMS | [wolfiesch/wolfies-imessage-gateway](https://github.com/wolfiesch/wolfies-imessage-gateway) | Small macOS iMessage MCP server; MIT | Useful small comparison, not a durable dependency. |
| WhatsApp | [tulir/whatsmeow](https://github.com/tulir/whatsmeow) | Go implementation of WhatsApp's multi-device protocol; 7,057 stars; MPL-2.0 | The underlying library to understand and pin. Unsupported protocol risk remains. |
| WhatsApp | [lharries/whatsapp-mcp](https://github.com/lharries/whatsapp-mcp) | WhatsApp bridge and MCP surface; 6,161 stars; MIT | Existing collector base. Upstream has not pushed since 2025-07; keep the compatibility pin and read-only patch. |
| Signal | [AsamK/signal-cli](https://github.com/AsamK/signal-cli) | Unofficial Signal JSON-RPC/DBus/CLI; 4,870 stars; GPL-3.0 | Best maintained public protocol bridge found. Run out of process and expose a read-only adapter. |
| Notion | [can1357/notionfs](https://github.com/can1357/notionfs) | Syncs selected Notion pages to Markdown; MIT | Promising, but only four default-branch commits at the first snapshot. Keep an export fallback. |
| Email to Notion | [andylizf/inboard](https://github.com/andylizf/inboard) | Self-hosted inbox agent producing a Notion board; MIT | Mine source normalization and action ideas; not a general email archive. |
| Obsidian | [louis030195/easy-obsidian-mcp](https://github.com/louis030195/easy-obsidian-mcp) | Obsidian MCP access | Behavior reference only until a license is present. Direct vault indexing is simpler for Search Bar. |

## Document, media, and web extraction

| Repository | Formats and role | Decision |
| --- | --- | --- |
| [microsoft/markitdown](https://github.com/microsoft/markitdown) | Office files, PDFs, images, audio, HTML, archives, and other formats to Markdown; 174,269 stars; MIT | First general extractor to integrate or call as a worker. Its simple Markdown output fits the normalized-document schema. |
| [docling-project/docling](https://github.com/docling-project/docling) | Layout-aware PDF, Office, image, table, formula, and OCR extraction; 64,949 stars; MIT | Use when MarkItDown loses important layout or tables. Heavier deployment must earn its cost on fixtures. |
| [ocrmypdf/OCRmyPDF](https://github.com/ocrmypdf/OCRmyPDF) | Adds searchable text layers to scanned PDFs; 34,471 stars; MPL-2.0 | Best preprocessing candidate for scanned PDFs. Preserve the original file and index the derived text. |
| [PaddlePaddle/PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR) | Multilingual OCR and document structure; 87,808 stars; Apache-2.0 | Benchmark for images and difficult scans if Apple Vision or OCRmyPDF is insufficient. |
| [opendatalab/MinerU](https://github.com/opendatalab/MinerU) | Complex PDF and Office documents to Markdown/JSON; 77,833 stars | Strong comparison for academic documents; resolve its multi-file license before reuse. |
| [Unstructured-IO/unstructured](https://github.com/Unstructured-IO/unstructured) | Broad document ETL, partitioning, enrichment, and chunking; 15,319 stars; Apache-2.0 | Mine its format coverage and partition model. It is heavier than this local utility needs initially. |
| [firecrawl/firecrawl](https://github.com/firecrawl/firecrawl) | Web search, crawl, and page extraction; 168,609 stars; AGPL-3.0 | Prefer its API or a separate service boundary if dynamic sites require it. |
| [unclecode/crawl4ai](https://github.com/unclecode/crawl4ai) | LLM-oriented web crawling and extraction; 78,507 stars; Apache-2.0 | Better licensing for a local worker; still much larger than a browser-history collector. |
| [simonw/strip-tags](https://github.com/simonw/strip-tags) | Small HTML-to-text CLI; 362 stars; Apache-2.0 | Good deterministic fallback for saved pages and email HTML. |
| [RhetTbull/textinator](https://github.com/RhetTbull/textinator) | macOS screenshot OCR; 203 stars; MIT | Useful Apple Vision reference if screenshot indexing becomes a source. |

## Retrieval engines to benchmark

| Repository | Strength | Threshold for adoption |
| --- | --- | --- |
| SQLite FTS5 | Already embedded, deterministic, low operational cost | Keep as the control until another engine materially improves real top-five results. |
| [quickwit-oss/tantivy](https://github.com/quickwit-oss/tantivy) | Native Rust lexical engine with richer indexing and query machinery; 15,720 stars; MIT | Adopt when corpus size, facets, or ranking control outgrow FTS5 without breaking the 50 ms warm p95 target. |
| [meilisearch/meilisearch](https://github.com/meilisearch/meilisearch) | Typo tolerance, filters, hybrid search, polished API; 58,994 stars | Benchmark if misspellings dominate failures. Its separate process and current multi-license terms must be acceptable. |
| [typesense/typesense](https://github.com/typesense/typesense) | Fast typo-tolerant and hybrid search; 26,442 stars; GPL-3.0 | Same test as Meilisearch; do not embed without resolving GPL implications. |
| [StarTrail-org/LEANN](https://github.com/StarTrail-org/LEANN) | Compact local vector retrieval | Test as a reranker over lexical candidates before making it the primary index. |
| [hanxiao/omni-macos](https://github.com/hanxiao/omni-macos) | On-device cross-modal retrieval and exact rescoring | Files-only benchmark for concepts, images, audio, and video. |
| [lancedb/lancedb](https://github.com/lancedb/lancedb) | Embedded multimodal vector storage; 11,170 stars; Apache-2.0 | Best general embedded-vector candidate if LEANN is too specialized. |
| [chroma-core/chroma](https://github.com/chroma-core/chroma) | Widely used local retrieval API; 29,077 stars; Apache-2.0 | Useful comparison; Python/service lifecycle is a disadvantage for the desktop app. |
| [qdrant/qdrant](https://github.com/qdrant/qdrant) | Mature vector service with filters and scale; 34,029 stars; Apache-2.0 | Server deployment is unjustified until one local index serves several clients or exceeds embedded limits. |

## Launcher and interaction references

| Repository | Relevance | Decision |
| --- | --- | --- |
| [ospfranco/sol](https://github.com/ospfranco/sol) | Native macOS launcher and command palette; 3,059 stars; MIT | Best code-level launcher reference for this Mac-first app. Compare shortcut, focus, actions, previews, and window lifecycle. |
| [Xoshbin/asyar](https://github.com/Xoshbin/asyar) | Cross-platform launcher, file index, clipboard, previews, extensions, and published latency benchmarks; GPL-3.0 | Best benchmark and feature reference; avoid copying code until license compatibility is explicit. |
| [raycast/extensions](https://github.com/raycast/extensions) | Thousands of command and result interaction examples; MIT | Mine result layouts and action conventions. Raycast itself is not the open-source base. |
| [cerebroapp/cerebro](https://github.com/cerebroapp/cerebro) | Extensible Electron launcher; 8,557 stars; MIT | Useful plugin-model reference; too heavy and generic as a replacement. |
| [albertlauncher/albert](https://github.com/albertlauncher/albert) | Fast extensible keyboard launcher; 7,975 stars | Linux-oriented and license metadata is unresolved; interaction reference only. |
| [Ulauncher/Ulauncher](https://github.com/Ulauncher/Ulauncher) | Mature Linux launcher; 4,502 stars | Plugin and ranking reference only. |

## Knowledge and memory systems to mine

These projects are not replacement launchers. Inspect their data models, deletion semantics, source status, chunking, and retrieval evaluation.

| Repository | Useful layer | Caveat |
| --- | --- | --- |
| [letta-ai/letta](https://github.com/letta-ai/letta) | Stateful memory objects and agent-visible memory operations | Agent-centric; persistent memory is not the same as faithful source search. |
| [mem0ai/mem0](https://github.com/mem0ai/mem0) | Cross-application memory extraction and retrieval | Summarized memories can lose provenance; never substitute them for source records. |
| [siyuan-note/siyuan](https://github.com/siyuan-note/siyuan) | Mature local knowledge graph and document workspace | Full application and AGPL-3.0, not a component library. |
| [blinkospace/blinko](https://github.com/blinkospace/blinko) | Self-hosted personal notes plus AI retrieval | GPL-3.0 and note-centric. |
| [matthiasn/lotti](https://github.com/matthiasn/lotti) | Encrypted personal logbook with local AI and cross-device sync | Good privacy and approval patterns; GPL-3.0 and a different product. |
| [reorproject/reor](https://github.com/reorproject/reor) | Local knowledge manager and semantic search | Archived since 2025; historical design reference only. |

## Watch, borrow narrowly, or reject

| Repository | Why not a current base |
| --- | --- |
| [screenpipe/screenpipe](https://github.com/screenpipe/screenpipe) | Valuable optional screen/audio source, but it loses native provenance and now uses a custom source-available commercial license. Integrate through an API, if at all. |
| [openrecall/openrecall](https://github.com/openrecall/openrecall) | Screenshot/OCR memory only; three contributors and no push since 2025-09 at the snapshot. |
| [samuelmeseret/memsearch](https://github.com/samuelmeseret/memsearch) | Concrete Raycast, PhotoKit, PDF, and image experiment, but one star, 15 commits, one contributor, and a required Gemini key. |
| [moyangzhan/mango-finder](https://github.com/moyangzhan/mango-finder) | Natural-language multi-machine file search, but no detected license and one primary contributor. |
| [spyglass-search/spyglass](https://github.com/spyglass-search/spyglass) | Direct personal-search precedent, but archived. Hister is the active comparison. |
| [hanxiao/searchbox](https://github.com/hanxiao/searchbox) | High-signal air-gapped closed-corpus QA experiment, but designed around an agent exploring a dataroom rather than interactive personal search. |
| [badlogic/doxie](https://github.com/badlogic/doxie) | Small, legible RAG retrieval experiment; useful to read, too small to depend on. |
| [Mathews-Tom/omnex](https://github.com/Mathews-Tom/omnex) | Interesting structure-aware retrieval claim with almost no independent use evidence. |
| [Mathews-Tom/searchat](https://github.com/Mathews-Tom/searchat) | Semantic search over agent conversations; narrow and very early. |
| [can1357/obsidian-agent](https://github.com/can1357/obsidian-agent) | The likely original lead, but it is a fork of Obsidian Copilot and searches inside Obsidian rather than across personal sources. |
| [can1357/smgrep](https://github.com/can1357/smgrep) | Useful local semantic code-search experiment; explicitly built for coding agents, outside the requested product boundary. |

## Recommended system boundary

```text
Google APIs / IMAP ───── mail collectors ─────────┐
WhatsApp / Signal ───── audited protocol workers ─┤
Notes / Photos / files ─ native Mac collectors ───┤
browser history ─────── Hister-style collector ───┼→ normalized records → local FTS index → Search Bar
documents and media ─── extraction workers ───────┘                         └→ optional semantic reranker
```

Collectors may fail independently. The local replica remains searchable; every result preserves its source identifier and native open target. Network services and embedding models stay out of the per-keystroke query path.

## Adoption gates

- A collector must demonstrate incremental sync, deletions, stable identifiers, resumable backfill, and visible error state on a disposable account before touching the main archive.
- Write-capable mail and messaging tools must be denied write credentials or wrapped in a process that exposes no write operation.
- Extractors must pass a fixture corpus before inclusion. Record supported formats, failed files, elapsed time, peak memory, output size, and whether the original can still be opened.
- Retrieval candidates must beat SQLite FTS5 on the same 100 real queries. Record top-1, top-5, warm/cold p50 and p95, index time, incremental delay, disk, and idle memory.
- Keep warm query p95 under 50 ms and launcher-to-first-frame p95 under 100 ms.
- Do not send private content to a remote embedding or OCR endpoint by default.
- Do not vendor code without a compatible license. Missing GitHub license detection is a reason to inspect, not permission to copy.
- A one-maintainer dependency needs a maintained fork or a replaceable subprocess boundary before it becomes critical infrastructure.
