<div align="center">
  <img src="" width="140" alt="Seawatch Logo" />
  <h1>🌊☠️ Seawatch</h1>
  <p><strong>Real-time Oracle Data • Analytics • Backtesting • Rust-Powered</strong></p>
</div>

---

### ⚓ Overview

**Seawatch** is a modular, pirate-themed Rust engine that sails the seas of on-chain data.  
It pulls live prices from the [Pyth Network](https://pyth.network).

---

### 🧩 Workspace Architecture

```
seawatch/
├─ Cargo.toml                # Workspace manifest
└─ crates/
   ├─ types/                 # Shared structs (ticks, candles, frames)
   ├─ pyth-client/           # Fetches Pyth Network oracle data
   ├─ indicators/            # RSI, Bollinger, and TA logic
   ├─ ws-server/             # Axum WebSocket broadcaster
   ├─ backtest/              # (WIP) Historical strategy simulator
   ├─ seawatchd/             # The daemon – runs the engine
   └─ seawatch-gui/          # (Optional) Desktop GUI built with egui / Tauri
```

Each crate is its own Rust package, making Seawatch easy to extend, test, and deploy.

---

### ⚙️ Features

- ⚡ **Real-time price feed** via Pyth Network’s Hermes API  
- 📡 **WebSocket daemon** for broadcasting normalized ticks & indicators  
- 📊 **Technical analysis** (RSI, Bollinger Bands, SMA/EMA)  
- 📚 **Modular workspace** with reusable crates  
- 🧠 **Backtesting engine** (in progress)  
- 🖥️ **GUI dashboard** with live plots and timeframes  
- 🔒 **Pure Rust** — safe, concurrent, and blazing fast

---

### 🛠️ Getting Started

#### 1. Clone & build

```bash
git clone https://github.com/CalicoNino/seawatch.git
cd seawatch
cargo build --release
```

#### 2. Run the daemon

```bash
cargo run -p seawatchd
# WebSocket will start on ws://127.0.0.1:8787/ws
```

#### 3. Connect the GUI (optional)

```bash
cargo run -p seawatch-gui
```

---

### 🧭 Example: Subscribe to live ticks

Once the daemon is running, connect to its WebSocket endpoint:

```bash
wscat -c ws://127.0.0.1:8787/ws
```

You’ll see JSON messages like:

```json
{
  "type": "Tick",
  "data": {
    "feed_id": "0xff6149...ace",
    "price": 67123.42,
    "conf": 1.2,
    "publish_time": 1720000123
  }
}
```

---

### 📈 Roadmap

| Milestone | Status | Notes |
|------------|:-------:|-------|
| Pyth live polling | ✅ | Hermes REST integration |
| WebSocket broadcaster | ✅ | Axum-based server |
| RSI & Bollinger computation | ⚙️ | via `ta` crate |
| Candle aggregation | 🧱 | multi-TF buckets |
| GUI dashboard | 🧭 | using egui / Tauri |
| Historical backtesting | 🧪 | in development |

---

### ⚔️ Tech Stack

- **Language:** Rust (Edition 2021 / 2024)
- **Runtime:** Tokio async
- **Networking:** Axum, Reqwest
- **Analytics:** [`ta`](https://crates.io/crates/ta)
- **Frontend:** `egui` / `eframe`
- **Logging:** Tracing + EnvFilter

---

### ☠️‍💻 Author

**Calico Nino**  
📬 <caliconino@duck.com>

---

### ⚖️ License

Licensed under the **MIT License** — see [`LICENSE`](LICENSE) for details.

---
