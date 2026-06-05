# 🎮 Discord Game Bot & Web Frontend (Rust)

這是一個基於 Rust 開發的跨平台遊戲專案。透過將核心遊戲邏輯完全抽離至共享模組，本專案實現了**同一個遊戲核心運算（狀態機與 AI 邏輯），同時在 Discord 機器人與瀏覽器網頁端遊玩**的功能。

---

## 📁 專案目錄結構

```text
HELLO-RUST/
├── src/
│   ├── bin/
│   │   ├── bot.rs          # Discord 機器人進入點（native 執行）
│   │   └── web.rs          # Dioxus 網頁前端進入點（WASM/Dioxus 0.6 渲染）
│   ├── game.rs             # 共享遊戲核心邏輯（黑傑克、數字炸彈、德州撲克、Wordle、TTT等）
│   ├── todo.rs             # 待辦事項邏輯（Bot端共用）
│   └── lib.rs              # 專案庫根目錄（定義公開模組介面，告別脆弱的 path 引用）
├── .env                    # 環境變數設定檔（需自行建立）
├── Cargo.toml              # 專案依賴與多目標 Feature 配置
└── README.md               # 本說明文件

```

---

## ⚠️ Cargo.toml 依賴與架構說明

為了避免跨平台編譯（Native 與 WebAssembly）環境衝突，專案在 `Cargo.toml` 中採取了嚴格的 **Optional Dependencies + Feature Flags** 策略，從根本上解決了以下 Rust 跨平台痛點：

| 衝突與痛點來源                 | 問題描述                                                                               | 最新優化解決方式                                                                |
| ------------------------------ | -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| `dioxus` 與 `serenity` 共存    | Dioxus 0.6 的全端架構若直接載入 native，會與 bot 的 tokio 發生多執行緒衝突。           | 將 `dioxus` 設為 `optional`，唯有在啟用 `web` feature 時才會被編譯引入。        |
| 缺少 `[[bin]]` 與 Feature 綁定 | 雙 Binary 入口若直接以 `cargo run` 執行網頁端，會因缺少 Dioxus 的 feature 宣告而報錯。 | 在 `Cargo.toml` 明確綁定 `required-features = ["web"]` 至 `web` binary 入口。   |
| 編輯器語法紅線錯誤             | `rust-analyzer` 在預設 Native 目標下無法解析 Web 元件。                                | 專案依賴改採 `discord-game-bot` 的 crate 庫引用架構（透過 `lib.rs` 暴露介面）。 |

> **🔥 網頁端開發重要警告**：網頁端本地測試時請務必使用 `cargo check --bin web --features web` 進行型別檢查。實際執行請使用 **Dioxus CLI** 工具 `dx serve`，它會自動打通 WASM 編譯目標。

---

## 🛠️ 前置準備與環境安裝

### 1. 安裝 Rust 環境

* **Linux / macOS**：
  
```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh

```

* **Windows**：下載並執行 [rustup-init.exe](https://rustup.rs/)

### 2. 安裝 Dioxus CLI（網頁端編譯必備工具）

```bash
cargo install dioxus-cli --version 0.6.0

```

確認版本為 `0.6.x`（必須與 `Cargo.toml` 中的 Dioxus 0.6 大版本完全對應）：

```bash
dx --version

```

---

## ⚙️ 設定檔配置

在專案**根目錄**（與 `Cargo.toml` 同層）建立 `.env` 檔案：

```env
# Discord 機器人 Token（請至 Discord Developer Portal 申請）
DISCORD_TOKEN=your_discord_bot_token_here

# （選填）後端 API 統計同步網址
API_URL=http://localhost:8000/api

```

> ⚠️ `.env` 包含敏感憑證，專案已將其加入 `.gitignore`，切勿將其推送到公開的 GitHub 儲存庫！

---

## 🚀 啟動與執行命令

專案區分為兩個獨立的 Binary 服務：

### 🤖 方案 A：Discord 機器人端 (Native)

```bash
cargo run --bin bot

```

* **預期行為**：Bot 上線後終端機會顯示 `🤖 [機器人名稱] 準備就緒！`，並自動向 Discord 伺服器註冊全域斜線（Slash）指令。

### 🌐 方案 B：Web 瀏覽器網頁前端 (Dioxus WASM)

```bash
# 語法型別檢查（整合開發環境推薦）
cargo check --bin web --features web

# 啟動網頁本地伺服器
dx serve

```

* **預期行為**：Dioxus CLI 會自動將 Rust 核心編譯為 WebAssembly 並在本地熱重載（Hot Reload）執行。完成後會自動開啟瀏覽器頁面：
👉 `http://localhost:8080`

---

## 🕹️ 核心遊戲功能與底層技術一覽

本專案經過代碼重構，**不論是從 Discord 的按鈕交互互動，還是從網頁上的 Pico.css 美化按鈕點擊**，底層 100% 呼叫 `src/game.rs` 裡的純淨狀態機：

1. **🃏 單人黑傑克 (21點)**

* 支援 Web 端使用大括號區塊 `{}` 包裹的全新 `rsx!` 迭代器渲染動態手牌。
* 包含自動爆牌判定、莊家 AI 補牌至 17 點邏輯。

2. **💣 數字炸彈 (對抗 AI)**

* 採用內建強型別 `BombResult` 進行安全區間限縮判定。
* 網頁端利用可變借用閉包（`mut handle_guess`）實現與 AI 互猜數字的動態互動。

3. **🎰 老虎機拉霸**

* 支援非同步 `spawn(async move)` 動態模擬轉動特效。
* 精準匹配對子、三連線、`7️⃣7️⃣7️⃣` 天降大獎判定。

4. **🂡 德州撲克 (Texas Hold'em)**

* 完整的 PreFlop、Flop、Turn、River、Showdown 五階段德州撲克核心狀態機。
* 內建最優 5 張牌型自動比牌算法（高牌至皇家同花順）。

5. **❌⭕ 圈圈叉叉 (TicTacToe Game)**

* 內建強大的 **Minimax 決策決策算法 AI**，提供網頁端及 Discord 終端文字版棋盤渲染。

6. **📝 Wordle 猜字遊戲**

* 支援 5 字母合法性檢查、字母狀態標記（`Correct` / `Present` / `Absent`）。
* 網頁端內建動態更新顏色狀態的 **QWERTY 虛擬鍵盤**。
