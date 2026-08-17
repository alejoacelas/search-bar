# Search landscape

No mature open-source project found in the first pass searches local files, email, WhatsApp, and Signal behind one polished launcher. Keep this app as the integration shell; borrow source adapters, retrieval engines, and interaction patterns selectively.

## Best next experiments

1. Audit [OpenMessage](https://github.com/MaxGhenis/openmessage) as a read-only WhatsApp and Signal collector. Run it against disposable linked-device sessions before replacing the current WhatsApp bridge.
2. Benchmark [Hister](https://github.com/asciimoo/hister) beside the current FTS5 index for browser history, saved web pages, and richer lexical queries.
3. Test [LEANN](https://github.com/StarTrail-org/LEANN) as an optional semantic reranker and source of Apple Mail, iMessage, Slack, and browser-history import patterns.
4. Compare [Omni](https://github.com/hanxiao/omni-macos) against the current file search on remembered-concept, PDF, image, and audio queries.
5. Use [Asyar](https://github.com/Xoshbin/asyar) as the launcher and extension-system reference. Do not copy GPL code until this project's license is chosen.

The existing application already has the right boundary: collectors normalize records, the Mac keeps a local replica, and keystrokes never wait on a network service. None of the candidates justifies moving or replacing the app before those experiments produce better measured results.

## Files

- [Repository survey](repositories.md) records fit, maturity, licensing, and what to reuse.
- [AI power users](ai-power-users.md) maps 203 people across eight networks; [the CSV](ai-power-users.csv) is the reusable list.
- [Construction record](../reproduce/search-landscape.md) preserves the seed repositories, method, queries, and checks.
