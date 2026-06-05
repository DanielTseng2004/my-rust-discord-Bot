#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::collections::HashMap;
use discord_game_bot::game::{
    BlackjackCore,
    BombResult,
    Card,
    LetterHint,
    NumberBomb,
    SlotMachine,
    TexasHoldem,
    TexasStage,
    TicTacToe,
    WordleGame,
};

fn main() {
    dioxus::launch(App);
}

// 🌟 核心組件：渲染卡牌，轉移所有權，徹底與生命週期切割
#[component]
fn PlayingCard(card: Card) -> Element {
    rsx! {
        span { 
            class: "playing-card", 
            style: "color: {card.suit_color()};", 
            "{card.display()}" 
        }
    }
}

#[component]
fn App() -> Element {
    let mut active_tab = use_signal(|| "blackjack");

    rsx! {
        link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css" }
        style {
            "
            .card-container {{ display: flex; gap: 8px; flex-wrap: wrap; margin: 8px 0; }}
            .playing-card {{ 
                display: inline-block; width: 3.5em; text-align: center; 
                padding: 12px 4px; border: 2px solid #ccc; border-radius: 8px; 
                font-size: 1.2em; font-weight: bold; background: white;
            }}
            .slot-reel {{ font-size: 3em; letter-spacing: 10px; text-align: center; }}
            .spinning {{ animation: blur 0.1s infinite; }}
            @keyframes blur {{
                0% {{ filter: blur(0px); }}
                50% {{ filter: blur(2px); }}
                100% {{ filter: blur(0px); }}
            }}
            nav li button {{ padding: 4px 12px; }}
            .wordle-key {{
                display: inline-block; width: 2.2em; height: 3em; line-height: 3em;
                text-align: center; margin: 2px; border-radius: 4px; font-weight: bold;
                cursor: pointer; user-select: none; border: none;
            }}
            "
        }

        main { class: "container",
            h1 { style: "text-align: center; margin-top: 20px;", "🎮 Rust 遊戲中心" }
            
            nav {
                ul {
                    li { strong { "遊戲選單：" } }
                }
                ul {
                    li { button { 
                        class: if active_tab() == "blackjack" { "" } else { "outline" },
                        onclick: move |_| active_tab.set("blackjack"), 
                        "🃏 Blackjack" 
                    } }
                    li { button { 
                        class: if active_tab() == "slot" { "" } else { "outline" },
                        onclick: move |_| active_tab.set("slot"), 
                        "🎰 Slot" 
                    } }
                    li { button { 
                        class: if active_tab() == "bomb" { "" } else { "outline" },
                        onclick: move |_| active_tab.set("bomb"), 
                        "💣 Bomb" 
                    } }
                    li { button { 
                        class: if active_tab() == "texas" { "" } else { "outline" },
                        onclick: move |_| active_tab.set("texas"), 
                        "🂡 Texas" 
                    } }
                    li { button { 
                        class: if active_tab() == "ttt" { "" } else { "outline" },
                        onclick: move |_| active_tab.set("ttt"), 
                        "❌⭕ TTT" 
                    } }
                    li { button { 
                        class: if active_tab() == "wordle" { "" } else { "outline" },
                        onclick: move |_| active_tab.set("wordle"), 
                        "📝 Wordle" 
                    } }
                }
            }

            hr {}

            match active_tab() {
                "blackjack" => rsx! { BlackjackGame {} },
                "slot" => rsx! { SlotGame {} },
                "bomb" => rsx! { BombGame {} },
                "texas" => rsx! { TexasGame {} },
                "ttt" => rsx! { TicTacToeGame {} },
                "wordle" => rsx! { WordleGameComponent {} },
                _ => rsx! { div { "選擇一個遊戲開始吧！" } }
            }
        }
    }
}

