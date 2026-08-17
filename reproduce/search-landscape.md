# Reproduce the search landscape

This record supports the repository survey dated 2026-08-17. It preserves a short path to refresh the work rather than a transcript of the session.

## Request

Find current open-source work that could inform one personal search bar across local files, email, WhatsApp, Signal, and similar sources. Start from substantial contributors to current AI and agent software, inspect their public repositories, document close candidates, and save the useful accounts for later research.

## Inputs

The contributor seed used these active public repositories:

- `can1357/oh-my-pi`
- `openai/codex`
- `anomalyco/opencode`
- `aaif-goose/goose`
- `cline/cline`
- `OpenHands/OpenHands`
- `continuedev/continue`
- `Aider-AI/aider`
- `charmbracelet/crush`
- `openinterpreter/openinterpreter`

GitHub redirects supplied the current owner names for OpenCode, Goose, and OpenHands. Orca was not used as a seed because no unambiguous public repository for the product was found.

## Method

1. Fetch repository stars, forks, dates, license, description, and default branch through `gh api repos/OWNER/REPO`.
2. Fetch the top 100 contributor records through `gh api repos/OWNER/REPO/contributors?per_page=100`.
3. Inspect public profiles and up to 100 recently updated repositories for names, descriptions, and topics matching `search`, `obsidian`, `launcher`, `memory`, `knowledge`, `mail`, `whatsapp`, `signal`, `messages`, `notes`, `index`, `recall`, or `local-first`.
4. Follow direct searches for open-source unified personal search, desktop search, local recall, and macOS semantic search.
5. Read candidate READMEs, license metadata, and repository roots. Use GraphQL default-branch history, release, and REST contributor counts as rough maturity signals.
6. Separate complete products from reusable collectors, retrieval engines, and launcher references. Discount inherited history on forks.

Representative commands:

```sh
gh api repos/can1357/oh-my-pi/contributors?per_page=100 \
  --jq '.[] | select(.type == "User") | [.login,.contributions] | @tsv'

gh api repos/asciimoo/hister \
  --jq '[.full_name,.stargazers_count,.created_at,.pushed_at,.license.spdx_id] | @tsv'

gh api repos/asciimoo/hister/readme \
  -H 'Accept: application/vnd.github.raw+json'
```

Web discovery queries included:

- `open source personal search engine local files email messages unified search GitHub macOS`
- `GitHub open source Rewind alternative local search all computer data`
- `GitHub local first personal knowledge search email files Obsidian`
- `GitHub open source desktop launcher unified search local files email macOS`

## Selection rules

A repository stayed in the main survey when it implemented at least one hard product layer: source ingestion, persistent personal indexing, mixed-format retrieval, or an always-available launcher. Generic agent memory and code-search projects were recorded only when they came directly from the Oh My Pi lead.

An account stayed in the reusable list when it met a contribution threshold or combined sustained seed contributions with directly relevant independent work. Automation-like and company-only accounts were omitted.

## Checks

- Every linked repository resolved on GitHub on 2026-08-17.
- Counts in the candidate table were re-fetched after redirects were resolved.
- Fork status was checked before interpreting commit or contributor history.
- GitHub license metadata was checked against README license statements where relevant. Screenpipe's custom commercial license and Mango Finder's missing detected license are called out explicitly.
- The local project's existing docs and implementation boundary were reviewed before recommending replacement or relocation.

Refresh the counts and activity dates before making a dependency decision; they are snapshots, not live badges.
