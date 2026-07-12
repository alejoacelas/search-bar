<!--ai-->
# Connector coverage

All sources should emit the collector's normalized document and change-feed format. The Mini improves ingestion uptime. It does not expand what a provider permits an account to retrieve.

| Source | Collection path | Available | Important omissions or setup cost | Open action |
| --- | --- | --- | --- | --- |
| WhatsApp | Unofficial `whatsmeow` linked device | History WhatsApp transfers during pairing; subsequent linked-device events; text and common media metadata | No completeness guarantee; protocol can break; current adapter loses edits, reactions, deletions, polls, locations, stickers, view-once and disappearing content | Direct chats by phone; groups open at app level until a stable target is verified |
| Gmail | Official Gmail API with read-only OAuth | Messages, threads, headers, bodies, labels, attachments, and incremental mailbox history | One OAuth consent; attachment extraction is separate | Gmail thread URL |
| Slack | Official Web API with user-authorized read scopes | Public channels plus private channels and DMs the token may access | Workspace administrator may forbid scopes; history rate limits can make a large first import take days; retention plans hide older messages | Message permalink |
| Telegram | Official MTProto user API or TDLib | Cloud chats and supported media available to the account | Requires an API ID, phone login and possibly 2FA; Secret Chats are device-specific and unavailable through normal cloud history; third-party clients are monitored for abuse | Telegram message link where supported |
| Signal | Unofficial read-only adapter to a linked Signal Desktop installation | Whatever history exists in that Desktop profile and can be decoded by the adapter | No supported personal-history API; private schema/encryption can change; sealed or expired content stays unavailable | Conversation-level opening at best |
| Apple Notes | AppleScript/JXA automation on a Mac logged into the Apple account | Note title, body, folder, modification date, and attachments exposed by Notes scripting | Requires macOS Automation permission; must remain on a Mac; locked notes and some rich objects may be unavailable | `notes://` or Notes activation plus scripted selection after verification |
| Google Keep | Official Keep API only for managed Google Workspace use | Enterprise-admin note management where the domain grants the required authority | The official API is designed for enterprise administration, not ordinary consumer Keep accounts; a personal connector would require export or unsupported browser automation | Keep web URL if a stable note URL is available |

## Build order after WhatsApp

1. Gmail. Its history cursor maps directly onto our change feed and tests rich message/thread retrieval.
2. Apple Notes. The Mini is the correct collection host because Notes automation requires macOS and the signed-in Apple account.
3. Telegram. Use TDLib in read-only mode and explicitly exclude sending and Secret Chats.
4. Slack. Reuse the existing Slack authorization work in this workspace, but expect administrator and rate-limit differences per workspace.
5. Google Keep export importer. Do not promise live consumer-account synchronization unless Google exposes a supported path.
6. Signal feasibility spike. Stop if two routine Signal Desktop updates require adapter repair.

## Primary references

- [Gmail API resources and mailbox history](https://developers.google.com/workspace/gmail/api/reference/rest)
- [Slack conversation history, scopes, access, and current rate limits](https://api.slack.com/methods/conversations.history)
- [Telegram API and third-party user authorization](https://core.telegram.org/api)
- [Telegram message search and media filters](https://core.telegram.org/api/search)
- [Google Keep API overview and enterprise-administration scope](https://developers.google.com/workspace/keep/api/guides)
- [Apple macOS automation scripting](https://developer.apple.com/library/archive/documentation/LanguagesUtilities/Conceptual/MacAutomationScriptingGuide/)
- [Signal backups and linked-device history constraints](https://support.signal.org/hc/en-us/articles/10074659364122-Backups-and-Device-Transfers-on-Signal)
<!--/ai-->
