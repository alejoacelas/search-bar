<!--ai-->
# Collector setup

Search Bar searches a local replica. The collector runs continuously, converts each source into one document format, and exposes only an authenticated incremental feed. Moving it from this Mac to a Mac Mini changes its URL, not the search or source schema.

## Try WhatsApp on this Mac

Prerequisites: Python 3, `jq`, and Go. Install Go if needed:

```sh
brew install go
```

Build the pinned WhatsApp bridge:

```sh
cd collector
./install-whatsapp-bridge.sh
```

The installer checks out `lharries/whatsapp-mcp` at commit `7d6a06dcdce1f01dfb24f60e1030d5efba9f3b88`, updates its outdated protocol dependency to pinned `whatsmeow` commit `73fe7355f59f`, and applies the repository's compatibility and read-only patch. That patch disables the upstream unauthenticated REST server and every sending route. Review both before pairing:

```sh
git -C whatsapp-mcp diff
cat patches/disable-whatsapp-actions.patch
```

Start the bridge and collector:

```sh
./run-local.sh
```

Scan the terminal QR code from WhatsApp → Settings → Linked Devices. The first history synchronization can take several minutes. The collector writes:

- WhatsApp session and source data under `collector/whatsapp-mcp/whatsapp-bridge/store/`;
- normalized documents and changes to `collector/data/collector.sqlite3`;
- a 256-bit bearer token to `collector/data/token`.

Point Search Bar at it, then restart Search Bar:

```sh
./scripts/configure-collector.sh http://127.0.0.1:8742 collector/data/token
```

Search Bar synchronizes every ten seconds. A collector outage does not block queries; the interface continues to search the last local replica and says that the collector is offline.

After pairing and verifying searches, install the collector as a login service:

```sh
cd collector
./install-launch-agent.sh
```

Its logs are under `~/Library/Logs/Search Bar/`. The generated launch agent uses absolute paths to this checkout; reinstall it after moving the repository.

## Move it to a Mac Mini

1. Install this repository, Go, Python, and Tailscale on the Mini.
2. Run the bridge installer and pair WhatsApp again. This consumes one WhatsApp linked-device slot.
3. Install the login service on the Mini's Tailscale address, not `0.0.0.0` or a public interface:

```sh
cd collector
./install-launch-agent.sh <mini-tailscale-ip>
```

4. Copy only the token to the client Mac through an authenticated channel, then run:

```sh
./scripts/configure-collector.sh http://<mini-tailscale-ip>:8742 <copied-token-file>
```

Tailscale supplies transport encryption and device authentication. The bearer token is an additional application credential. The collector uses plain HTTP inside that tunnel; do not expose port 8742 directly to a LAN or the internet.

## What is implemented

- Incremental, idempotent WhatsApp imports from the bridge database.
- An append-only, cursor-based change feed with constant-time bearer-token comparison.
- Loopback-only binding by default.
- Local Search Bar replication; no network request occurs while typing.
- Visible offline behavior.
- WhatsApp direct-chat opening where a phone-number target exists.
- Text and metadata for images, video, audio, and documents received by the upstream bridge.

## Remaining before unattended Mini operation

- A signed application installer. The current launchd service is tied to this checkout's absolute path.
- A pairing screen instead of terminal QR output and a configuration script.
- Keychain storage for the collector token. The current configuration file is mode `0600`.
- Eager media download and replication.
- Source health records that distinguish logged out, syncing, caught up, and possible history gaps.
- Automatic bridge updates after compatibility testing. Updates must remain pinned; following upstream `main` would let an unaudited protocol client read correspondence.
- Deletion, edit, reaction, disappearing-message, and unsupported-message events. The reference bridge does not preserve these faithfully.
<!--/ai-->
