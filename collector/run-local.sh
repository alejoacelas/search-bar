#!/bin/zsh
set -euo pipefail

cd "${0:A:h}"

bridge=data/bin/whatsapp-bridge
messages=whatsapp-mcp/whatsapp-bridge/store/messages.db
host=${SEARCH_BAR_COLLECTOR_HOST:-127.0.0.1}

if [[ ! -x "$bridge" ]]; then
  echo "Run ./install-whatsapp-bridge.sh first." >&2
  exit 1
fi

mkdir -p data whatsapp-mcp/whatsapp-bridge/store

(cd whatsapp-mcp/whatsapp-bridge && ../../data/bin/whatsapp-bridge) &
bridge_pid=$!
collector_pid=
trap '[[ -n "$collector_pid" ]] && kill "$collector_pid" 2>/dev/null || true; kill "$bridge_pid" 2>/dev/null || true' EXIT INT TERM

python3 collector.py \
  --database data/collector.sqlite3 \
  serve \
  --host "$host" \
  --token-file data/token \
  --whatsapp-database "$messages" &
collector_pid=$!
wait "$collector_pid"
