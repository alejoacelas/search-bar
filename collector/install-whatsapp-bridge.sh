#!/bin/zsh
set -euo pipefail

cd "${0:A:h}"

revision=7d6a06dcdce1f01dfb24f60e1030d5efba9f3b88
repository=https://github.com/lharries/whatsapp-mcp.git
whatsmeow=v0.0.0-20260709092057-73fe7355f59f

if ! command -v go >/dev/null; then
  echo "Go is required. Install it with: brew install go" >&2
  exit 1
fi

if [[ ! -d whatsapp-mcp/.git ]]; then
  git clone "$repository" whatsapp-mcp
fi

git -C whatsapp-mcp fetch origin "$revision"
if ! git -C whatsapp-mcp diff --quiet; then
  if git -C whatsapp-mcp apply --reverse --check ../patches/disable-whatsapp-actions.patch; then
    git -C whatsapp-mcp apply --reverse ../patches/disable-whatsapp-actions.patch
  else
    echo "The WhatsApp bridge has changes not made by this installer; preserving them and stopping." >&2
    exit 1
  fi
fi
git -C whatsapp-mcp checkout --detach "$revision"
git -C whatsapp-mcp apply ../patches/disable-whatsapp-actions.patch

mkdir -p data/bin
(
  cd whatsapp-mcp/whatsapp-bridge
  go get "go.mau.fi/whatsmeow@$whatsmeow"
  go mod tidy
  go build -trimpath -o ../../data/bin/whatsapp-bridge .
)

echo "$PWD/data/bin/whatsapp-bridge"
