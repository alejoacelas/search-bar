# AI power users

This is a directory of people whose public work is useful for discovering how serious AI users build, test, and integrate tools. It is not a ranking and does not claim that everyone here uses AI in the same way.

The first version was too dependent on Oh My Pi contributors. This version follows eight independent networks: personal search and memory, agent runtimes, agent workflow design, MCP and connectors, local inference, retrieval infrastructure, evaluation, and independent experimentation.

[Download the CSV](ai-power-users.csv) for filtering and reuse. Snapshot: 2026-08-17.

## Start here

These accounts have the highest overlap with this project or connect several of the networks below:

| Account | Why start here |
| --- | --- |
| [can1357](https://github.com/can1357) | Builds Oh My Pi, semantic search, Notion sync, and a local-first Obsidian fork. |
| [badlogic](https://github.com/badlogic) | Created Pi, maintains its current agent toolkit, and builds small personal utilities such as `gmcli`. |
| [steipete](https://github.com/steipete) | Builds OpenClaw and a large portfolio of practical AI utilities for macOS and the command line. |
| [simonw](https://github.com/simonw) | Builds the `llm` CLI and hundreds of inspectable experiments around models, tools, structured data, and retrieval. |
| [mitsuhiko](https://github.com/mitsuhiko) | Works on the Pi agent toolkit and repeatedly turns new AI interaction ideas into small public tools. |
| [asciimoo](https://github.com/asciimoo) | Created Searx and now builds Hister, a private index of web history and local files. |
| [hanxiao](https://github.com/hanxiao) | Builds Omni's on-device multimodal search and has a long search-infrastructure history. |
| [MaxGhenis](https://github.com/MaxGhenis) | Builds OpenMessage, the closest public WhatsApp and Signal ingestion candidate found. |
| [yichuan-w](https://github.com/yichuan-w) | Leads LEANN's compact local retrieval work across files, mail, messages, and browsing history. |
| [debanjum](https://github.com/debanjum) | Leads Khoj, one of the longest-running open personal-AI and document-search systems. |
| [thedotmack](https://github.com/thedotmack) | Builds persistent memory across several coding-agent products. |
| [jlowin](https://github.com/jlowin) | Created FastMCP and is a useful bridge into the MCP implementation ecosystem. |
| [MagMueller](https://github.com/MagMueller) | Leads Browser Use, a major implementation of browser access for agents. |
| [nickscamara](https://github.com/nickscamara) | Leads Firecrawl, a high-activity web ingestion and search project. |
| [ggerganov](https://github.com/ggerganov) | Created `llama.cpp`; central to practical local-model inference. |
| [karpathy](https://github.com/karpathy) | Publishes minimal, legible implementations that expose how modern language models and agents work. |
| [hamelsmu](https://github.com/hamelsmu) | Publishes unusually concrete work on AI evaluation, data, and production practice. |
| [shreyashankar](https://github.com/shreyashankar) | Builds and researches reliable evaluation and data systems for AI applications. |
| [typpo](https://github.com/typpo) | Leads Promptfoo's practical testing and red-teaming workflow. |
| [jxnl](https://github.com/jxnl) | Builds Instructor and publishes applied patterns for structured outputs, retrieval, and evaluation. |

## Personal search, memory, and local-first AI

These people are closest to the search-bar problem: indexing personal data, retrieving it, or building an end-user system around it.

| Account | Public evidence |
| --- | --- |
| [can1357](https://github.com/can1357) | Creator of [Oh My Pi](https://github.com/can1357/oh-my-pi), [smgrep](https://github.com/can1357/smgrep), [notionfs](https://github.com/can1357/notionfs), and a local-first [Obsidian Copilot fork](https://github.com/can1357/obsidian-agent). |
| [badlogic](https://github.com/badlogic) | Creator of the original Pi agent and [gmcli](https://github.com/badlogic/gmcli), a focused Gmail command-line client. |
| [Mathews-Tom](https://github.com/Mathews-Tom) | Prototypes [archex](https://github.com/Mathews-Tom/archex), [searchat](https://github.com/Mathews-Tom/searchat), and [VaultMind](https://github.com/Mathews-Tom/VaultMind). Most are idea sources rather than validated dependencies. |
| [metaphorics](https://github.com/metaphorics) | Builds memory and search experiments around Pi, Morph, Tantivy, and code graphs. |
| [oldschoola](https://github.com/oldschoola) | Builds [omp-chrollo](https://github.com/oldschoola/omp-chrollo) and other retrieval or memory experiments around Pi. |
| [wolfiesch](https://github.com/wolfiesch) | Builds [OMP episodic memory](https://github.com/wolfiesch/omp-episodic-memory) and an [iMessage gateway](https://github.com/wolfiesch/wolfies-imessage-gateway). |
| [kitlangton](https://github.com/kitlangton) | Builds [mail-control](https://github.com/kitlangton/mail-control), a multi-account Gmail and iCloud mail CLI. |
| [pashpashpash](https://github.com/pashpashpash) | Former Cline AI lead and creator of [vault-ai](https://github.com/pashpashpash/vault-ai), an early personal knowledge retrieval app. |
| [jlongster](https://github.com/jlongster) | Built [absurd-sql](https://github.com/jlongster/absurd-sql) and [rocket-bar](https://github.com/jlongster/rocket-bar), an older live deep-search experiment. |
| [MaxGhenis](https://github.com/MaxGhenis) | Leads [OpenMessage](https://github.com/MaxGhenis/openmessage), a local Google Messages, WhatsApp, and Signal inbox. |
| [RhetTbull](https://github.com/RhetTbull) | Builds [osxphotos](https://github.com/RhetTbull/osxphotos), [macnotesapp](https://github.com/RhetTbull/macnotesapp), Apple Notes parsing, Spotlight metadata tools, and macOS OCR utilities. |
| [asciimoo](https://github.com/asciimoo) | Created Searx and leads [Hister](https://github.com/asciimoo/hister), a private index of visited pages and local files. |
| [hanxiao](https://github.com/hanxiao) | Leads [Omni](https://github.com/hanxiao/omni-macos), native multimodal file search on Apple silicon. |
| [yichuan-w](https://github.com/yichuan-w) | Top contributor to [LEANN](https://github.com/StarTrail-org/LEANN), covering compact retrieval over files, mail, chat, and browser history. |
| [andylizf](https://github.com/andylizf) | Core LEANN contributor working on the same compact personal-retrieval stack. |
| [debanjum](https://github.com/debanjum) | Creator and lead maintainer of [Khoj](https://github.com/khoj-ai/khoj). |
| [sabaimran](https://github.com/sabaimran) | Core Khoj contributor across its personal-AI application and integrations. |
| [louis030195](https://github.com/louis030195) | Creator and lead contributor to [Screenpipe](https://github.com/screenpipe/screenpipe). |
| [senamakel](https://github.com/senamakel) | Lead contributor to [OpenHuman](https://github.com/tinyhumansai/openhuman), a local-first personal memory and agent system. |
| [aavshr](https://github.com/aavshr) | Leads [Deta Surf](https://github.com/deta/surf), local notebooks over files and webpages. |
| [88250](https://github.com/88250) | Co-leads [SiYuan](https://github.com/siyuan-note/siyuan), a privacy-first knowledge workspace. |
| [Vanessa219](https://github.com/Vanessa219) | Co-leads SiYuan's long-running local knowledge application. |
| [blinko-space](https://github.com/blinko-space) | Leads [Blinko](https://github.com/blinkospace/blinko), a self-hosted AI note tool. |
| [lcomplete](https://github.com/lcomplete) | Leads [Huntly](https://github.com/lcomplete/huntly), a personal information hub. |
| [matthiasn](https://github.com/matthiasn) | Leads [Lotti](https://github.com/matthiasn/lotti), an encrypted personal logbook with local AI assistants. |
| [samlhuillier](https://github.com/samlhuillier) | Created [Reor](https://github.com/reorproject/reor), a private local AI knowledge manager. |
| [carenthomas](https://github.com/carenthomas) | Leads [Letta](https://github.com/letta-ai/letta), stateful agents with persistent memory. |
| [sarahwooders](https://github.com/sarahwooders) | Core Letta contributor and memory-system builder. |
| [thedotmack](https://github.com/thedotmack) | Creator of [claude-mem](https://github.com/thedotmack/claude-mem), persistent context shared across several agent runtimes. |
| [deshraj](https://github.com/deshraj) | Core contributor to [Mem0](https://github.com/mem0ai/mem0), a reusable agent-memory layer. |
| [timothycarambat](https://github.com/timothycarambat) | Creator and lead maintainer of [AnythingLLM](https://github.com/Mintplex-Labs/anything-llm), a local-first end-user AI workspace. |

## Agent runtimes and coding harnesses

These accounts expose what heavy users need from tool execution, context management, orchestration, and interaction loops.

| Account | Public evidence |
| --- | --- |
| [steipete](https://github.com/steipete) | Creator and lead contributor to [OpenClaw](https://github.com/openclaw/openclaw). |
| [vincentkoc](https://github.com/vincentkoc) | Major OpenClaw contributor. |
| [teknium1](https://github.com/teknium1) | Creator and lead contributor to [Hermes Agent](https://github.com/NousResearch/hermes-agent). |
| [OutThisLife](https://github.com/OutThisLife) | Core Hermes Agent contributor. |
| [kshitijk4poor](https://github.com/kshitijk4poor) | Core Hermes Agent contributor. |
| [mitsuhiko](https://github.com/mitsuhiko) | Core contributor to the current [Pi toolkit](https://github.com/earendil-works/pi) and prolific independent toolmaker. |
| [thdxr](https://github.com/thdxr) | Creator and lead contributor to [OpenCode](https://github.com/anomalyco/opencode). |
| [adamdotdevin](https://github.com/adamdotdevin) | Core OpenCode contributor. |
| [rekram1-node](https://github.com/rekram1-node) | Core OpenCode contributor. |
| [nwparker](https://github.com/nwparker) | Lead contributor to [Orca](https://github.com/stablyai/orca), an environment for parallel agent fleets. |
| [AmethystLiang](https://github.com/AmethystLiang) | Core Orca contributor. |
| [brennanb2025](https://github.com/brennanb2025) | Core Orca contributor. |
| [saoudrizwan](https://github.com/saoudrizwan) | Creator and lead contributor to [Cline](https://github.com/cline/cline). |
| [abeatrix](https://github.com/abeatrix) | Core Cline contributor. |
| [celestial-vault](https://github.com/celestial-vault) | Core Cline contributor. |
| [tofarr](https://github.com/tofarr) | Core [OpenHands](https://github.com/OpenHands/OpenHands) contributor. |
| [hieptl](https://github.com/hieptl) | Core OpenHands contributor. |
| [xingyaoww](https://github.com/xingyaoww) | OpenHands co-founder and core contributor. |
| [rbren](https://github.com/rbren) | OpenHands co-founder and core contributor. |
| [neubig](https://github.com/neubig) | OpenHands co-founder, maintainer, and language-model researcher. |
| [sestinj](https://github.com/sestinj) | Creator and lead contributor to [Continue](https://github.com/continuedev/continue). |
| [RomneyDa](https://github.com/RomneyDa) | Core Continue contributor. |
| [Patrick-Erichsen](https://github.com/Patrick-Erichsen) | Core Continue contributor. |
| [paul-gauthier](https://github.com/paul-gauthier) | Creator and lead contributor to [Aider](https://github.com/Aider-AI/aider). |
| [kujtimiihoxha](https://github.com/kujtimiihoxha) | Lead contributor to [Crush](https://github.com/charmbracelet/crush). |
| [andreynering](https://github.com/andreynering) | Core Crush contributor. |
| [meowgorithm](https://github.com/meowgorithm) | Core Crush contributor and Charm founder. |
| [aymanbagabas](https://github.com/aymanbagabas) | Core Crush and Charm contributor. |
| [DOsinga](https://github.com/DOsinga) | Core [Goose](https://github.com/aaif-goose/goose) contributor with many independent experiments. |
| [alexhancock](https://github.com/alexhancock) | Core Goose contributor. |
| [zanesq](https://github.com/zanesq) | Core Goose contributor. |
| [blackgirlbytes](https://github.com/blackgirlbytes) | Goose contributor with a large public portfolio of AI application experiments. |
| [MagMueller](https://github.com/MagMueller) | Creator and lead contributor to [Browser Use](https://github.com/browser-use/browser-use). |
| [pirate](https://github.com/pirate) | Core Browser Use contributor and long-running archiving toolmaker. |
| [mertunsall](https://github.com/mertunsall) | Core Browser Use contributor. |
| [sauravpanda](https://github.com/sauravpanda) | Core Browser Use contributor. |
| [stunningpixels](https://github.com/stunningpixels) | Lead contributor to [Vibe Kanban](https://github.com/BloopAI/vibe-kanban), a multi-agent work surface. |
| [code-yeongyu](https://github.com/code-yeongyu) | Creator and lead contributor to [Oh My OpenAgent](https://github.com/code-yeongyu/oh-my-openagent). |
| [Bill-Billion](https://github.com/Bill-Billion) | Lead contributor to [learn-claude-code](https://github.com/shareAI-lab/learn-claude-code), a minimal agent-harness implementation. |
| [Gui-Yue](https://github.com/Gui-Yue) | Core learn-claude-code contributor. |
| [affaan-m](https://github.com/affaan-m) | Creator of [ECC](https://github.com/affaan-m/ECC), an agent-harness optimization system. |

## Agent workflow, skills, and application design

These people work one layer above the runtime: development methods, reusable skills, typed interfaces, and user-facing agent products.

| Account | Public evidence |
| --- | --- |
| [obra](https://github.com/obra) | Creator of [Superpowers](https://github.com/obra/superpowers), an agentic skills framework and development method. |
| [addyosmani](https://github.com/addyosmani) | Creator of [agent-skills](https://github.com/addyosmani/agent-skills) and many public AI engineering guides. |
| [dexhorthy](https://github.com/dexhorthy) | Creator of [12-factor-agents](https://github.com/humanlayer/12-factor-agents), production principles for LLM software. |
| [tmchow](https://github.com/tmchow) | Lead contributor to Every's [compound-engineering plugin](https://github.com/everyinc/compound-engineering-plugin). |
| [kieranklaassen](https://github.com/kieranklaassen) | Core contributor to the compound-engineering workflow. |
| [joaomdmoura](https://github.com/joaomdmoura) | Creator and lead contributor to [CrewAI](https://github.com/crewAIInc/crewAI). |
| [ashpreetbedi](https://github.com/ashpreetbedi) | Creator and lead contributor to [Agno](https://github.com/agno-agi/agno). |
| [DouweM](https://github.com/DouweM) | Lead contributor to [Pydantic AI](https://github.com/pydantic/pydantic-ai). |
| [samuelcolvin](https://github.com/samuelcolvin) | Pydantic creator and core Pydantic AI contributor. |
| [klieret](https://github.com/klieret) | Lead contributor to [SWE-agent](https://github.com/SWE-agent/SWE-agent). |
| [john-b-yang](https://github.com/john-b-yang) | Lead contributor to [SWE-bench](https://github.com/swe-bench/SWE-bench) and contributor to SWE-agent. |
| [closji](https://github.com/closji) | Core contributor across SWE-agent and SWE-bench. |
| [lgrammel](https://github.com/lgrammel) | Lead contributor to Vercel's [AI SDK](https://github.com/vercel/ai). |
| [ospfranco](https://github.com/ospfranco) | Creator and lead contributor to [Sol](https://github.com/ospfranco/sol), a native macOS launcher and command palette. |

## MCP, connectors, browser access, and ingestion

This network matters because unified personal search succeeds or fails on source access and stable interfaces.

| Account | Public evidence |
| --- | --- |
| [jlowin](https://github.com/jlowin) | Creator and lead contributor to [FastMCP](https://github.com/PrefectHQ/fastmcp). |
| [strawgate](https://github.com/strawgate) | Core FastMCP contributor. |
| [zzstoatzz](https://github.com/zzstoatzz) | Core FastMCP contributor. |
| [olaservo](https://github.com/olaservo) | Lead contributor to the official [MCP servers](https://github.com/modelcontextprotocol/servers) repository. |
| [tadasant](https://github.com/tadasant) | Core contributor to the official MCP servers. |
| [cliffhall](https://github.com/cliffhall) | Core contributor to the official MCP servers. |
| [punkpeye](https://github.com/punkpeye) | Maintains [awesome-mcp-servers](https://github.com/punkpeye/awesome-mcp-servers), a broad map of the ecosystem. |
| [SecretiveShell](https://github.com/SecretiveShell) | Core contributor to the MCP server directory and several practical MCP tools. |
| [tonxxd](https://github.com/tonxxd) | Lead contributor to [mcp-use](https://github.com/mcp-use/mcp-use). |
| [pietrozullo](https://github.com/pietrozullo) | Core mcp-use contributor. |
| [khandrew1](https://github.com/khandrew1) | Core mcp-use contributor. |
| [saqadri](https://github.com/saqadri) | Creator and lead contributor to [mcp-agent](https://github.com/lastmile-ai/mcp-agent). |
| [evalstate](https://github.com/evalstate) | Core mcp-agent contributor and MCP application experimenter. |
| [cameroncooke](https://github.com/cameroncooke) | Creator and lead contributor to [XcodeBuildMCP](https://github.com/getsentry/XcodeBuildMCP). |
| [ahujasid](https://github.com/ahujasid) | Creator and lead contributor to [Blender MCP](https://github.com/ahujasid/blender-mcp). |
| [nickscamara](https://github.com/nickscamara) | Co-founder and lead contributor to [Firecrawl](https://github.com/firecrawl/firecrawl). |
| [mogery](https://github.com/mogery) | Core Firecrawl contributor. |
| [rafaelsideguide](https://github.com/rafaelsideguide) | Core Firecrawl contributor. |
| [unclecode](https://github.com/unclecode) | Creator and lead contributor to [Crawl4AI](https://github.com/unclecode/crawl4ai). |
| [ntohidi](https://github.com/ntohidi) | Core Crawl4AI contributor. |
| [ctate](https://github.com/ctate) | Lead contributor to Vercel's [agent-browser](https://github.com/vercel-labs/agent-browser). |
| [afourney](https://github.com/afourney) | Lead contributor to [MarkItDown](https://github.com/microsoft/markitdown), a useful document-normalization layer. |
| [soywod](https://github.com/soywod) | Creator and lead contributor to [Himalaya](https://github.com/pimalaya/himalaya) and the surrounding Rust email libraries. |
| [jpoehnelt](https://github.com/jpoehnelt) | Lead human contributor to the official [Google Workspace CLI](https://github.com/googleworkspace/cli). |

## Local inference and model enablement

These accounts reveal what is becoming practical on laptops, consumer GPUs, and self-hosted servers.

| Account | Public evidence |
| --- | --- |
| [mxyng](https://github.com/mxyng) | Lead contributor to [Ollama](https://github.com/ollama/ollama). |
| [dhiltgen](https://github.com/dhiltgen) | Core Ollama contributor. |
| [jmorganca](https://github.com/jmorganca) | Ollama co-founder and core contributor. |
| [BruceMacD](https://github.com/BruceMacD) | Core Ollama contributor. |
| [ggerganov](https://github.com/ggerganov) | Creator and lead contributor to [llama.cpp](https://github.com/ggml-org/llama.cpp). |
| [ngxson](https://github.com/ngxson) | Core llama.cpp contributor. |
| [JohannesGaessler](https://github.com/JohannesGaessler) | Core llama.cpp contributor focused on performance and hardware support. |
| [slaren](https://github.com/slaren) | Core llama.cpp contributor. |
| [DarkLight1337](https://github.com/DarkLight1337) | Lead contributor to [vLLM](https://github.com/vllm-project/vllm). |
| [WoosukKwon](https://github.com/WoosukKwon) | vLLM co-creator and core contributor. |
| [mgoin](https://github.com/mgoin) | Core vLLM contributor. |
| [hmellor](https://github.com/hmellor) | Core vLLM contributor. |
| [merrymercy](https://github.com/merrymercy) | Lead contributor to [SGLang](https://github.com/sgl-project/sglang). |
| [hnyls2002](https://github.com/hnyls2002) | Core SGLang contributor. |
| [fzyzcjy](https://github.com/fzyzcjy) | Core SGLang contributor. |
| [zhyncs](https://github.com/zhyncs) | Core SGLang contributor. |
| [ishaan-jaff](https://github.com/ishaan-jaff) | Creator and lead contributor to [LiteLLM](https://github.com/BerriAI/litellm). |
| [Sameerlite](https://github.com/Sameerlite) | Core LiteLLM contributor. |
| [tjbck](https://github.com/tjbck) | Creator and lead contributor to [Open WebUI](https://github.com/open-webui/open-webui). |
| [Classic298](https://github.com/Classic298) | Core Open WebUI contributor. |
| [shatfield4](https://github.com/shatfield4) | Core AnythingLLM contributor. |
| [Blaizzy](https://github.com/Blaizzy) | Creator and lead contributor to [MLX-VLM](https://github.com/Blaizzy/mlx-vlm). |
| [lucasnewman](https://github.com/lucasnewman) | Core MLX-VLM contributor and Apple-platform model toolmaker. |
| [Lazarus-931](https://github.com/Lazarus-931) | Core MLX-VLM contributor. |
| [turboderp](https://github.com/turboderp) | Creator of [ExLlama](https://github.com/turboderp-org/exllamav3), optimized local inference for consumer GPUs. |
| [awni](https://github.com/awni) | Lead contributor to Apple's public [MLX examples](https://github.com/ml-explore/mlx-examples). |
| [TimDettmers](https://github.com/TimDettmers) | Creator of bitsandbytes and widely reused low-memory training techniques. |
| [tloen](https://github.com/tloen) | Creator of [alpaca-lora](https://github.com/tloen/alpaca-lora), an early accessible fine-tuning reference. |
| [artidoro](https://github.com/artidoro) | Co-creator of [QLoRA](https://github.com/artidoro/qlora). |
| [bghira](https://github.com/bghira) | Builds practical open diffusion-model training infrastructure. |

## Retrieval, indexing, RAG, and data systems

These people build the substrate beneath personal search and agent memory.

| Account | Public evidence |
| --- | --- |
| [rescrv](https://github.com/rescrv) | Lead contributor to [Chroma](https://github.com/chroma-core/chroma). |
| [HammadB](https://github.com/HammadB) | Core Chroma contributor. |
| [codetheweb](https://github.com/codetheweb) | Core Chroma contributor and independent toolmaker. |
| [jeffchuber](https://github.com/jeffchuber) | Chroma co-founder and core contributor. |
| [wjones127](https://github.com/wjones127) | Lead contributor to [LanceDB](https://github.com/lancedb/lancedb). |
| [changhiskhan](https://github.com/changhiskhan) | Core LanceDB contributor. |
| [eddyxu](https://github.com/eddyxu) | LanceDB co-founder and core contributor. |
| [westonpace](https://github.com/westonpace) | Core LanceDB and Arrow contributor. |
| [Dev-Khant](https://github.com/Dev-Khant) | Lead contributor to [Mem0](https://github.com/mem0ai/mem0). |
| [taranjeet](https://github.com/taranjeet) | Core Mem0 contributor. |
| [kartik-mem0](https://github.com/kartik-mem0) | Core Mem0 contributor. |
| [danielaskdd](https://github.com/danielaskdd) | Lead contributor to [LightRAG](https://github.com/HKUDS/LightRAG). |
| [LarFii](https://github.com/LarFii) | Core LightRAG contributor. |
| [generall](https://github.com/generall) | Lead contributor to [Qdrant](https://github.com/qdrant/qdrant). |
| [agourlay](https://github.com/agourlay) | Core Qdrant contributor. |
| [timvisee](https://github.com/timvisee) | Core Qdrant contributor. |
| [antas-marcin](https://github.com/antas-marcin) | Lead contributor to [Weaviate](https://github.com/weaviate/weaviate). |
| [etiennedi](https://github.com/etiennedi) | Core Weaviate contributor. |
| [dirkkul](https://github.com/dirkkul) | Core Weaviate contributor. |
| [baskaryan](https://github.com/baskaryan) | Lead contributor to [LangChain](https://github.com/langchain-ai/langchain). |
| [hwchase17](https://github.com/hwchase17) | LangChain creator and core contributor. |
| [ccurme](https://github.com/ccurme) | Core LangChain contributor. |
| [logan-markewich](https://github.com/logan-markewich) | Lead contributor to [LlamaIndex](https://github.com/run-llama/llama_index). |
| [jerryjliu](https://github.com/jerryjliu) | LlamaIndex creator and core contributor. |

## Evaluation, observability, and AI security

These accounts are useful precisely because they measure failure rather than only shipping demos.

| Account | Public evidence |
| --- | --- |
| [typpo](https://github.com/typpo) | Creator and lead contributor to [Promptfoo](https://github.com/promptfoo/promptfoo). |
| [mldangelo](https://github.com/mldangelo) | Core Promptfoo contributor. |
| [sklein12](https://github.com/sklein12) | Core Promptfoo contributor. |
| [will-holley](https://github.com/will-holley) | Core Promptfoo contributor. |
| [marcklingen](https://github.com/marcklingen) | Co-founder and lead contributor to [Langfuse](https://github.com/langfuse/langfuse). |
| [maxdeichmann](https://github.com/maxdeichmann) | Co-founder and lead contributor to Langfuse. |
| [Steffen911](https://github.com/Steffen911) | Core Langfuse contributor. |
| [marliessophie](https://github.com/marliessophie) | Core Langfuse contributor. |
| [penguine-ip](https://github.com/penguine-ip) | Lead contributor to [DeepEval](https://github.com/confident-ai/deepeval). |
| [A-Vamshi](https://github.com/A-Vamshi) | Core DeepEval contributor. |
| [jwongster2](https://github.com/jwongster2) | Core DeepEval contributor. |
| [mikeldking](https://github.com/mikeldking) | Lead contributor to [Arize Phoenix](https://github.com/Arize-ai/phoenix). |
| [RogerHYang](https://github.com/RogerHYang) | Core Phoenix contributor. |
| [axiomofjoy](https://github.com/axiomofjoy) | Core Phoenix contributor. |
| [leondz](https://github.com/leondz) | Creator and lead contributor to NVIDIA's [Garak](https://github.com/NVIDIA/garak). |
| [jmartin-tech](https://github.com/jmartin-tech) | Core Garak contributor. |
| [erickgalinkin](https://github.com/erickgalinkin) | Core Garak contributor and AI-security researcher. |
| [kevinmessiaen](https://github.com/kevinmessiaen) | Co-founder and lead contributor to [Giskard](https://github.com/Giskard-AI/giskard-oss). |
| [andreybavt](https://github.com/andreybavt) | Core Giskard contributor. |
| [mattbit](https://github.com/mattbit) | Core Giskard contributor. |
| [henchaves](https://github.com/henchaves) | Core Giskard contributor. |
| [dinmukhamedm](https://github.com/dinmukhamedm) | Lead contributor to [Laminar](https://github.com/lmnr-ai/lmnr). |
| [olzhik11](https://github.com/olzhik11) | Core Laminar contributor. |
| [skull8888888](https://github.com/skull8888888) | Core Laminar contributor. |
| [hamelsmu](https://github.com/hamelsmu) | Publishes applied evaluation methods, datasets, and production AI guidance. |
| [shreyashankar](https://github.com/shreyashankar) | Researches and builds reliable data and evaluation systems for AI applications. |
| [eugeneyan](https://github.com/eugeneyan) | Publishes detailed production patterns for recommendation, retrieval, and LLM evaluation. |

## Independent experimenters and explainers

These people are useful because they publish small, legible implementations or unusually broad portfolios rather than only one flagship product.

| Account | Public evidence |
| --- | --- |
| [karpathy](https://github.com/karpathy) | Builds minimal model implementations including `nanoGPT`, `llm.c`, and educational neural-network projects. |
| [rasbt](https://github.com/rasbt) | Builds [LLMs from Scratch](https://github.com/rasbt/LLMs-from-scratch) and a large portfolio of reproducible model work. |
| [lucidrains](https://github.com/lucidrains) | Maintains hundreds of concise implementations of current model papers. |
| [philschmid](https://github.com/philschmid) | Publishes extensive applied model training, inference, and deployment examples. |
| [jph00](https://github.com/jph00) | Fast.ai co-founder and builder of literate, notebook-centered AI tools. |
| [simonw](https://github.com/simonw) | Creator of the [LLM CLI](https://github.com/simonw/llm) and hundreds of public AI experiments. |
| [swyxio](https://github.com/swyxio) | Publishes and prototypes across the AI-engineering tool ecosystem. |
| [jxnl](https://github.com/jxnl) | Creator of Instructor and a prolific source of applied structured-output, retrieval, and evaluation patterns. |
| [mrm8488](https://github.com/mrm8488) | Maintains hundreds of NLP and generative-AI experiments. |
| [willccbb](https://github.com/willccbb) | Creator of the reinforcement-learning environment and evaluation library now maintained as [Verifiers](https://github.com/PrimeIntellect-ai/verifiers). |
| [mlabonne](https://github.com/mlabonne) | Builds the [LLM Course](https://github.com/mlabonne/llm-course) and practical post-training examples. |
| [mrdbourke](https://github.com/mrdbourke) | Publishes large, executable machine-learning and AI courses. |
| [patrickloeber](https://github.com/patrickloeber) | Maintains a broad portfolio of compact model and application tutorials. |
| [geohot](https://github.com/geohot) | Leads [tinygrad](https://github.com/tinygrad/tinygrad) and publishes low-level model-compute experiments. |
| [charliermarsh](https://github.com/charliermarsh) | Builds high-performance Python tools and now works on developer infrastructure at OpenAI. |
| [BurntSushi](https://github.com/BurntSushi) | Creator of ripgrep and a useful reference for search performance, correctness, and command-line design. |

## How to expand the network

The useful graph is not “people followed by famous AI founders.” It has several edge types:

1. **Repository ownership:** start from a product with observed value and find the people who own its direction.
2. **Sustained contribution:** inspect the top human contributors, then discount vendored history, merge style, generated commits, and bots.
3. **Portfolio transfer:** inspect non-fork repositories on each profile for adjacent personal tools. This is how `gmcli`, `mail-control`, `notionfs`, and the iMessage gateway surfaced.
4. **Cross-project overlap:** prioritize people who contribute to two independent layers, such as an agent runtime plus retrieval, evaluation, or a source connector.
5. **Fork divergence:** inspect forks with meaningful changes rather than raw fork counts. The local-first Obsidian fork is one example.
6. **Release and review work:** maintainers who ship releases, review difficult pull requests, or own migrations often matter more than the raw commit leaderboard shows.
7. **Package neighborhoods:** follow dependencies and reverse dependencies around MCP frameworks, model gateways, vector stores, document parsers, and local inference engines.
8. **End-user surfaces:** mine Raycast extensions, launchers, macOS utilities, browser extensions, Obsidian plugins, and self-hosted personal tools—not only agent repositories.
9. **Evaluation authors:** follow people who publish failure cases, datasets, red-team tools, and benchmarks. Their repositories expose constraints that product demos omit.
10. **Public working style:** small tools, dotfiles, reproducible examples, issue write-ups, and construction notes reveal power use better than profile bios.
11. **Papers into code:** trace authors of influential retrieval, inference, and agent papers into their maintained implementations, then inspect collaborators.
12. **Temporal activity:** rerun the survey quarterly. A small project with three months of dense iteration can be more informative than a famous repository in maintenance mode.

## Inclusion and caution

An account belongs here when at least one public artifact shows sustained ownership, unusually deep contribution, or a portfolio of concrete AI experiments. Stars and followers are discovery signals only. Commit counts vary with repository history and do not measure code quality. Bios and employer names are never sufficient.

Organizations, bots, generated accounts, and company-only handles are omitted. The directory keeps people, because the goal is to transfer through their public work into other projects. Inclusion is not an endorsement of a project's security, license, claims, or production fitness.

The CSV records one primary category and anchor repository per person. The Markdown adds interpretation and direct links; use it before treating the CSV as a contact or recommendation list.
