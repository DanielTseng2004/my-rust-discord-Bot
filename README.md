# 🎮 Discord Game Bot & Web Frontend (Rust)

這是一個基於 Rust 開發的跨平台遊戲專案。透過將核心遊戲邏輯抽離，本專案實現了**同一個遊戲核心運算，同時在 Discord 機器人與瀏覽器網頁端遊玩**的功能。

---

## 📁 專案目錄結構

```text
HELLO-RUST/
├── src/
│   ├── bin/
│   │   ├── bot.rs          # Discord 機器人進入點（native 編譯）
│   │   └── web.rs          # Dioxus 網頁前端進入點（wasm32 編譯）
│   ├── game.rs             # 共享遊戲核心邏輯（黑傑克、數字炸彈、老虎機等）
│   └── todo.rs             # 待辦事項邏輯
├── .env                    # 環境變數設定檔（需自行建立）
├── Cargo.toml              # 專案依賴與配置
└── README.md               # 本說明文件
```

---

## ⚠️ Cargo.toml 依賴架構說明

本專案有兩個編譯目標（native bot、wasm32 web），依賴已透過 `[target.'cfg(...)'.dependencies]` **條件式分離**，避免以下衝突：

| 衝突來源                    | 問題描述                                                    | 解決方式                                  |
| --------------------------- | ----------------------------------------------------------- | ----------------------------------------- |
| `getrandom` 的 `js` feature | 只適用 wasm32，native 編譯時會報錯                          | 移至 `cfg(target_arch = "wasm32")` 區塊   |
| `dioxus` 與 `serenity` 共存 | Dioxus 0.6 拉入 WASM runtime，與 bot 的 native tokio 衝突   | `dioxus` 只在 wasm32 下引入               |
| `reqwest` 版本衝突          | Dioxus 0.6 可能拉入 0.12，與 serenity 0.12 內部的 0.11 重複 | 鎖定 0.11 並設 `default-features = false` |
| 缺少 `[[bin]]` 宣告         | 雙 binary 依賴 cargo 目錄慣例推測，容易出錯                 | 明確宣告兩個 `[[bin]]` 入口               |

> **重要**：網頁端請用 `dx serve --bin web`（由 Dioxus CLI 負責設定 wasm32 目標），**不要**用 `cargo run --bin web`，否則會以 native 目標編譯並缺少必要的 wasm 依賴。

---

## 🛠️ 前置準備與環境安裝

### 1. 安裝 Rust 環境

- **Linux / macOS**：
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Windows**：下載並執行 [rustup-init.exe](https://rustup.rs/)

### 2. 安裝 Dioxus CLI（網頁端必備）

```bash
cargo install dioxus-cli
```

確認版本為 0.6.x（與 `Cargo.toml` 中的 dioxus 版本對應）：

```bash
dx --version
```

---

## ⚙️ 設定檔配置

在專案**根目錄**（與 `Cargo.toml` 同層）建立 `.env` 檔案：

```env
# Discord 機器人 Token（至 Discord Developer Portal 申請）
DISCORD_TOKEN=your_discord_bot_token_here

# （選填）後端 API 統計同步網址，預設為 http://localhost:8000/api
API_URL=http://localhost:8000/api
```

> ⚠️ `.env` 包含敏感憑證，請確認已加入 `.gitignore`，勿上傳至公開代碼庫。

---

## 🚀 啟動與執行

### 🤖 方案 A：Discord 機器人（native）

```bash
cargo run --bin bot
```

Bot 上線後終端機會顯示 `🤖 [機器人名稱] 準備就緒！`，並自動向 Discord 註冊全域斜線指令。

### 🌐 方案 B：網頁前端（wasm32）

```bash
dx serve --bin web
```

Dioxus CLI 會自動將 Rust 編譯成 WebAssembly 並啟動本地伺服器，完成後開啟：

👉 `http://localhost:8080`

---

## 🕹️ 遊戲功能一覽

不論從 Discord 指令還是網頁按鈕觸發，底層都執行 `src/game.rs` 的相同邏輯：

1. **單人黑傑克（21點）** — 要牌 / 停牌，自動結算勝負
2. **數字炸彈（1\~100）** — 玩家與 AI 輪流猜數字，先踩爆炸失敗
3. **老虎機拉霸** — 對子、三連線、777 JACKPOT 判定
4. **猜拳大賽** — 剪刀石頭布對戰 Bot
5. **擲骰子 / 今日運勢 / 硬幣翻轉 / 俄羅斯輪盤** — 各式隨機小遊戲