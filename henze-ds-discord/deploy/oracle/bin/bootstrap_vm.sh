#!/usr/bin/env bash
set -euo pipefail

# Ubuntu 22.04+/Debian bootstrap for Docker + Compose plugin.
if ! command -v apt-get >/dev/null 2>&1; then
  echo "This bootstrap currently supports apt-based distributions only."
  exit 1
fi

sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  ca-certificates \
  curl \
  gnupg \
  lsb-release

if ! command -v docker >/dev/null 2>&1; then
  . /etc/os-release
  case "$ID" in
    ubuntu)
      docker_repo="https://download.docker.com/linux/ubuntu"
      ;;
    debian)
      docker_repo="https://download.docker.com/linux/debian"
      ;;
    *)
      echo "Unsupported distro for automated Docker repo setup: $ID"
      exit 1
      ;;
  esac

  sudo install -m 0755 -d /etc/apt/keyrings
  curl -fsSL "$docker_repo/gpg" | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
  sudo chmod a+r /etc/apt/keyrings/docker.gpg

  echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] $docker_repo \
    $VERSION_CODENAME stable" | \
    sudo tee /etc/apt/sources.list.d/docker.list >/dev/null

  sudo apt-get update
  sudo apt-get install -y --no-install-recommends \
    docker-ce \
    docker-ce-cli \
    containerd.io \
    docker-buildx-plugin \
    docker-compose-plugin
fi

if ! groups "$USER" | grep -q '\bdocker\b'; then
  sudo usermod -aG docker "$USER"
  echo "Added $USER to docker group. Log out and back in once before using docker without sudo."
fi

sudo mkdir -p /opt/henze-discord/runtime/instances /opt/henze-discord/runtime/state
sudo chown -R "$USER":"$USER" /opt/henze-discord

echo "Bootstrap completed."