// ── 🃏 Blackjack 組件 ────────────────────────────────────────────────
#[component]
fn BlackjackGame() -> Element {
    let mut bj = use_signal(|| BlackjackCore::new());

    rsx! {
        section {
            h3 { "🃏 單人黑傑克 (21點)" }
            div { class: "grid",
                div {
                    h5 { "👤 你的手牌 (總分: {BlackjackCore::get_score(&bj.read().player_cards)})" }
                    div { class: "card-container",
                        // 🌟 修正關鍵 1：用大括號把迭代器包起來，並加上 move 避免生命週期紅線
                        {
                            bj.read().player_cards.iter().map(move |c| rsx! {
                                PlayingCard { card: *c }
                            })
                        }
                    }
                }
                div {
                    h5 { "🤖 莊家手牌" }
                    div { class: "card-container",
                        if bj.read().is_game_over {
                            {
                                bj.read().bot_cards.iter().map(move |c| rsx! {
                                    PlayingCard { card: *c }
                                })
                            }
                        } else {{
                            rsx! {
                                PlayingCard { card: bj.read().bot_cards[0] }
                                span { class: "playing-card", style: "color: gray; background: #eee;", "?" }
                            }}
                        }
                    }
                    if bj.read().is_game_over {
                        p { "(總分: {BlackjackCore::get_score(&bj.read().bot_cards)})" }
                    }
                }
            }
{
            if bj.read().is_game_over {
                {
                    let p_score = BlackjackCore::get_score(&bj.read().player_cards);
                    let b_score = BlackjackCore::get_score(&bj.read().bot_cards);
                    let verdict = if p_score > 21 { "💥 你爆牌了！莊家獲勝！" }
                                else if b_score > 21 { "🎉 莊家爆牌了！你贏了！" }
                                else if p_score > b_score { "🏆 恭喜你，你贏了！" }
                                else if p_score < b_score { "❌ 可惜，你輸了..." }
                                else { "🤝 平手！" };
                    rsx! {
                        h4 { style: "color: orange;", "{verdict}" }
                        button { onclick: move |_| bj.set(BlackjackCore::new()), "🔄 再玩一局" }
                    }
                }
            } 
else {
                rsx! {
                    div { class: "grid",
                        button { onclick: move |_| bj.write().hit(), "🟢 要牌 (Hit)" }
                        button { class: "secondary", onclick: move |_| bj.write().stand(), "🔴 停牌 (Stand)" }
                    }
                }
            }
        }}
    }
}

// ── 🎰 老虎機組件 ──────────────────────────────────────────────────
#[component]
fn SlotGame() -> Element {
    // 🌟 修正關鍵 2：將初始值與回傳值統一定義為 String，解決 E0277 型別不匹配
    let mut slot_res = use_signal(|| vec!["❓".to_string(), "❓".to_string(), "❓".to_string()]);
    let mut slot_comment = use_signal(|| "點擊按鈕拉下拉霸！".to_string());
    let mut is_spinning = use_signal(|| false);

    let spin = move |_| {
        if is_spinning() {
            return;
        }
        is_spinning.set(true);
        slot_comment.set("🎰 轉動中...".to_string());

        spawn(async move {
            let (reels, level) = SlotMachine::spin();
            let comments = [
                "☁️ 殘念未中獎。",
                "✨ 拿到一對雙胞胎！",
                "🔥 大勝利！連成一線！",
                "🎉 👑 JACKPOT!!!",
            ];
            // 轉換為 Vec<String> 存入 signal
            slot_res.set(
                reels
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            );
            slot_comment.set(comments[level as usize].to_string());
            is_spinning.set(false);
        });
    };

    // 🌟 修正關鍵 3：簡化動態類別字串拼接，移除破壞巨集解析的引號轉義
    let reel_class = if is_spinning() { "slot-reel spinning" } else { "slot-reel" };

    rsx! {
        section {
            h3 { "🎰 老虎機拉霸" }
            div { 
                class: "{reel_class}",
                "[ {slot_res.read()[0]} | {slot_res.read()[1]} | {slot_res.read()[2]} ]"
            }
            p { style: "text-align: center; font-style: italic; margin-top: 10px;", "{slot_comment}" }
            button { 
                disabled: is_spinning(),
                onclick: spin, 
                "🎰 拉下拉霸！" 
            }
        }
    }
}

