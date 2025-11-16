# Market Discoverer

A Rust application that monitors blockchain network data from various APIs and sends notifications via Telegram when changes are detected.

This project can be used to track changes in JSON arrays, for my case, I'm using it for discovering appearing blockchains.

## Features

-   Monitors multiple blockchain network data sources (Ethereum List, Geckoterminal, Stargate Chains)
-   Detects additions and removals of network entries
-   Sends Telegram notifications on changes
-   Caches previous state for comparison
-   Supports multiple data source types through a generic architecture

## Prerequisites

-   Rust (latest stable version)
-   Cargo (comes with Rust)
-   Telegram Bot Token and Chat ID (for notifications)

## Setup

### 1. Install Rust

If you don't have Rust installed, follow the instructions at [rustup.rs](https://rustup.rs/).

### 2. Clone and Build

```bash
git clone <repository-url>
cd market-discoverer
cargo build --release
```

### 3. Configure Environment Variables

Create a `.env` file in the project root with your Telegram credentials:

```bash
TG_BOT_TOKEN=your_bot_token_here
TG_CHAT_ID=your_chat_id_here
TG_THREAD_ID=your_thread_id_here
```

**Getting Telegram Credentials:**

-   **Bot Token**: Create a bot via [@BotFather](https://t.me/botfather) on Telegram
-   **Chat ID**: Use [@userinfobot](https://t.me/userinfobot) to get your chat ID
-   **Thread ID**: Optional, for thread-specific notifications in groups

### 4. Configure Data Sources

Configuration files are located in the `config/` directory. Example configurations are provided:

-   `config/ethereum_list.toml` - Ethereum chain list
-   `config/geckoterminal.toml` - Geckoterminal networks
-   `config/stargate_chains.toml` - Stargate chains

Each config file follows this format:

```toml
name = "Source Name"
data_endpoint_url = "https://api.example.com/endpoint"
cache_file_path = "cache/filename.json"
```

## Usage

### Basic Usage

Run the application with a configuration file:

```bash
cargo run --release config/ethereum_list.toml
```

Or use the compiled binary:

```bash
./target/release/market-discoverer config/ethereum_list.toml
```

### Available Configurations

```bash
# Monitor Ethereum chain list
cargo run --release config/ethereum_list.toml

# Monitor Geckoterminal networks
cargo run --release config/geckoterminal.toml

# Monitor Stargate chains
cargo run --release config/stargate_chains.toml
```

### Logging

Set the log level using the `RUST_LOG` environment variable:

```bash
# Debug logging
RUST_LOG=debug cargo run --release config/ethereum_list.toml

# Info logging (default)
RUST_LOG=info cargo run --release config/ethereum_list.toml

# Error logging only
RUST_LOG=error cargo run --release config/ethereum_list.toml
```

## How It Works

1. **Initialization**: On first run, the application fetches data from the configured API and stores it in the cache file
2. **Monitoring**: On subsequent runs, it compares the current API response with the cached data
3. **Detection**: Identifies added and removed entries
4. **Notification**: Sends Telegram messages for each change detected
5. **Update**: Updates the cache file with the latest data

## Project Structure

```
market-discoverer/
├── src/
│   ├── main.rs              # Main entry point
│   ├── lib.rs               # Library root
│   ├── config.rs            # Configuration parsing
│   ├── fetch.rs             # API data fetching
│   ├── process.rs           # Core processing logic
│   ├── json.rs              # JSON file operations
│   ├── telegram.rs          # Telegram notifications
│   └── types/               # Type definitions
│       ├── common.rs        # Common traits (Entry, ResponseData)
│       ├── ethereum_list/   # Ethereum List types
│       ├── geckoterminal/   # Geckoterminal types
│       └── stargate_api/    # Stargate API types
├── config/                  # Configuration files
├── cache/                   # Cached data (gitignored)
└── Cargo.toml              # Rust dependencies
```

## Adding New Data Sources

To add a new data source:

1. Create type definitions in `src/types/<source_name>/`
2. Implement `ResponseData` and `Entry` traits
3. Create a config file in `config/`
4. Update the main binary to use the new type

## License

MIT, bruv
