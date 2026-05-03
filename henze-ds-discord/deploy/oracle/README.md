# Oracle VM deployment for henze-ds-discord

This setup runs multiple isolated Discord bot instances on one VM with Docker Compose.
Each instance uses its own environment file and its own SQLite file under a separate state directory.

## What this implementation includes

- GitHub Actions CD workflow via SSH upload + remote deploy.
- Dedicated Discord bot container image at docker/discord-bot.Dockerfile.
- Multi-instance compose stack with two profiles: alpha and beta.
- Per-instance SQLite separation using distinct bind-mounted data directories.
- Operational control script for start/stop/restart/status/logs and backups.

## VM bootstrap

From your Oracle VM:

```bash
bash deploy/oracle/bin/bootstrap_vm.sh
```

## Runtime directory layout on VM

The deploy script expects:

- /opt/henze-discord/runtime/instances
- /opt/henze-discord/runtime/state
- /opt/henze-discord/runtime/backups

Create per-instance env files on the VM only:

```bash
cp deploy/oracle/instances/alpha.env.example /opt/henze-discord/runtime/instances/alpha.env
cp deploy/oracle/instances/beta.env.example /opt/henze-discord/runtime/instances/beta.env
```

Then edit secrets in those files.

## GitHub repository configuration

Set these GitHub repository secrets:

- ORACLE_VM_HOST: VM public IP or DNS
- ORACLE_VM_USER: SSH user
- ORACLE_VM_SSH_KEY: private key for SSH login
- ORACLE_VM_SSH_PORT: optional (defaults to 22)

Optional GitHub repository variables:

- ORACLE_VM_APP_DIR (default: /opt/henze-discord/app)
- ORACLE_VM_RUNTIME_DIR (default: /opt/henze-discord/runtime)

## Deploy

Push to main or manually run workflow:

- .github/workflows/oracle-vm-discord-deploy.yml

## Operate instances on VM

```bash
# Status
bash deploy/oracle/bin/botctl.sh alpha status

# Restart
bash deploy/oracle/bin/botctl.sh alpha restart

# Trigger immediate run without waiting for cron
bash deploy/oracle/bin/botctl.sh alpha trigger-now

# Backup SQLite DB
bash deploy/oracle/bin/botctl.sh alpha backup-db
```

## Resource guidance for 1 OCPU / 1 GB RAM

Current compose limits reserve roughly:

- alpha: 320 MB, 0.45 CPU
- beta: 320 MB, 0.45 CPU
- remaining headroom: OS + Docker overhead

If memory pressure appears, run only one instance (only alpha.env) or lower settlement frequency.

## Cloudflare Access integration

This implementation keeps bot services private on an internal Docker network.
For remote operations, use Cloudflare Access over SSH to the VM, then run botctl locally.

A dedicated HTTP admin API can be layered on top in the next phase if you want browser/API-based operations instead of SSH-gated control.
