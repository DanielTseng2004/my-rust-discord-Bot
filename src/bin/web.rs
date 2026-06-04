#![allow(non_snake_case)]

use dioxus::prelude::*;

// Dioxus 0.6 的 bin 位於 src/bin/，透過 crate root 引用共享模組
use discord_game_bot::game::{ BlackjackCore, BombResult, NumberBomb, SlotMachine };

fn main() {
    dioxus::launch(App);
}

// ✅ Dioxus 0.6：組件不再接收 cx: Scope 參數
#[component]
fn App() -> Element {
    // ✅ use_signal 不再需要傳入 cx
    let mut bj = use_signal(|| BlackjackCore::new());
    let mut bomb = use_signal(|| NumberBomb::new());
    let mut bomb_input = use_signal(|| String::new());
    let mut bomb_msg = use_signal(|| "請輸入 1 ~ 100 之間的數字對抗 AI！".to_string());
    let mut slot_res = use_signal(|| vec!["❓", "❓", "❓"]);
    let mut slot_comment = use_signal(|| "點擊按鈕拉下拉霸！".to_string());

    // ✅ Dioxus 0.6：直接回傳 rsx!，不需要 cx.render()
    rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css" }

        main { class: "container",
            h1 { style: "text-align: center; margin-top: 20px;", "🎮 Rust 遊戲中心 (網頁版)" }
            p  { style: "text-align: center; color: gray;",
                "與 Discord Bot 共享完全一致的底層遊戲運算核心！"
            }

            hr {}

            // ─────────────────────────────────────────
            // 🃏 黑傑克
            // ─────────────────────────────────────────
            section {
                h3 { "🃏 單人黑傑克 (21點)" }
                div { class: "grid",
                    div {
                        h5 { "👤 你的手牌 (總分: {BlackjackCore::get_score(&bj.read().player_cards)})" }
                        p  { { bj.read().player_cards.iter().map(|c| format!("[{}] ", c)).collect::<String>() } }
                    }
                    div {
                        h5 { "🤖 莊家手牌" }
                        if bj.read().is_game_over {
                            p {
                                { bj.read().bot_cards.iter().map(|c| format!("[{}] ", c)).collect::<String>() }
                                " (總分: {BlackjackCore::get_score(&bj.read().bot_cards)})"
                            }
                        } else {
                            p { "[{bj.read().bot_cards[0]}] [?]" }
                        }
                    }
                }

                if bj.read().is_game_over {
                    {
                        let p_score = BlackjackCore::get_score(&bj.read().player_cards);
                        let b_score = BlackjackCore::get_score(&bj.read().bot_cards);
                        let verdict = if p_score > 21      { "💥 你爆牌了！莊家獲勝！" }
                                      else if b_score > 21 { "🎉 莊家爆牌了！你贏了！" }
                                      else if p_score > b_score { "🏆 恭喜你，你贏了！" }
                                      else if p_score < b_score { "❌ 可惜，你輸了..." }
                                      else                  { "🤝 平手！" };
                        rsx! {
                            h4 { style: "color: orange;", { verdict } }
                            button { onclick: move |_| bj.set(BlackjackCore::new()), "🔄 再玩一局" }
                        }
                    }
                } else {
                    div { class: "grid",
                        button { onclick: move |_| bj.write().hit(),   "🟢 要牌 (Hit)" }
                        button { class: "secondary",
                                 onclick: move |_| bj.write().stand(), "🔴 停牌 (Stand)" }
                    }
                }
            }

            hr {}

            // ─────────────────────────────────────────
            // 🎰 老虎機
            // ─────────────────────────────────────────
            section {
                h3 { "🎰 老虎機拉霸" }
                h2 { style: "text-align: center; letter-spacing: 10px;",
                    "[ {slot_res.read()[0]} | {slot_res.read()[1]} | {slot_res.read()[2]} ]"
                }
                p { style: "text-align: center; font-style: italic;", { slot_comment.read().clone() } }
                button {
                    onclick: move |_| {
                        let comments = [
                            "☁️ 殘念未中獎。",
                            "✨ 拿到一對雙胞胎！",
                            "🔥 大勝利！連成一線！",
                            "🎉 👑 JACKPOT!!! 終極大獎！",
                        ];
                        let (reels, level) = SlotMachine::spin();
                        slot_res.set(reels);
                        slot_comment.set(comments[level as usize].to_string());
                    },
                    "🎰 拉下拉霸！"
                }
            }

            hr {}

            // ─────────────────────────────────────────
            // 💣 數字炸彈
            // ─────────────────────────────────────────
            section {
                h3 { "💣 數字炸彈 (對抗 AI)" }
                p  { "目前安全範圍： {bomb.read().min} ~ {bomb.read().max}" }
                blockquote { { bomb_msg.read().clone() } }

                div { class: "grid",
                    input {
                        placeholder: "輸入數字...",
                        value: "{bomb_input}",
                        // ✅ Dioxus 0.6：e.value() 已是 String，不需要 .clone()
                        oninput: move |e| bomb_input.set(e.value()),
                    }
                    button {
                        onclick: move |_| {
                            if let Ok(val) = bomb_input.read().parse::<u32>() {
                                // ── 關鍵修正：Write guard 的生命週期 ──────────────────
                                // bomb.write() 回傳 Write<'_, NumberBomb> guard。
                                // 若直接寫 `match bomb.write().guess(val) { ... }`，
                                // 這個 guard 的生命週期會橫跨整個 match 區塊，
                                // 導致在 arm 內再呼叫 bomb.read() / bomb.write() / bomb.set()
                                // 時觸發 E0499 / E0502 借用衝突。
                                //
                                // 解法：先用區塊 `{ ... }` 把 write + guess 包起來，
                                // 讓 guard 在區塊結束時立刻 drop，
                                // 之後的 read / write / set 就不會有衝突。
                                // ─────────────────────────────────────────────────────

                                // 第一步：玩家猜測，guard 在此區塊結束後立刻 drop
                                let player_result = { bomb.write().guess(val) };

                                match player_result {
                                    BombResult::Invalid => {
                                        // guard 已 drop，可以安全 read
                                        let (lo, hi) = (bomb.read().min, bomb.read().max);
                                        bomb_msg.set(format!("⚠️ 請輸入 {} 到 {} 之間的數字", lo, hi));
                                    }
                                    BombResult::Exploded => {
                                        bomb_msg.set(format!("💥 BOMMMM!! 答案是 {}！你踩到炸彈敗北 💀", val));
                                        bomb.set(NumberBomb::new());
                                    }
                                    BombResult::Safe => {
                                        // 第二步：AI 猜測前先 read（guard 已 drop，安全）
                                        let ai_val = { bomb.read().ai_guess() };

                                        // 第三步：AI 猜測，guard 再次在區塊結束後立刻 drop
                                        let ai_result = { bomb.write().guess(ai_val) };

                                        match ai_result {
                                            BombResult::Exploded => {
                                                bomb_msg.set(format!("🎉 機器人猜了 {} 踩到炸彈！你贏了！🏆", ai_val));
                                                bomb.set(NumberBomb::new());
                                            }
                                            BombResult::Safe => {
                                                bomb_msg.set(format!("✅ 你安全。🤖 機器人猜了 {} 也安全！換你...", ai_val));
                                            }
                                            BombResult::Invalid => {
                                                bomb_msg.set("🤖 AI 計算出現異常，請繼續輸入。".to_string());
                                            }
                                        }
                                    }
                                }
                            }
                            bomb_input.set(String::new());
                        },
                        "確定"
                    }
                }
            }
        }
    }
}
