# Terminal Stock Ticker (终端盯盘神器)

A blazing fast, locally configurable, high-performance terminal stock ticker written in **Rust**. Built specifically to cleanly track your favorite assets—such as Chinese A-shares, Hong Kong Stocks, US Stocks, and Futures/ETFs—right from your terminal, without eating up system resources!

This project utilizes `ratatui` for an interactive TUI (Text User Interface) and uses the Sina Finance API to provide millisecond-latency price quotes without requiring any API keys.

## ✨ Features

- **Multi-market Support**: Automatically decodes A-shares, Hong Kong Stocks, US Stocks, and Futures (e.g., Gold).
- **Asynchronous Architecture**: Network fetching runs in an isolated background Tokio thread. The terminal UI never stutters, guaranteeing maximum fluidity.
- **Categorized Tabs**: Supports grouping your stocks by custom categories via interactive Tabs. Switch tabs seamlessly with the **`←` / `→` arrow keys** or **`Tab`**.
- **Interactive Sorting**: Dynamic, on-the-fly table sorting!
  - Press **`c`** to sort by Change Percentage (涨跌幅).
  - Press **`n`** to sort alphabetically by Name.
  - Press **`r`** to restore the default order.
- **Customizable Configuration**: A `config.json` file is automatically generated in your directory upon first launch. You can freely edit this file to add, rename, or reorganize your favorite stocks—no recompilation needed!
- **Visual Encoding**: Prices automatically turn Red/Green according to Chinese stock market conventions (Red for positive, Green for negative).

## 🚀 Pre-requisites

- **Rust toolchain** (`cargo` must be installed). If not, get it from [rustup.rs](https://rustup.rs/).

## 🛠️ Usage & Installation

### Option 1: Run Natively / Compile Locally
```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/stock_ticker.git
cd stock_ticker

# Run directly
cargo run

# Or build the release binary
cargo build --release
./target/release/stock_ticker
```

### Option 2: Run in WSL (Windows Subsystem for Linux)
If you are developing on Windows and want a true Linux TUI experience:
```bash
wsl cargo build --release
cp ./target/release/stock_ticker /path/to/your/linux/workspace
./stock_ticker
```

## ⚙️ Configuration (`config.json`)

The first time you run `./stock_ticker`, it generates a `config.json` next to the executable. Edit the file to customize your ticker list. Standard ticker prefixes are strongly recommended:
- `sh600036` (Shanghai A-share)
- `sz002594` (Shenzhen A-share)
- `rt_hk00700` (Hong Kong Stock)
- `gb_aapl` (US Stock)
- `hf_GC` (Futures/Gold)

Enjoy discreetly monitoring the market! Press **`q`** to quit anytime.

## License
MIT
