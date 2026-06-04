// src/lib.rs
// 將共享模組暴露為 crate 的公開介面，
// 讓 src/bin/bot.rs 與 src/bin/web.rs 都能透過
// `use discord_game_bot::game::...` 引用，
// 而不必用脆弱的 `#[path = "..."]` 相對路徑。

pub mod game;
pub mod todo;
