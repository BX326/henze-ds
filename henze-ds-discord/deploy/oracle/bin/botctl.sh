#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  botctl.sh <instance> <action>

Instances:
  alpha | beta

Actions:
  status      Show container status
  start       Start instance
  stop        Stop instance
  restart     Restart instance
  logs        Tail logs
  trigger-now Run one-off immediate post (--now)
  db-size     Show SQLite file size
  backup-db   Create a timestamped SQLite backup
EOF
}

if [[ $# -lt 2 ]]; then
  usage
  exit 1
fi

INSTANCE="$1"
ACTION="$2"

case "$INSTANCE" in
  alpha|beta) ;;
  *)
    echo "Unknown instance: $INSTANCE"
    exit 1
    ;;
esac

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
COMPOSE_FILE="$REPO_ROOT/deploy/oracle/docker-compose.yml"
PROJECT_NAME="henze-discord"
RUNTIME_DIR="${RUNTIME_DIR:-/opt/henze-discord/runtime}"
INSTANCE_CONFIG_DIR="${INSTANCE_CONFIG_DIR:-$RUNTIME_DIR/instances}"
INSTANCE_STATE_DIR="${INSTANCE_STATE_DIR:-$RUNTIME_DIR/state}"
SERVICE="bot_${INSTANCE}"
DB_FILE="$INSTANCE_STATE_DIR/$INSTANCE/henze_bets.db"

mkdir -p "$INSTANCE_STATE_DIR/$INSTANCE" "$RUNTIME_DIR/backups"

export INSTANCE_CONFIG_DIR
export INSTANCE_STATE_DIR

compose_base=(docker compose -f "$COMPOSE_FILE" --project-name "$PROJECT_NAME" --profile "$INSTANCE")

case "$ACTION" in
  status)
    "${compose_base[@]}" ps "$SERVICE"
    ;;
  start)
    "${compose_base[@]}" up -d "$SERVICE"
    ;;
  stop)
    "${compose_base[@]}" stop "$SERVICE"
    ;;
  restart)
    "${compose_base[@]}" restart "$SERVICE"
    ;;
  logs)
    "${compose_base[@]}" logs --tail 200 -f "$SERVICE"
    ;;
  trigger-now)
    "${compose_base[@]}" run --rm "$SERVICE" --now
    ;;
  db-size)
    if [[ -f "$DB_FILE" ]]; then
      du -h "$DB_FILE"
    else
      echo "No database found yet at $DB_FILE"
      exit 1
    fi
    ;;
  backup-db)
    if [[ ! -f "$DB_FILE" ]]; then
      echo "No database found yet at $DB_FILE"
      exit 1
    fi
    ts="$(date +%Y%m%d_%H%M%S)"
    backup_path="$RUNTIME_DIR/backups/${INSTANCE}_henze_bets_${ts}.sqlite"
    cp "$DB_FILE" "$backup_path"
    echo "Created backup: $backup_path"
    ;;
  *)
    echo "Unknown action: $ACTION"
    usage
    exit 1
    ;;
esac
