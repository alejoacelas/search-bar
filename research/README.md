# Search landscape

No mature open-source project found in the first pass searches local files, email, WhatsApp, and Signal behind one polished launcher. Keep this app as the integration shell; borrow source adapters, retrieval engines, and interaction patterns selectively.

## Best next experiments

1. Compare [Google Workspace CLI](https://github.com/googleworkspace/cli) with [Himalaya](https://github.com/pimalaya/himalaya) for email collection.
2. Prototype Apple Notes, Photos, and Spotlight collectors from [Rhet Turnbull's macOS tools](https://github.com/RhetTbull).
3. Audit [OpenMessage](https://github.com/MaxGhenis/openmessage) and [imsg](https://github.com/openclaw/imsg) behind a read-only process boundary.
4. Add [MarkItDown](https://github.com/microsoft/markitdown), then test [Docling](https://github.com/docling-project/docling) and [OCRmyPDF](https://github.com/ocrmypdf/OCRmyPDF) only where fixtures show a gap.
5. Benchmark [Hister](https://github.com/asciimoo/hister) for browser history and saved pages.
6. Compare FTS5 with [Tantivy](https://github.com/quickwit-oss/tantivy), [LEANN](https://github.com/StarTrail-org/LEANN), and [Omni](https://github.com/hanxiao/omni-macos).
7. Use [Sol](https://github.com/ospfranco/sol) and [Asyar](https://github.com/Xoshbin/asyar) as launcher interaction references.

The existing application already has the right boundary: collectors normalize records, the Mac keeps a local replica, and keystrokes never wait on a network service. None of the candidates justifies moving or replacing the app before those experiments produce better measured results.

## Files

- [Repository survey](repositories.md) records fit, maturity, licensing, and what to reuse.
- [AI power users](ai-power-users.md) maps 207 people across eight networks; [the CSV](ai-power-users.csv) is the reusable list.
- [Construction record](../reproduce/search-landscape.md) preserves the seed repositories, method, queries, and checks.
