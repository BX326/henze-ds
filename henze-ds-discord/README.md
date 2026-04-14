# Henze Discord Bot

A Discord bot that posts daily Henze bets to a specified channel.

## Features

- Fetches Henze bets with odds around 1.10 (±0.04)
- **AI-powered bet selection** using OpenAI GPT-4o-mini to pick the most interesting bets
- Falls back to random selection if OpenAI is not configured
- Posts 3 curated bets daily at a configurable time
- Beautiful embed messages with bet details and links

## Configuration

Set the following environment variables:

| Variable | Required | Description | Example |
|----------|----------|-------------|---------|
| `DISCORD_TOKEN` | Yes | Your Discord bot token | `MTIz...` |
| `DISCORD_CHANNEL_ID` | Yes | The channel ID to post bets to | `123456789012345678` |
| `CRON_SCHEDULE` | No | Cron expression for daily posting | `0 0 8 * * *` (default: 8:00 AM UTC) |
| `OPENAI_API_KEY` | No | OpenAI API key for AI-powered selection | `sk-...` |
| `SYSTEM_PROMPT` | No | Custom AI system prompt (overrides default) | `You are a betting expert...` |

## Setup

### 1. Create a Discord Bot

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications)
2. Click "New Application" and give it a name
3. Go to "Bot" section and click "Add Bot"
4. Copy the bot token (keep this secret!)
5. Enable these Privileged Gateway Intents:
   - Message Content Intent
6. Go to "OAuth2" > "URL Generator"
7. Select scopes: `bot`
8. Select permissions: `Send Messages`, `Embed Links`
9. Copy the generated URL and use it to invite the bot to your server

### 2. Get the Channel ID

1. Enable Developer Mode in Discord (Settings > Advanced > Developer Mode)
2. Right-click on the channel where you want daily bets posted
3. Click "Copy ID"

### 3. Run the Bot

```bash
# Set environment variables
export DISCORD_TOKEN="your_bot_token_here"
export DISCORD_CHANNEL_ID="your_channel_id_here"
export CRON_SCHEDULE="0 0 8 * * *"  # Optional: defaults to 8 AM UTC

# Run the bot (will post at scheduled times)
cargo run -p henze-ds-discord

# Or send a message immediately for testing
cargo run -p henze-ds-discord -- --now
```

## Cron Schedule Examples

| Schedule | Cron Expression |
|----------|-----------------|
| 8:00 AM UTC daily | `0 0 8 * * *` |
| 10:00 AM UTC daily | `0 0 10 * * *` |
| 6:00 PM UTC daily | `0 0 18 * * *` |
| Every hour | `0 0 * * * *` |

## Docker

You can also run the bot using Docker:

```bash
docker build -t henze-discord-bot .
docker run -e DISCORD_TOKEN=... -e DISCORD_CHANNEL_ID=... -e OPENAI_API_KEY=... henze-discord-bot
```

## AI-Powered Bet Selection

When `OPENAI_API_KEY` is set, the bot uses GPT-4o-mini to intelligently select bets based on:

1. **High-profile matches** - Prioritizes major leagues and popular teams
2. **Clear market types** - Prefers straightforward markets like Match Result, Over/Under
3. **Timing** - Considers when each match starts
4. **Variety** - Diversifies across different sports and markets
5. **Live matches** - Gives priority to exciting live matches from major leagues

If the AI selection fails or `OPENAI_API_KEY` is not set, the bot falls back to random selection.
