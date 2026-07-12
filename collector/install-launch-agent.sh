#!/bin/zsh
set -euo pipefail

cd "${0:A:h}"

host=${1:-127.0.0.1}

if [[ ! -x data/bin/whatsapp-bridge ]]; then
  echo "Run ./install-whatsapp-bridge.sh first." >&2
  exit 1
fi

label=com.alejo.search-bar.collector
destination="$HOME/Library/LaunchAgents/$label.plist"
logs="$HOME/Library/Logs/Search Bar"
mkdir -p "${destination:h}" "$logs"
cp launchd/$label.plist.in "$destination"
perl -pi -e 's|__RUNNER__|'"$PWD/run-local.sh"'|g; s|__DIRECTORY__|'"$PWD"'|g; s|__LOG_DIRECTORY__|'"$logs"'|g; s|__HOST__|'"$host"'|g' "$destination"
plutil -lint "$destination"

launchctl bootout "gui/$UID/$label" 2>/dev/null || true
launchctl bootstrap "gui/$UID" "$destination"
launchctl kickstart -k "gui/$UID/$label"

echo "$destination"
echo "Logs: $logs"