// ── 💣 數字炸彈組件 ────────────────────────────────────────────────
#[component]
fn BombGame() -> Element {
    let mut bomb = use_signal(|| NumberBomb::new());
    let mut bomb_input = use_signal(|| String::new());
    let mut bomb_msg = use_signal(|| "請輸入 1 ~ 100 之間的數字對抗 AI！".to_string());

    // 🌟 修正關鍵 4：加上 mut 關鍵字，解決 E0596 無法可變借用閉包錯誤
    let mut handle_guess = move || {
        if let Ok(val) = bomb_input.read().parse::<u32>() {
            let player_result = { bomb.write().guess(val) };
            match player_result {
                BombResult::Invalid => {
                    let (lo, hi) = (bomb.read().min, bomb.read().max);
                    bomb_msg.set(format!("⚠️ 請輸入 {} 到 {} 之間的數字", lo, hi));
                }
                BombResult::Exploded => {
                    bomb_msg.set(format!("💥 BOMMMM!! 答案是 {}！你踩到炸彈敗北 💀", val));
                    bomb.set(NumberBomb::new());
                }
                BombResult::Safe => {
                    let ai_val = bomb.read().ai_guess();
                    let ai_result = { bomb.write().guess(ai_val) };
                    match ai_result {
                        BombResult::Exploded => {
                            bomb_msg.set(format!("🎉 機器人猜了 {} 踩到炸彈！你贏了！🏆", ai_val));
                            bomb.set(NumberBomb::new());
                        }
                        BombResult::Safe => {
                            bomb_msg.set(
                                format!("✅ 你安全。🤖 機器人猜了 {} 也安全！換你...", ai_val)
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
        bomb_input.set(String::new());
    };

    rsx! {
        section {
            h3 { "💣 數字炸彈 (對抗 AI)" }
            p { "目前安全範圍： {bomb.read().min} ~ {bomb.read().max}" }
            blockquote { "{bomb_msg}" }
            div { class: "grid",
                input {
                    placeholder: "輸入數字...",
                    value: "{bomb_input}",
                    oninput: move |e| bomb_input.set(e.value()),
                    onkeydown: move |e| if e.key() == Key::Enter { handle_guess(); }
                }
                button { onclick: move |_| handle_guess(), "確定" }
            }
        }
    }
}

// ── 🂡 德州撲克組件 ────────────────────────────────────────────────
#[component]
fn TexasGame() -> Element {
    let mut texas = use_signal(|| TexasHoldem::new());

    rsx! {
        section {
            h3 { "🂡 德州撲克（Texas Hold'em）" }
            div { style: "margin: 0.8rem 0;",
                h5 { "👤 你的底牌" }
                div { class: "card-container",
                    {
                        texas.read().player_hole.iter().map(move |c| rsx! {
                            PlayingCard { card: *c }
                        })
                    }
                }
            }
            div { style: "margin: 0.8rem 0;",
                h5 { "🃏 公共牌" }
                div { class: "card-container",
                    {
                        (0..5usize).map(move |i| {
                            let visible = texas.read().visible_community().len();
                            if i < visible {
                                rsx! { PlayingCard { card: texas.read().visible_community()[i] } }
                            } else {
                                rsx! { span { class: "playing-card", style: "color:#aaa; border-style:dashed;", "?" } }
                            }
                        })
                    }
                }
            }

            {
                let stage = texas.read().stage.clone();
                match stage {
                    TexasStage::PreFlop => rsx! { button { onclick: move |_| texas.write().next_stage(), "翻牌 Flop (3張)" } },
                    TexasStage::Flop => rsx! { button { class: "secondary", onclick: move |_| texas.write().next_stage(), "轉牌 Turn (1張)" } },
                    TexasStage::Turn => rsx! { button { class: "secondary", onclick: move |_| texas.write().next_stage(), "河牌 River (1張)" } },
                    TexasStage::River => rsx! { button { class: "contrast", onclick: move |_| texas.write().next_stage(), "🏆 開牌比較" } },
                    TexasStage::Showdown => {
                        let res = texas.read();
                        let r = res.result.as_ref().unwrap();
                        rsx! {
                            div { class: "grid",
                                div {
                                    h5 { "👤 你的最佳 5 張" }
                                    div { class: "card-container", 
                                        {
                                            r.player_best.iter().map(move |c| rsx! {
                                                PlayingCard { card: *c }
                                            })
                                        }
                                    }
                                    p { "牌型：", b { "{r.player_rank.name()}" } }
                                }
                                div {
                                    h5 { "🤖 AI 底牌：{res.ai_hole[0].display()} {res.ai_hole[1].display()}" }
                                    div { class: "card-container", 
                                        {
                                            r.ai_best.iter().map(move |c| rsx! {
                                                PlayingCard { card: *c }
                                            })
                                        }
                                    }
                                    p { "牌型：", b { "{r.ai_rank.name()}" } }
                                }
                            }
                            h4 { style: "color: orange;", "{r.verdict}" }
                            button { onclick: move |_| texas.set(TexasHoldem::new()), "🔄 新局" }
                        }
                    }
                }
            }
        }
    }
}

// ── ❌⭕ 圈圈叉叉組件 ──────────────────────────────────────────────
#[component]
fn TicTacToeGame() -> Element {
    let mut ttt = use_signal(|| TicTacToe::new());

    rsx! {
        section {
            h3 { "❌⭕ 圈圈叉叉 (對抗 AI)" }
            div { style: "display: grid; grid-template-columns: repeat(3, 80px); gap: 6px; margin: 1rem 0;",
                {
                    (0..9usize).map(move |i| {
                        let cell = ttt.read().board[i];
                        let symbol = match cell { 1 => "X", 2 => "O", _ => "" };
                        let bg = match cell { 1 => "background:#e74c3c; color:#fff;", 2 => "background:#3498db; color:#fff;", _ => "" };
                        rsx! {
                            button {
                                key: "{i}",
                                style: "height:80px; font-size:2em; {bg}",
                                disabled: cell != 0 || ttt.read().game_over,
                                onclick: move |_| { ttt.write().player_move(i); },
                                "{symbol}"
                            }
                        }
                    })
                }
            }{
                if ttt.read().game_over {
                    let msg = match ttt.read().winner { 
                        1 => "🏆 你贏了！", 
                        2 => "🤖 AI 獲勝！", 
                        3 => "🤝 平手！", 
                        _ => "" 
                    };
                    // 遊戲結束時，正常渲染結算畫面
                    Some(rsx! {
                        h4 { style: "color: orange;", "{msg}" }
                        button { onclick: move |_| ttt.set(TicTacToe::new()), "🔄 重新開局" }
                    })
                } else {
                    // 遊戲尚未結束時，回傳 None（意即什麼都不渲染）
                    None
                }
            }
        }
    }
}

// ── 📝 Wordle 組件 ──────────────────────────────────────────────────
#[component]
fn WordleGameComponent() -> Element {
    let mut wordle = use_signal(|| WordleGame::new());
    let mut wordle_input = use_signal(|| String::new());
    let mut wordle_msg = use_signal(|| "猜一個 5 字母英文單字".to_string());

    let key_states = use_memo(move || {
        let mut states = HashMap::new();
        let w = wordle.read();
        for (guess, hints) in w.guesses.iter().zip(w.hints.iter()) {
            for (ch, hint) in guess.chars().zip(hints.iter()) {
                let current = states.entry(ch).or_insert(LetterHint::Absent);
                if *hint == LetterHint::Correct {
                    *current = LetterHint::Correct;
                } else if *hint == LetterHint::Present && *current != LetterHint::Correct {
                    *current = LetterHint::Present;
                }
            }
        }
        states
    });

    let mut submit_guess = move || {
        let word = wordle_input.read().clone();
        if word.len() != 5 {
            return;
        }
        let result = { wordle.write().guess(&word) };
        match result {
            None => wordle_msg.set(format!("⚠️ `{}` 不是合法的5字母單字", word)),
            Some(_) => {
                let w = wordle.read();
                if w.solved {
                    wordle_msg.set(format!("🎉 答對了！答案是 {}", w.answer));
                } else if w.is_over() {
                    wordle_msg.set(format!("💀 用完次數！答案是 {}", w.answer));
                } else {
                    wordle_msg.set(
                        format!("還剩 {} 次機會！", (w.max_guesses as usize) - w.guesses.len())
                    );
                }
            }
        }
        wordle_input.set(String::new());
    };

    let qwerty = ["QWERTYUIOP", "ASDFGHJKL", "ZXCVBNM"];

    let keyboard_rows = qwerty.iter().map(move |row| {
        rsx! {
            div { key: "{row}",
                { row.chars().map(move |ch| {
                    let state = key_states.read().get(&ch).cloned().unwrap_or(LetterHint::Absent);
                    let has_guessed = key_states.read().contains_key(&ch);
                    let bg = if !has_guessed { "#d3d6da" } else {
                        match state { LetterHint::Correct => "#6aaa64", LetterHint::Present => "#c9b458", LetterHint::Absent => "#787c7e" }
                    };
                    let color = if !has_guessed { "#000" } else { "#fff" };
                    rsx! { 
                        button { 
                            class: "wordle-key", 
                            style: "background: {bg}; color: {color};",
                            onclick: move |_| {
                                if wordle_input.read().len() < 5 {
                                    wordle_input.write().push(ch);
                                }
                            },
                            "{ch}"
                        } 
                    }
                })}
            }
        }
    });

    rsx! {
        section {
            h3 { "📝 Wordle 猜字遊戲" }
            
            div { style: "font-family: monospace; margin-bottom: 20px;",
                {
                    wordle.read().guesses.iter().zip(wordle.read().hints.iter()).enumerate().map(move |(i, (g, h))| {
                        rsx! {
                            div { key: "{i}", style: "margin-bottom: 4px;",
                                { g.chars().zip(h.iter()).map(move |(ch, hint)| {
                                    let color = match hint { LetterHint::Correct => "#6aaa64", LetterHint::Present => "#c9b458", LetterHint::Absent => "#787c7e" };
                                    rsx! { span { style: "display:inline-block; width:2em; text-align:center; background:{color}; color:#fff; margin:2px; padding:4px; border-radius:4px;", "{ch}" } }
                                })}
                            }
                        }
                    })
                }
            }

            blockquote { "{wordle_msg}" }

            if !wordle.read().is_over() {
                div { class: "grid",
                    input {
                        placeholder: "輸入5字母...", maxlength: "5", value: "{wordle_input}",
                        oninput: move |e| wordle_input.set(e.value().to_uppercase()),
                        onkeydown: move |e| if e.key() == Key::Enter { submit_guess(); }
                    }
                    button { onclick: move |_| submit_guess(), "確定" }
                }
                
                div { style: "text-align: center; margin-top: 10px;",
                    { keyboard_rows }
                }
            } else {
                button { onclick: move |_| { wordle.set(WordleGame::new()); wordle_msg.set("猜一個 5 字母英文單字".to_string()); }, "🔄 新的一局" }
            }
        }
    }
}
