#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
COMPOSE_FILE="$REPO_ROOT/deploy/oracle/docker-compose.yml"
PROJECT_NAME="henze-discord"
RUNTIME_DIR="${RUNTIME_DIR:-/opt/henze-discord/runtime}"
INSTANCE_CONFIG_DIR="${INSTANCE_CONFIG_DIR:-$RUNTIME_DIR/instances}"
INSTANCE_STATE_DIR="${INSTANCE_STATE_DIR:-$RUNTIME_DIR/state}"

mkdir -p "$INSTANCE_CONFIG_DIR" "$INSTANCE_STATE_DIR/alpha" "$INSTANCE_STATE_DIR/beta"

if ! command -v docker >/dev/null 2>&1; then
  echo "docker is required on the VM"
  exit 1
fi

if ! docker compose version >/dev/null 2>&1; then
  echo "docker compose v2 is required on the VM"
  exit 1
fi

instances=()
for env_file in "$INSTANCE_CONFIG_DIR"/*.env; do
  if [[ -f "$env_file" ]]; then
    name="$(basename "$env_file" .env)"
    case "$name" in
      alpha|beta)
        instances+=("$name")
        ;;
      *)
        echo "Skipping unknown instance env file: $env_file"
        ;;
    esac
  fi
done

if [[ ${#instances[@]} -eq 0 ]]; then
  echo "No enabled instances found in $INSTANCE_CONFIG_DIR (expected alpha.env and/or beta.env)."
  echo "Create env files from deploy/oracle/instances/*.env.example first."
  exit 1
fi

compose_cmd=(docker compose -f "$COMPOSE_FILE" --project-name "$PROJECT_NAME")
for instance in "${instances[@]}"; do
  compose_cmd+=(--profile "$instance")
done

export INSTANCE_CONFIG_DIR
export INSTANCE_STATE_DIR

"${compose_cmd[@]}" build --pull
"${compose_cmd[@]}" up -d --remove-orphans
"${compose_cmd[@]}" ps
