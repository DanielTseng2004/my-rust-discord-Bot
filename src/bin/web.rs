#![allow(non_snake_case)]

use dioxus::prelude::*;

// Dioxus 0.6 的 bin 位於 src/bin/，透過 crate root 引用共享模組
use discord_game_bot::game::{
    BlackjackCore,
    BombResult,
    NumberBomb,
    SlotMachine,
    TexasHoldem,
    TexasStage,
    TicTacToe,
    WordleGame,
    LetterHint,
};

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

    // ── 德州撲克（Texas Hold'em）──────────────────────────────────────
    let mut texas = use_signal(|| TexasHoldem::new());

    // ── 圈圈叉叉 ──────────────────────────────────────────────────────
    let mut ttt = use_signal(|| TicTacToe::new());

    // ── Wordle ─────────────────────────────────────────────────────────
    let mut wordle = use_signal(|| WordleGame::new());
    let mut wordle_input = use_signal(|| String::new());
    let mut wordle_msg = use_signal(|| "猜一個 5 字母英文單字（按 Enter 或點確定）".to_string());

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
            hr {}

            // ─────────────────────────────────────────
            // 🂡 德州撲克（Texas Hold'em）
            // ─────────────────────────────────────────
            section {
                h3 { "🂡 德州撲克（Texas Hold'em）" }
                p { style: "color: gray; font-size: 0.9em;",
                    "荷官發 2 張底牌，依序翻 Flop/Turn/River，從 7 張選最佳 5 張比大小（無下注）。"
                }

                // 玩家底牌（永遠顯示）
                div { style: "margin: 0.8rem 0;",
                    h5 { style: "margin-bottom: 4px;", "👤 你的底牌" }
                    div { style: "display: flex; gap: 8px;",
                        { texas.read().player_hole.iter().map(|c| {
                            let label = c.display();
                            let suit_color = if c.suit == 1 || c.suit == 2 { "#e74c3c" } else { "#2c3e50" };
                            rsx! {
                                span {
                                    style: "display:inline-block; width:3em; text-align:center; padding:8px 4px; border:2px solid #ccc; border-radius:8px; font-size:1.3em; font-weight:bold; color:{suit_color};",
                                    { label }
                                }
                            }
                        })}
                    }
                }

                // 公共牌區
                div { style: "margin: 0.8rem 0;",
                    h5 { style: "margin-bottom: 4px;", "🃏 公共牌" }
                    div { style: "display: flex; gap: 8px; flex-wrap: wrap;",
                        { (0..5usize).map(|i| {
                            let visible = texas.read().visible_community().len();
                            if i < visible {
                                let c = texas.read().visible_community()[i];
                                let label = c.display();
                                let suit_color = if c.suit == 1 || c.suit == 2 { "#e74c3c" } else { "#2c3e50" };
                                rsx! {
                                    span {
                                        key: "{i}",
                                        style: "display:inline-block; width:3em; text-align:center; padding:8px 4px; border:2px solid #ccc; border-radius:8px; font-size:1.3em; font-weight:bold; color:{suit_color};",
                                        { label }
                                    }
                                }
                            } else {
                                rsx! {
                                    span {
                                        key: "{i}",
                                        style: "display:inline-block; width:3em; text-align:center; padding:8px 4px; border:2px dashed #ccc; border-radius:8px; font-size:1.3em; color:#aaa;",
                                        "?"
                                    }
                                }
                            }
                        })}
                    }
                }

                // 操作按鈕 + 狀態
                {
                    let stage = texas.read().stage.clone();
                    match stage {
                        TexasStage::PreFlop => rsx! {
                            div { class: "grid",
                                button { onclick: move |_| { texas.write().next_stage(); }, "翻牌 Flop（3張）" }
                            }
                        },
                        TexasStage::Flop => rsx! {
                            div { class: "grid",
                                button { class: "secondary", onclick: move |_| { texas.write().next_stage(); }, "轉牌 Turn（1張）" }
                            }
                        },
                        TexasStage::Turn => rsx! {
                            div { class: "grid",
                                button { class: "secondary", onclick: move |_| { texas.write().next_stage(); }, "河牌 River（1張）" }
                            }
                        },
                        TexasStage::River => rsx! {
                            div { class: "grid",
                                button { class: "contrast", onclick: move |_| { texas.write().next_stage(); }, "🏆 開牌比較 Showdown" }
                            }
                        },
                        TexasStage::Showdown => {
                            let res = texas.read();
                            let r = res.result.as_ref().unwrap();
                            let pb = r.player_best.iter().map(|c| c.display()).collect::<Vec<_>>().join("  ");
                            let ab = r.ai_best.iter().map(|c| c.display()).collect::<Vec<_>>().join("  ");
                            let ai_hole = res.ai_hole.iter().map(|c| c.display()).collect::<Vec<_>>().join("  ");
                            let p_rank = r.player_rank.name().to_string();
                            let a_rank = r.ai_rank.name().to_string();
                            let verdict = r.verdict.to_string();
                            rsx! {
                                div { class: "grid", style: "margin-top: 0.8rem;",
                                    div {
                                        h5 { "👤 你的最佳5張" }
                                        p { style: "font-size: 1.1em; letter-spacing: 3px;", { pb } }
                                        p { "牌型：", b { { p_rank } } }
                                    }
                                    div {
                                        h5 { "🤖 AI 底牌：", { ai_hole }, "　最佳5張" }
                                        p { style: "font-size: 1.1em; letter-spacing: 3px;", { ab } }
                                        p { "牌型：", b { { a_rank } } }
                                    }
                                }
                                h4 { style: "color: orange;", { verdict } }
                                button { onclick: move |_| texas.set(TexasHoldem::new()), "🔄 新局" }
                            }
                        }
                    }
                }
            }

            hr {}

            // ─────────────────────────────────────────
            // ❌⭕ 圈圈叉叉
            // ─────────────────────────────────────────
            section {
                h3 { "❌⭕ 圈圈叉叉 (對抗 AI)" }
                p { style: "color: gray; font-size: 0.9em;",
                    "你是 X，AI 是 O。AI 使用 Minimax 演算法，盡力不讓你贏！"
                }
                // 棋盤 3x3 按鈕
                div { style: "display: grid; grid-template-columns: repeat(3, 80px); gap: 6px; margin: 1rem 0;",
                    { (0..9usize).map(|i| {
                        let cell = ttt.read().board[i];
                        let symbol = match cell { 1 => "X", 2 => "O", _ => "" };
                        let disabled = cell != 0 || ttt.read().game_over;
                        let bg = match cell {
                            1 => "background:#e74c3c; color:#fff;",
                            2 => "background:#3498db; color:#fff;",
                            _ => "",
                        };
                        rsx! {
                            button {
                                key: "{i}",
                                style: "height:80px; font-size:2em; {bg}",
                                disabled: disabled,
                                onclick: move |_| { ttt.write().player_move(i); },
                                { symbol }
                            }
                        }
                    })}
                }
                // 狀態文字
                {
                    let w = ttt.read().winner;
                    let over = ttt.read().game_over;
                    if over {
                        let msg = match w {
                            1 => "🏆 你贏了！",
                            2 => "🤖 AI 獲勝！",
                            3 => "🤝 平手！",
                            _ => "",
                        };
                        rsx! {
                            h4 { style: "color: orange;", { msg } }
                            button { onclick: move |_| ttt.set(TicTacToe::new()), "🔄 重新開局" }
                        }
                    } else {
                        rsx! { p { "點擊格子落子，AI 會立刻應對！" } }
                    }
                }
            }

            hr {}

            // ─────────────────────────────────────────
            // 📝 Wordle
            // ─────────────────────────────────────────
            section {
                h3 { "📝 Wordle 猜字遊戲" }
                p { style: "color: gray; font-size: 0.9em;",
                    "猜 5 字母英文單字，共 6 次機會。🟩=位置正確 🟨=有但位置錯 ⬛=不存在"
                }

                // 猜測歷史
                div { style: "font-family: monospace; font-size: 1.1em; margin: 0.5rem 0;",
                    { wordle.read().guesses.iter().zip(wordle.read().hints.iter()).enumerate().map(|(i, (g, h))| {
                        // 先把字母和對應顏色收成普通 Vec，避免在 rsx! 外使用 rsx::Element 型別
                        let pairs: Vec<(char, &'static str)> = g.chars().zip(h.iter()).map(|(ch, hint)| {
                            let color = match hint {
                                LetterHint::Correct => "#6aaa64",
                                LetterHint::Present => "#c9b458",
                                LetterHint::Absent  => "#787c7e",
                            };
                            (ch, color)
                        }).collect();
                        rsx! {
                            div { key: "{i}", style: "margin-bottom: 4px;",
                                span { style: "margin-right:6px; color:gray;", { format!("#{}", i+1) } }
                                { pairs.into_iter().map(|(ch, color)| rsx! {
                                    span {
                                        style: "display:inline-block; width:2em; text-align:center; background:{color}; color:#fff; margin:2px; padding:4px; border-radius:4px;",
                                        { ch.to_string() }
                                    }
                                })}
                            }
                        }
                    })}
                }

                blockquote { { wordle_msg.read().clone() } }

                if !wordle.read().is_over() {
                    div { class: "grid",
                        input {
                            placeholder: "輸入5字母...",
                            maxlength: "5",
                            value: "{wordle_input}",
                            oninput: move |e| wordle_input.set(e.value().to_uppercase()),
                            onkeydown: move |e| {
                                if e.key() == Key::Enter {
                                    let word = wordle_input.read().clone();
                                    // write guard 在此區塊結束後立刻 drop，
                                    // 之後的 read 才不會發生借用衝突
                                    let result = { wordle.write().guess(&word) };
                                    match result {
                                        None => wordle_msg.set(format!("⚠️ `{}` 不是合法的5字母單字", word)),
                                        Some(_) => {
                                            let (solved, over, answer, remaining) = {
                                                let w = wordle.read();
                                                (w.solved, w.is_over(), w.answer, w.max_guesses as usize - w.guesses.len())
                                            };
                                            if solved {
                                                wordle_msg.set(format!("🎉 答對了！答案是 {}", answer));
                                            } else if over {
                                                wordle_msg.set(format!("💀 用完次數！答案是 {}", answer));
                                            } else {
                                                wordle_msg.set(format!("還剩 {} 次機會！", remaining));
                                            }
                                        }
                                    }
                                    wordle_input.set(String::new());
                                }
                            },
                        }
                        button {
                            onclick: move |_| {
                                let word = wordle_input.read().clone();
                                // 同樣用區塊讓 write guard 提前 drop
                                let result = { wordle.write().guess(&word) };
                                match result {
                                    None => wordle_msg.set(format!("⚠️ `{}` 不是合法的5字母單字", word)),
                                    Some(_) => {
                                        let (solved, over, answer, remaining) = {
                                            let w = wordle.read();
                                            (w.solved, w.is_over(), w.answer, w.max_guesses as usize - w.guesses.len())
                                        };
                                        if solved {
                                            wordle_msg.set(format!("🎉 答對了！答案是 {}", answer));
                                        } else if over {
                                            wordle_msg.set(format!("💀 用完次數！答案是 {}", answer));
                                        } else {
                                            wordle_msg.set(format!("還剩 {} 次機會！", remaining));
                                        }
                                    }
                                }
                                wordle_input.set(String::new());
                            },
                            "確定"
                        }
                    }
                }

                if wordle.read().is_over() {
                    button {
                        onclick: move |_| {
                            wordle.set(WordleGame::new());
                            wordle_msg.set("猜一個 5 字母英文單字（按 Enter 或點確定）".to_string());
                        },
                        "🔄 新的一局"
                    }
                }
            }
        }
    }
}
