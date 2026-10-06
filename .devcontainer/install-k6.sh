#!/usr/bin/env bash
# Installs the k6 load testing tool (https://k6.io) from the official GitHub release.
set -euo pipefail

K6_VERSION="${K6_VERSION:-v2.3.0}"

if command -v k6 >/dev/null 2>&1; then
	echo "k6 already installed: $(k6 version)"
	exit 0
fi

case "$(uname -m)" in
	x86_64) arch=amd64 ;;
	aarch64 | arm64) arch=arm64 ;;
	*) echo "Unsupported architecture: $(uname -m)" >&2; exit 1 ;;
esac

name="k6-${K6_VERSION}-linux-${arch}"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

curl -fsSL "https://github.com/grafana/k6/releases/download/${K6_VERSION}/${name}.tar.gz" | tar -xz -C "$tmp"
sudo install -m 0755 "$tmp/${name}/k6" /usr/local/bin/k6
k6 version
