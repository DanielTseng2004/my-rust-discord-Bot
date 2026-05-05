# 🦀 Rust Discord Todo & Game Bot

這是一個使用 Rust 語言編寫的 Discord 機器人，整合了 **斜線指令 (Slash Commands)**、**待辦清單管理 (Todo List)** 以及一個簡單的 **互動式猜拳遊戲**。

本專案旨在展示 Rust 在非同步環境（Async）下的狀態管理與 Discord API 的整合應用。

## ✨ 功能亮點

- **斜線指令**：採用 Discord 最新的 `/` 指令互動模式，提供參數提示與預選單。
- **記憶體狀態管理**：使用 `Arc<RwLock<T>>` 實現執行緒安全的資料共享。
- **模組化邏輯**：遊戲邏輯與 Bot 指令處理分離，易於擴充。
- **安全性**：整合 `dotenvy` 管理環境變數，確保 Token 不外流。

## 🛠️ 開發工具

- [Serenity](https://github.com/serenity-rs/serenity)：強大的 Discord API 封裝庫。
- [Tokio](https://tokio.rs/)：高效能的非同步 Runtime。
- [Dotenvy](https://github.com/allan2/dotenvy)：環境變數管理。

## 🚀 快速開始

### 1. 取得 Token

請至 [Discord Developer Portal](https://discord.com/developers/applications) 建立應用程式，並取得 Bot Token。請確保開啟 **Message Content Intent**。

### 2. 環境設定

在專案根目錄建立 `.env` 檔案，並填入你的 Token：

```text
DISCORD_TOKEN=你的_DISCORD_TOKEN_在這裡
```

### 3. 編譯與執行

確保你已安裝 [Rust/Cargo](https://rustup.rs/)：

```bash
# 檢查代碼
cargo check

# 啟動機器人
cargo run
```

## 🎮 指令說明

| 指令          | 說明                                   |
| :------------ | :------------------------------------- |
| `/add`        | 新增一個待辦事項 (需輸入 content 參數) |
| `/list`       | 列出目前存儲在記憶體中的所有任務       |
| `/game`       | 啟動猜拳比大小遊戲 (含互動按鈕)        |
| `/guess`      | 猜一個數字，踩到炸彈就輸了！           |
| `/reset_bomb` | 重置炸彈範圍                           |

## 📂 專案架構

```text
.
├── src/
│   ├── main.rs          # Bot 核心邏輯與指令註冊
│   └── game.rs          # 遊戲邏輯 (Service 層)
├── .env                 # 環境變數 (已由 .gitignore 忽略)
├── .gitignore           # 排除 target/ 與敏感資訊
└── Cargo.toml           # 專案依賴設定
```

## 📝 學習筆記

這個專案實踐了 Rust 的多項核心特性：

- **Ownership & Borrowing**：在多執行緒環境下安全地讀寫 Todo 清單。

- **Concurrency**：利用 `tokio` 處理高併發的 Discord 事件。
- **Error Handling**：透過 Rust 嚴謹的類型系統處理 API 調用。

---
Generated with ❤️ by Gemini Rust Assistant
