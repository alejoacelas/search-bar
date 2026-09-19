# REPLICATE

## Finish the workspace migration

The human wanted the remaining folder reorganization finished and authorized interrupting conflicting sessions.

- Preserved this repository and its instruction wording, declared the tools group, and made CLAUDE.md import AGENTS.md. Its current path is /Users/alejo/best/tools/active/search-bar.

Agent session 01a072fe-84d6-73f3-b37e-3bb912088c38 · Commits search-bar: 6dd8963

## Agent instructions cleanup — 2026-09-19

Alejo asked to refresh project instructions and remove redundant Claude instruction files where native AGENTS.md loading is available.

- Updated the applicable instructions and removed redundant local Claude copies; distinct content and preserved snapshots remain.
- Checked instruction references and shared-context freshness; native Claude loading requires 2.1.277+ with the built-in feature enabled.

Agent session 01a0b915-3eb2-78b2-9add-6ba48ad9a3b1 · Commits 138ae9366a05528b96a101841629627e4fe8501d

## Explicit startup instructions

Alejo wanted shared instructions selected deliberately at startup, without copied text or automatic parent inheritance.

- Removed agent-context YAML and generated shared text; retained project-specific instructions locally.
- Shared groups: tools. Selection now lives in the machine's context registry; startup does not rewrite this file.

Agent session 01a0b915-3eb2-78b2-9add-6ba48ad9a3b1 · Commits 0c4dd7d

<a id="construction-records"></a>

## Construction records

Preserved records from the former construction-notes folder.

<a id="record-search-landscape"></a>

### Reproduce the search landscape

This record supports the repository survey dated 2026-08-17. It preserves a short path to refresh the work rather than a transcript of the session.

#### Request

Find current open-source work that could inform one personal search bar across local files, email, WhatsApp, Signal, and similar sources. Save a broad directory of useful people rather than anchoring it on one contributor network.

#### Inputs

The repository survey started with these active public coding-agent repositories:

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

GitHub redirects supplied the current owner names for OpenCode, Goose, and OpenHands. The first pass did not resolve Orca; the expanded search later found [stablyai/orca](https://github.com/stablyai/orca) and added its core contributors.

The account directory then expanded through independent project neighborhoods:

- personal search and memory: OpenMessage, Hister, LEANN, Khoj, Omni, Screenpipe, OpenHuman, Surf, SiYuan, Blinko, Lotti, Reor, Letta, and Mem0;
- agent runtimes: OpenClaw, Hermes Agent, Pi, OpenCode, Orca, Cline, OpenHands, Continue, Aider, Crush, Goose, Browser Use, and Vibe Kanban;
- workflow design: Superpowers, agent-skills, 12-factor-agents, Compound Engineering, CrewAI, Agno, Pydantic AI, SWE-agent, and SWE-bench;
- MCP and ingestion: FastMCP, the official MCP servers, mcp-use, mcp-agent, XcodeBuildMCP, Firecrawl, Crawl4AI, agent-browser, and MarkItDown;
- local inference: Ollama, llama.cpp, vLLM, SGLang, LiteLLM, Open WebUI, AnythingLLM, MLX-VLM, ExLlama, and MLX;
- retrieval: Chroma, LanceDB, LightRAG, Qdrant, Weaviate, LangChain, and LlamaIndex;
- evaluation: Promptfoo, Langfuse, DeepEval, Phoenix, Garak, Giskard, Laminar, and independent evaluation practitioners;
- independent portfolios: small public implementations, courses, model experiments, search tools, and AI engineering notes.

#### Method

1. Fetch repository stars, forks, dates, license, description, and default branch through `gh api repos/OWNER/REPO`.
2. Fetch the top 100 contributor records through `gh api repos/OWNER/REPO/contributors?per_page=100`.
3. Search current GitHub repositories by `ai-agent`, `llm`, `local-ai`, `model-context-protocol`, `rag`, `llm-evaluation`, `coding agent`, and `personal AI`, with activity and star thresholds used only to find neighborhoods.
4. Inspect public profiles and up to 100 recently updated repositories for adjacent personal tools. This portfolio transfer surfaced mail clients, message gateways, local search, agent memory, and document utilities that contributor rankings missed.
5. Read candidate READMEs, license metadata, and repository roots. Use default-branch history, releases, and human contributor counts as rough activity signals.
6. Add creators and several sustained human contributors from each independent neighborhood. Exclude bots, organizations, generated accounts, and company-only handles.
7. Add independent builders when a broad portfolio of inspectable experiments supplied evidence that one flagship repository could not.
8. Verify every profile and anchor repository through the GitHub API, then check that the Markdown and CSV contain the same unique people.

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

GitHub repository queries used for the account expansion included:

- `topic:ai-agent stars:>2000 pushed:>2026-01-01`
- `topic:local-ai stars:>1000 pushed:>2026-01-01`
- `topic:model-context-protocol stars:>500 pushed:>2026-01-01`
- `topic:rag stars:>2000 pushed:>2026-01-01`
- `topic:llm-evaluation stars:>500 pushed:>2026-01-01`
- `"coding agent" in:name,description stars:>1000 pushed:>2026-01-01`
- `"personal AI" in:name,description stars:>500 pushed:>2026-01-01`

#### Selection rules

A repository stayed in the main survey when it implemented at least one hard product layer: source ingestion, persistent personal indexing, mixed-format retrieval, or an always-available launcher. Generic agent memory and code-search projects were recorded only when they came directly from the Oh My Pi lead.

An account stayed in the reusable list when public work showed project ownership, sustained core contribution, or an unusually concrete portfolio of AI experiments. Commit counts, stars, followers, bios, and employers were not sufficient by themselves. Each person was assigned one primary category even when their work crossed several.

#### Checks

- Every linked repository resolved on GitHub on 2026-08-17.
- Counts in the candidate table were re-fetched after redirects were resolved.
- Fork status was checked before interpreting commit or contributor history.
- GitHub license metadata was checked against README license statements where relevant. Screenpipe's custom commercial license and Mango Finder's missing detected license are called out explicitly.
- The local project's existing docs and implementation boundary were reviewed before recommending replacement or relocation.
- Every GitHub repository linked from the research documents resolved after the expanded source, extraction, retrieval, and launcher scan.
- All 207 account profiles and their anchor repositories resolved through the GitHub API.
- The CSV has 207 unique case-insensitive logins across eight categories; no bots or organizations remain.
- The Markdown contains the same 207 unique profile links as the CSV.

Refresh the counts and activity dates before making a dependency decision; they are snapshots, not live badges.
