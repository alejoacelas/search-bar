# Search Bar decisions

## Core decisions

### Local search

- [Use embedded FTS5 instead of rescanning on each keystroke](#decision-1).
- [Keep indexing limits and failures visible](#decision-2).

### Connected sources

- [Keep source ingestion behind a stable result and change-feed boundary](#decision-3).

### Research reuse

- [Select dependencies by implemented product layers](#decision-4).
- [Keep the people directory broad and evidence-backed](#decision-5).

## Details

<a id="decision-1"></a>

### Use embedded FTS5 instead of rescanning on each keystroke

The initial index-free search was too slow over a home directory. Background indexing with ignore-aware traversal keeps queries local and avoids supervising a separate search service; reconsider Typesense only when its ranking or typo tolerance justifies that operational cost. See [docs/search-engine-candidates.md](docs/search-engine-candidates.md).

<a id="decision-2"></a>

### Keep indexing limits and failures visible

The native layer skips unreadable, binary and oversized files and limits returned results. Do not present the index as every file’s complete contents; preserve the documented refresh and extraction limits when changing ranking or coverage. See [docs/how-search-works.md](docs/how-search-works.md).

<a id="decision-3"></a>

### Keep source ingestion behind a stable result and change-feed boundary

Local files and collector records share typed search results. The collector can move to an always-on Mac without coupling application actions to ingestion; distinguish the implemented WhatsApp slice from planned email and other sources. See [docs/connector-coverage.md](docs/connector-coverage.md).

<a id="decision-4"></a>

### Select dependencies by implemented product layers

The survey includes ingestion, persistent personal indexing, mixed-format retrieval or a launcher. Stars and activity help discovery but do not prove suitability; refresh licenses, redirects and maintenance evidence before adopting a dependency. See [research/repositories.md](research/repositories.md).

<a id="decision-5"></a>

### Keep the people directory broad and evidence-backed

Use ownership, sustained contributions or inspectable portfolios across independent project neighborhoods. Do not substitute employer names, follower counts or one contributor network; keep case-insensitive unique people consistent between Markdown and CSV. See [research/ai-power-users.md](research/ai-power-users.md). History inspected: [e09e957](https://github.com/alejoacelas/search-bar/commit/e09e957).
