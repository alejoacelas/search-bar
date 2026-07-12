#!/bin/zsh
set -euo pipefail

if (( $# != 2 )); then
  echo "Usage: $0 <collector-url> <token-file>" >&2
  exit 1
fi

url=$1
token_file=$2
if [[ ! -s "$token_file" ]]; then
  echo "Token file is missing or empty: $token_file" >&2
  exit 1
fi

config="$HOME/Library/Application Support/search-bar/collector.json"
mkdir -p "${config:h}"
umask 077
jq -n --arg url "$url" --arg token "$(<"$token_file")" '{url: $url, token: $token}' > "$config"
chmod 600 "$config"
echo "$config"
