<!--ai-->
# Universal search

## Decision

Keep one index on this Mac. Feed it with source-specific connectors and search only local data on each keystroke. Do not put a server in the query path.

This gives the common path no network dependency, keeps private correspondence off another machine, and lets results open in installed apps. Add a server later only to collect data from devices this Mac cannot reach or to search from multiple devices; sync normalized documents from it into the same local index.

## Product contract

- ⌘Space shows or hides the bar and focuses an empty query.
- Typing produces useful local results within 50 ms at p95. Showing the already-running window takes under 100 ms at p95.
- Each result has a source, title, people, time, excerpt, and one stable source identifier.
- Return opens the result in its native app or browser. ⌘C copies its useful content or URL. ⌘Return reveals the available actions.
- Results remain searchable offline. Each source shows its last successful sync and any error.
- Search never waits for a connector. Connectors update the index in the background.

Measure window latency from shortcut event to first rendered frame and query latency from input event to painted results. Keep a rolling p50/p95 in a local diagnostics page. The targets should change only after measuring real use.

## Shape of the system

```text
local files ─┐
Git repos ───┤
Gmail ───────┤  connectors → normalized documents → SQLite FTS index → Search Bar
WhatsApp ────┤                         ↑                    │
Signal ──────┘                    sync checkpoints          └→ open/copy actions
```

Use a shared document schema instead of giving the UI a different result type for every source:

```text
id              source + immutable source identifier
source          file | github | gmail | whatsapp | signal
kind            file | commit | email | message | thread | attachment
title           filename, subject, or conversation name
body            extracted searchable text
people          normalized sender and recipients
occurred_at     source timestamp
open_target     file path, HTTPS URL, or app deep link
copy_text       useful default clipboard value
metadata_json   source-specific fields
updated_at      connector observation time
```

Store source records in ordinary SQLite tables and maintain an FTS5 external-content table over the searchable fields. The current index deletes every file on refresh; replace that with upserts and tombstones before adding a second source. Track one cursor per connector so routine sync cost is proportional to changes, not total history.

## Sources

### Local files and Git repositories

Index local checkouts directly. Git already supplies the durable sync protocol, and a checkout preserves native paths, ignores, history, branches, and offline access. For repositories that are not on this Mac, clone selected repositories as bare or partial clones into an app-managed directory, then index the default branch. Do not fetch every blob through the GitHub API: the recursive tree endpoint can truncate above 100,000 entries or 7 MB, while Git fetch is built for repository transfer.

Add PDF, Word, and image extraction after incremental file watching works. Metadata-only results can appear immediately; extraction can fill bodies later.

Open a working-tree file in its default app. Open a remote-only file or commit at its GitHub URL. Copy the path for local files and the permalink for remote files.

### Gmail

Use OAuth with the read-only Gmail scope. Perform one paginated import, then use the Gmail history cursor for incremental additions, changes, and deletions. Store message headers, decoded text/plain bodies, a conservative HTML-to-text fallback, attachment names, and thread IDs. Fetch attachment contents only when their formats become supported.

Open with a Gmail thread URL in the browser. Copy the message excerpt by default; expose copy-link and copy-sender as secondary actions.

Gmail is the first non-file connector because it has a supported API for messages, threads, labels, and mailbox history.

### WhatsApp

Do not build the core product on the WhatsApp Business API: it is for messages handled by a business account, not an API for a person's existing consumer history. Start with explicit chat exports imported as snapshots. A connector can watch an import folder and replace each conversation snapshot when a newer export appears.

Opening an exact historical message may not be possible through a supported deep link. Open the conversation in WhatsApp when a stable target exists; otherwise copy the matched text and display the conversation and timestamp needed to find it.

Treat automated extraction from WhatsApp Desktop's private storage as an optional experiment. It can break after any app update and should never be the only copy of indexed provenance.

### Signal

Signal exposes no supported API for searching a personal message archive. Its Desktop data is local, but its private schema and encryption are not an integration contract. The supported history-transfer flow is for moving history into a linked Desktop installation, not exporting it for another app.

Defer Signal until files, GitHub, and Gmail prove the unified result model. Then test a local, read-only Desktop adapter behind a feature flag. It must detect schema/version changes, stop with a visible error, never modify Signal data, and rebuild from scratch. If that proves unreliable, keep Signal as a federated action that opens Signal's own search rather than pretending its history is indexed.

## Ranking

Start with lexical retrieval. Rank by:

1. exact title or filename match;
2. BM25 across title, people, and body;
3. a bounded recency boost;
4. a bounded boost from how often and how recently this result was opened.

Mix sources in one list, but cap one source at roughly half the first ten results so a long email thread cannot hide an exact filename. Add source filters such as `mail:`, `from:`, and `in:` after observing ambiguous real queries. Add embeddings only if a query log shows that remembered concepts, rather than remembered words, are a material failure mode. Embeddings should rerank a lexical candidate set, not replace it.

## Server boundary

A server becomes useful when at least one of these is true:

- another always-on device has data unavailable on this Mac;
- more than one client must search the same corpus;
- background ingestion must continue while this Mac is off.

If that threshold is crossed, let the server ingest and encrypt normalized source records, then sync changes to a local SQLite replica. Queries still hit the replica. Use end-to-end encryption if the server stores correspondence; otherwise it becomes a new plaintext archive of email and messages. GitHub repositories should continue to move through Git rather than through this record-sync service.

## Build order

1. Launcher: ⌘Space toggle, launch at login, hide on Escape and focus loss, stable signing, and latency instrumentation.
2. Index core: normalized schema, incremental upserts/deletes, filesystem watcher, multiple roots, source status, and open/copy action registry.
3. Retrieval: usage history, recency, cross-source ranking tests, and a small fixture corpus with expected top results.
4. Git: discover existing checkouts, optionally clone selected missing repositories, fetch in the background, and open local paths or GitHub permalinks.
5. Gmail: OAuth, initial import, history-based incremental sync, thread links, and explicit revocation/deletion behavior.
6. WhatsApp export importer.
7. Signal feasibility spike with a stop condition: abandon direct indexing if two ordinary Signal Desktop upgrades require adapter repairs.
8. Reconsider a sync server only against the threshold above.

The first vertical slice after the launcher should search local files plus one Gmail account through the same schema and expose open and copy actions. That tests the hard boundary—heterogeneous data with native actions—without first taking on unsupported messaging stores.

## Open decisions to settle from use

- Whether a query should search message-level results or one aggregated conversation/thread result. Start with messages for email and messages, then group adjacent hits in the UI if repetition is noisy.
- Whether deleted source material should disappear immediately or remain in a local archive. Default to mirroring deletion; an archive is a separate product and privacy decision.
- Whether remote Git branches matter. Start with each repository's default branch plus the current branch of local working trees.
- Which copy action deserves ⌘C for each kind. Record secondary-action use before changing the defaults.

## Primary references

- [Apple: change or disable a conflicting keyboard shortcut](https://support.apple.com/guide/mac-help/mchlp2262/mac)
- [Tauri global shortcut plugin](https://v2.tauri.app/plugin/global-shortcut/)
- [Gmail API](https://developers.google.com/workspace/gmail/api/reference/rest)
- [Gmail message listing and search](https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.messages/list)
- [GitHub Git trees API and limits](https://docs.github.com/en/rest/git/trees)
- [Signal backups and device transfers](https://support.signal.org/hc/en-us/articles/10074659364122-Backups-and-Device-Transfers-on-Signal)
<!--/ai-->
