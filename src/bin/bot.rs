// 宣告同層目錄的模組
#![allow(dead_code)]
#[path = "../game.rs"]
mod game;
#[path = "../todo.rs"]
mod todo;

use serenity::async_trait;
use serenity::builder::{
    CreateActionRow, CreateButton, CreateCommand, CreateCommandOption, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse,
};
use serenity::model::application::{ButtonStyle, Command, CommandOptionType, Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::sync::Arc;

use game::{
    BlackjackCore, BombResult, GameResult, GuessGame, NumberBomb, Roulette, SlotMachine,
    TexasHoldem, TexasStage, TicTacToe, WordleGame,
};

struct TodoState;
impl TypeMapKey for TodoState {
    type Value = Arc<RwLock<todo::TodoList>>;
}

struct BombState;
impl TypeMapKey for BombState {
    type Value = Arc<RwLock<NumberBomb>>;
}

struct RouletteState;
impl TypeMapKey for RouletteState {
    type Value = Arc<RwLock<Roulette>>;
}

struct BlackjackState;
impl TypeMapKey for BlackjackState {
    type Value = Arc<RwLock<BlackjackCore>>;
}

struct TttState;
impl TypeMapKey for TttState {
    type Value = Arc<RwLock<TicTacToe>>;
}

struct TexasState;
impl TypeMapKey for TexasState {
    type Value = Arc<RwLock<TexasHoldem>>;
}

struct WordleState;
impl TypeMapKey for WordleState {
    type Value = Arc<RwLock<WordleGame>>;
}

struct Handler;

async fn sync_game_stat(game_name: &str) {
    let api_url =
        std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:8000/api".to_string());
    let url = format!("{}/games/{}/stat", api_url, game_name);
    if let Ok(client) = reqwest::Client::new().post(&url).send().await {
        if client.status().is_success() {
            println!("✅ 已同步遊戲統計: {}", game_name);
        }
    }
}

// ── 優化點：黑傑克渲染改為精美 Embed 樣式 ───────────────────────────────
fn render_bj_embed(bj: &BlackjackCore) -> CreateEmbed {
    let p_score = BlackjackCore::get_score(&bj.player_cards);
    let b_score = BlackjackCore::get_score(&bj.bot_cards);

    let p_str = bj
        .player_cards
        .iter()
        .map(|c| format!("`{}` ", c.display()))
        .collect::<String>();

    if !bj.is_game_over {
        CreateEmbed::new()
            .title("🃏 德州黑傑克 (Blackjack)")
            .color(0x3498db) // 經典藍色
            .description("對決正在進行中！看準點數，考驗你的膽量與策略。")
            .field(
                "👤 你的手牌",
                format!("{} ➔ 總分: **`{}`**", p_str, p_score),
                false,
            )
            .field(
                "🤖 莊家手牌",
                format!("`{}` `[?]`", bj.bot_cards[0].display()),
                false,
            )
            .footer(serenity::builder::CreateEmbedFooter::new(
                "💬 請點擊下方按鈕選擇【🟢 要牌】或【🔴 停牌】！",
            ))
    } else {
        let b_str = bj
            .bot_cards
            .iter()
            .map(|c| format!("`{}` ", c.display()))
            .collect::<String>();

        let (res_title, res_color) = if p_score > 21 {
            ("🚨 玩家爆牌，莊家獲勝！ 💀", 0xe74c3c) // 紅色
        } else if b_score > 21 {
            ("👑 莊家爆牌，玩家獲勝！ 🏆", 0x2ecc71) // 綠色
        } else if p_score > b_score {
            ("👑 恭喜你戰勝了莊家！ 🎉", 0x2ecc71) // 綠色
        } else if p_score < b_score {
            ("❌ 很遺憾，你輸給了莊家... 💀", 0xe74c3c) // 紅色
        } else {
            ("🤝 雙方平手，平分秋色！ ⚖️", 0x95a5a6) // 灰色
        };

        CreateEmbed::new()
            .title(res_title)
            .color(res_color)
            .field(
                "👤 你的最終手牌",
                format!("{} ➔ 總分: **`{}`**", p_str, p_score),
                true,
            )
            .field(
                "🤖 莊家的最終手牌",
                format!("{} ➔ 總分: **`{}`**", b_str, b_score),
                true,
            )
            .footer(serenity::builder::CreateEmbedFooter::new(
                "💡 想要復仇嗎？再次輸入 /blackjack 即可重新開局！",
            ))
    }
}

// ── TicTacToe 3x3 按鈕矩陣渲染 ─────────────────────────────────
fn render_ttt_buttons(ttt: &TicTacToe) -> Vec<CreateActionRow> {
    let mut rows = Vec::new();
    for row in 0..3 {
        let mut buttons = Vec::new();
        for col in 0..3 {
            let pos = row * 3 + col;
            let mark = ttt.board[pos]; // 0: 空, 1: X (玩家), 2: O (AI)

            let (label, style, disabled) = match mark {
                1 => ("❌", ButtonStyle::Danger, true),
                2 => ("⭕", ButtonStyle::Primary, true),
                _ => {
                    if ttt.game_over {
                        ("➖", ButtonStyle::Secondary, true)
                    } else {
                        ("", ButtonStyle::Secondary, false)
                    }
                }
            };

            let mut btn = CreateButton::new(format!("ttt_play_{}", pos))
                .style(style)
                .disabled(disabled);

            if !label.is_empty() {
                btn = btn.label(label);
            } else {
                btn = btn.label(format!("{}", pos + 1));
            }
            buttons.push(btn);
        }
        rows.push(CreateActionRow::Buttons(buttons));
    }
    rows
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("🤖 {} 準備就緒！", ready.user.name);
        let commands = vec![
            CreateCommand::new("games").description("列出所有可玩的遊戲與指令"),
            CreateCommand::new("game").description("猜拳大賽"),
            CreateCommand::new("blackjack").description("黑傑克對決"),
            CreateCommand::new("slot").description("老虎機拉霸"),
            CreateCommand::new("guess")
                .description("數字炸彈")
                .add_option(
                    CreateCommandOption::new(CommandOptionType::Integer, "number", "你猜的數字")
                        .required(true),
                ),
            CreateCommand::new("roulette").description("俄羅斯輪盤"),
            CreateCommand::new("web").description("開啟網頁版遊戲中心"),
            CreateCommand::new("poker").description("德州撲克比牌（無下注）"),
            CreateCommand::new("ttt").description("圈圈叉叉對抗 AI（全新按鈕互動版）"),
            CreateCommand::new("wordle").description("Wordle 猜字（新局）"),
            CreateCommand::new("wguess")
                .description("Wordle 猜一個五字母英文單字")
                .add_option(
                    CreateCommandOption::new(CommandOptionType::String, "word", "五字母英文單字")
                        .required(true),
                ),
        ];
        let _ = Command::set_global_commands(&ctx.http, commands).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let data = ctx.data.read().await;
                match command.data.name.as_str() {
                    "games" => {
                        let embed = CreateEmbed::new()
                            .title("🎮 歡迎來到 Discord 遊戲大廳！")
                            .color(0x9b59b6) // 紫色質感外觀
                            .description("請在聊天框輸入以下斜線指令即可開始遊玩：")
                            .field("🟢 對決與策略遊戲", "- `/blackjack` : 🃏 **黑傑克** - 與莊家對決點數，試試你的膽量！\n- `/poker`     : 🂡 **德州撲克** - 經典比牌，推進翻牌圈看誰手氣好。\n- `/ttt`       : ❌⭕ **圈圈叉叉** - 全新全按鈕介面，與 AI 進行九宮格對戰。", false)
                            .field("🟡 運氣與趣味抽獎", "- `/game`      : 🖐️ **猜拳大賽** - 經典剪刀石頭布互動按鈕。\n- `/guess`     : 💣 **數字炸彈** - 與機器人輪流猜數字，小心別踩到！\n- `/slot`      : 🎰 **老虎機拉霸** - 一鍵拉霸，挑戰極限 JACKPOT！\n- `/roulette`  : 🔫 **俄羅斯輪盤** - 考驗運氣的致命時刻...", false)
                            .field("🔵 益智猜謎", "- `/wordle`    : 📝 **Wordle 猜字** - 開啟每日五字英文猜謎新局。\n- `/wguess`    : 🔠 **送出答案** - 用於輸入你要猜的 Wordle 單字。", false)
                            .field("🌐 網頁版擴充", "*想要更豐富的視覺體驗？輸入 `/web` 開啟網頁版遊戲中心！*", false);

                        let _ = command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new().embed(embed),
                                ),
                            )
                            .await;
                    }
                    "slot" => {
                        let (cards, level) = SlotMachine::spin();
                        let (comment, color) = match level {
                            0 => ("☁️ 殘念未中獎，再接再厲！", 0x95a5a6),
                            1 => ("✨ 拿到一對雙胞胎！運氣不錯喔！", 0x3498db),
                            2 => ("🔥 大勝利！三連線達成！ 🎉", 0xe67e22),
                            _ => ("🎉 👑 JACKPOT 終極大獎！！！神之右手！ 👑 🎉", 0xf1c40f), // 金色
                        };

                        let embed = CreateEmbed::new()
                            .title("🎰 老虎機拉霸 (Slot Machine)")
                            .color(color)
                            .description(format!(
                                "\n# 🎰 **[ {} | {} | {} ]** 🎰\n\n> **{}**",
                                cards[0], cards[1], cards[2], comment
                            ));

                        let _ = command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new().embed(embed),
                                ),
                            )
                            .await;
                        tokio::spawn(async { sync_game_stat("slotMachine").await });
                    }
                    "guess" => {
                        let bomb_lock = data.get::<BombState>().unwrap().clone();
                        let val = command.data.options[0].value.as_i64().unwrap() as u32;
                        let mut bomb = bomb_lock.write().await;

                        let mut embed = CreateEmbed::new().title("💣 數字炸彈 (Number Bomb)");

                        match bomb.guess(val) {
                            BombResult::Invalid => {
                                let content = format!(
                                    "⚠️ 違規操作！目前只能輸入 **{} 到 {}** 之間的數字！",
                                    bomb.min, bomb.max
                                );
                                let _ = command
                                    .create_response(
                                        &ctx.http,
                                        CreateInteractionResponse::Message(
                                            CreateInteractionResponseMessage::new()
                                                .content(content)
                                                .ephemeral(true),
                                        ),
                                    )
                                    .await;
                                return;
                            }
                            BombResult::Exploded => {
                                embed = embed
                                    .color(0xe74c3c)
                                    .description(format!("💥 **💥 BOMMMM!! 💥**\n\n答案就是 `{}`！你不幸踩到炸彈了 💀\n🏁 **GAME OVER** 玩家敗北...", val));
                                *bomb = NumberBomb::new();
                            }
                            BombResult::Safe => {
                                let mut desc = format!(
                                    "👤 **玩家回合**：\n└ 選擇了 `{}`... 🧭 安全過關！\n📉 剩餘安全範圍縮小為：`{} ~ {}`\n\n",
                                    val, bomb.min, bomb.max
                                );
                                let ai_val = bomb.ai_guess();
                                desc.push_str(&format!(
                                    "🤖 **機器人思考中...** 🤔\n└ 猜測了數字：`{}`\n",
                                    ai_val
                                ));

                                match bomb.guess(ai_val) {
                                    BombResult::Exploded => {
                                        embed = embed.color(0x2ecc71);
                                        desc.push_str(&format!(
                                            "\n💥 **💥 BOMMMM!! 💥**\n🤖 機器人踩到了炸彈 (`{}`)！\n🏆 **恭喜你贏了！** 機器人已被炸飛 🤖💀",
                                            ai_val
                                        ));
                                        *bomb = NumberBomb::new();
                                    }
                                    BombResult::Safe => {
                                        embed = embed.color(0x34495e);
                                        desc.push_str(&format!(
                                            "\n🔒 機器人也安全過關！\n👉 **輪到你了！** 請再次輸入 `/guess` 下注。\n🚨 最新安全範圍：**`{} ~ {}`**",
                                            bomb.min, bomb.max
                                        ));
                                    }
                                    BombResult::Invalid => {
                                        desc.push_str("\n⚠️ 機器人發生計算錯誤（猜測範圍外）。");
                                    }
                                }
                                embed = embed.description(desc);
                            }
                        }
                        let _ = command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new().embed(embed),
                                ),
                            )
                            .await;
                    }
                    "blackjack" => {
                        let bj_lock = data.get::<BlackjackState>().unwrap().clone();
                        let mut bj = bj_lock.write().await;
                        *bj = BlackjackCore::new();
                        let resp = CreateInteractionResponseMessage::new()
                            .embed(render_bj_embed(&bj))
                            .button(
                                CreateButton::new("bj_hit")
                                    .label("🟢 要牌")
                                    .style(ButtonStyle::Success),
                            )
                            .button(
                                CreateButton::new("bj_stand")
                                    .label("🔴 停牌")
                                    .style(ButtonStyle::Danger),
                            );
                        let _ = command
                            .create_response(&ctx.http, CreateInteractionResponse::Message(resp))
                            .await;
                    }
                    "game" => {
                        let embed = CreateEmbed::new()
                            .title("🖐️ 猜拳大賽 (Rock Paper Scissors)")
                            .color(0x34495e)
                            .description("請做出你的抉擇！點擊下方按鈕出拳：");

                        let resp = CreateInteractionResponseMessage::new()
                            .embed(embed)
                            .button(
                                CreateButton::new("btn_1")
                                    .label("✊ 石頭")
                                    .style(ButtonStyle::Secondary),
                            )
                            .button(
                                CreateButton::new("btn_2")
                                    .label("✌️ 剪刀")
                                    .style(ButtonStyle::Primary),
                            )
                            .button(
                                CreateButton::new("btn_3")
                                    .label("✋ 布")
                                    .style(ButtonStyle::Success),
                            );
                        let _ = command
                            .create_response(&ctx.http, CreateInteractionResponse::Message(resp))
                            .await;
                    }
                    "roulette" => {
                        let r_lock = data.get::<RouletteState>().unwrap().clone();
                        let mut r = r_lock.write().await;
                        let dead = r.pull_trigger();

                        let (msg, color) = if dead {
                            *r = Roulette::new();
                            ("💥 **💥 BANG!!! 💥**\n\n💀 伴隨一聲巨響，你中彈倒地了... 命運之輪重新洗牌。", 0xe74c3c)
                        } else {
                            (
                                "🔒 *咔噠*......\n\n呼，只有清脆的空槍聲。你安全了！💨",
                                0x2ecc71,
                            )
                        };

                        let embed = CreateEmbed::new()
                            .title("🔫 俄羅斯輪盤 (Russian Roulette)")
                            .color(color)
                            .description(msg);

                        let _ = command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new().embed(embed),
                                ),
                            )
                            .await;
                    }
                    "web" => {
                        let web_url = std::env::var("WEB_URL")
                            .unwrap_or_else(|_| "http://localhost:8080".to_string());

                        let embed = CreateEmbed::new()
                            .title("🌐 網頁版遊戲中心")
                            .color(0x1abc9c)
                            .description(format!("點擊以下連結開啟：{}\n\n> 與 Discord Bot 共享完全一致的底層遊戲運算核心！", web_url));

                        let _ = command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new().embed(embed),
                                ),
                            )
                            .await;
                    }
                    "poker" => {
                        let tx_lock = data.get::<TexasState>().unwrap().clone();
                        drop(data);
                        let mut tx = tx_lock.write().await;

                        *tx = TexasHoldem::new();

                        let hole_str = tx
                            .player_hole
                            .iter()
                            .map(|c| format!("`{}`", c.display()))
                            .collect::<Vec<_>>()
                            .join(" ");

                        let embed = CreateEmbed::new()
                            .title("🃏 德州撲克（Texas Hold'em）— 新局開始")
                            .color(0xd35400)
                            .field("👤 你的專屬底牌", hole_str, true)
                            .field("🧱 桌面公共牌", "`🂠 🂠 🂠` `🂠` `🂠` （蓋牌中）", true)
                            .footer(serenity::builder::CreateEmbedFooter::new(
                                "💡 請點擊下方按鈕依序推進牌局階段。",
                            ));

                        let resp = CreateInteractionResponseMessage::new()
                            .embed(embed)
                            .button(
                                CreateButton::new("tx_flop")
                                    .label("翻牌 Flop (3張)")
                                    .style(ButtonStyle::Primary),
                            )
                            .button(
                                CreateButton::new("tx_turn")
                                    .label("轉牌 Turn (1張)")
                                    .style(ButtonStyle::Secondary)
                                    .disabled(true),
                            )
                            .button(
                                CreateButton::new("tx_river")
                                    .label("河牌 River (1張)")
                                    .style(ButtonStyle::Secondary)
                                    .disabled(true),
                            )
                            .button(
                                CreateButton::new("tx_show")
                                    .label("🏆 開牌比較")
                                    .style(ButtonStyle::Success)
                                    .disabled(true),
                            );
                        let _ = command
                            .create_response(&ctx.http, CreateInteractionResponse::Message(resp))
                            .await;
                    }
                    "ttt" => {
                        let ttt_lock = data.get::<TttState>().unwrap().clone();
                        drop(data);
                        let mut ttt = ttt_lock.write().await;
                        *ttt = TicTacToe::new();

                        let embed = CreateEmbed::new()
                            .title("❌⭕ 圈圈叉叉（Tic-Tac-Toe）")
                            .color(0x34495e)
                            .description("戰局開始！你是 **❌ (先手)**，AI 是 **⭕**。\n👉 *請直接點擊下方九宮格按鈕落子：*");

                        let resp = CreateInteractionResponseMessage::new()
                            .embed(embed)
                            .components(render_ttt_buttons(&ttt));

                        let _ = command
                            .create_response(&ctx.http, CreateInteractionResponse::Message(resp))
                            .await;
                    }
                    "wordle" => {
                        let wl = data.get::<WordleState>().unwrap().clone();
                        drop(data);
                        let mut w = wl.write().await;
                        *w = WordleGame::new();

                        let embed = CreateEmbed::new()
                            .title("📝 Wordle 猜字")
                            .color(0xf39c12)
                            .description("猜一個 **5 字母英文單字**，共 **6** 次機會。\n🟩 = 正確位置 🟨 = 有此字母但位置錯誤 ⬛ = 字母不存在\n\n👉 請用斜線指令 `/wguess word:XXXXX` 開始輸入答案！");

                        let _ = command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new().embed(embed),
                                ),
                            )
                            .await;
                    }
                    "wguess" => {
                        let wl = data.get::<WordleState>().unwrap().clone();
                        drop(data);
                        let word = command.data.options[0]
                            .value
                            .as_str()
                            .unwrap_or("")
                            .to_string()
                            .to_uppercase();

                        if !WordleGame::is_valid_word(&word) {
                            let _ = command
                                .create_response(
                                    &ctx.http,
                                    CreateInteractionResponse::Message(
                                        CreateInteractionResponseMessage::new()
                                            .content(format!("⚠️ `{}` 不是合法的五字母英文字母組合，請重新確認！", word))
                                            .ephemeral(true),
                                    ),
                                )
                                .await;
                            return;
                        }

                        let mut w = wl.write().await;
                        if w.is_over() {
                            let _ = command
                                .create_response(
                                    &ctx.http,
                                    CreateInteractionResponse::Message(
                                        CreateInteractionResponseMessage::new()
                                            .content("⚠️ 本局遊戲已結束囉！請用 `/wordle` 開啟全新一局。")
                                            .ephemeral(true),
                                    ),
                                )
                                .await;
                            return;
                        }

                        let current_hints = match w.guess(&word) {
                            Some(h) => h,
                            None => {
                                let _ = command
                                    .create_response(
                                        &ctx.http,
                                        CreateInteractionResponse::Message(
                                            CreateInteractionResponseMessage::new()
                                                .content("⚠️ 猜測失敗，請檢查遊戲狀態。"),
                                        ),
                                    )
                                    .await;
                                return;
                            }
                        };

                        let mut base_lines = String::new();
                        let current_turn = w.guesses.len();
                        for (i, (g, h)) in w.guesses.iter().zip(w.hints.iter()).enumerate() {
                            if i < current_turn - 1 {
                                base_lines.push_str(&format!(
                                    "第 {} 次：` {} ` {}\n",
                                    i + 1,
                                    g,
                                    WordleGame::hints_to_emoji(h)
                                ));
                            }
                        }

                        let answer_str = w.answer.to_string();
                        let is_solved = w.solved;
                        let is_over = w.is_over();
                        let max_guesses = w.max_guesses;
                        let guesses_count = w.guesses.len();
                        drop(w);

                        let embed = CreateEmbed::new()
                            .title("📝 Wordle 猜字盤面")
                            .color(0xf39c12)
                            .description(format!(
                                "{}{}\n第 {} 次：` {} ` ⏳ 正在翻開字牌...",
                                base_lines,
                                if base_lines.is_empty() { "" } else { "\n" },
                                current_turn,
                                word
                            ));

                        let _ = command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new().embed(embed),
                                ),
                            )
                            .await;

                        let http_clone = ctx.http.clone();
                        let command_clone = command.clone();

                        tokio::spawn(async move {
                            let mut anim_emoji = String::new();
                            for i in 0..5 {
                                tokio::time::sleep(std::time::Duration::from_millis(350)).await;
                                let single_emoji = match current_hints[i] {
                                    game::LetterHint::Correct => "🟩",
                                    game::LetterHint::Present => "🟨",
                                    game::LetterHint::Absent => "⬛",
                                };
                                anim_emoji.push_str(single_emoji);

                                let mut placeholder = String::new();
                                for _ in i + 1..5 {
                                    placeholder.push_str("⬜");
                                }

                                let anim_embed = CreateEmbed::new()
                                    .title("📝 Wordle 猜字盤面")
                                    .color(0xf39c12)
                                    .description(format!(
                                        "{}{}\n第 {} 次：` {} ` {}{}",
                                        base_lines,
                                        if base_lines.is_empty() { "" } else { "\n" },
                                        current_turn,
                                        word,
                                        anim_emoji,
                                        placeholder
                                    ));

                                let _ = command_clone
                                    .edit_response(
                                        &http_clone,
                                        EditInteractionResponse::new().embed(anim_embed),
                                    )
                                    .await;
                            }

                            tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                            base_lines.push_str(&format!(
                                "第 {} 次：` {} ` {}\n",
                                current_turn, word, anim_emoji
                            ));

                            let (final_status, color) = if is_solved {
                                (
                                    format!("\n🎉 **恭喜答對！答案就是 `{}`** 🏆", answer_str),
                                    0x2ecc71,
                                )
                            } else if is_over {
                                (
                                    format!("\n💀 **挑戰失敗！殘念！答案是 `{}`**", answer_str),
                                    0xe74c3c,
                                )
                            } else {
                                (
                                    format!(
                                        "\n💡 還剩下 **{}** 次機會，繼續加油！請輸入 `/wguess`",
                                        (max_guesses as usize) - guesses_count
                                    ),
                                    0xf39c12,
                                )
                            };

                            let final_embed = CreateEmbed::new()
                                .title("📝 Wordle 猜字盤面")
                                .color(color)
                                .description(format!("{}{}", base_lines, final_status));

                            let _ = command_clone
                                .edit_response(
                                    &http_clone,
                                    EditInteractionResponse::new().embed(final_embed),
                                )
                                .await;
                        });
                    }
                    _ => {}
                }
            }
            Interaction::Component(component) => {
                let data = ctx.data.read().await;

                // ── 德州撲克按鈕流轉 ──────────────────────────────────────────
                if component.data.custom_id.starts_with("tx_") {
                    let tx_lock = data.get::<TexasState>().unwrap().clone();
                    drop(data);
                    let mut tx = tx_lock.write().await;

                    let action = component.data.custom_id.as_str();
                    match action {
                        "tx_flop" if tx.stage == TexasStage::PreFlop => tx.next_stage(),
                        "tx_turn" if tx.stage == TexasStage::Flop => tx.next_stage(),
                        "tx_river" if tx.stage == TexasStage::Turn => tx.next_stage(),
                        "tx_show" if tx.stage == TexasStage::River => tx.next_stage(),
                        _ => {}
                    }

                    let hole_str = tx
                        .player_hole
                        .iter()
                        .map(|c| format!("`{}`", c.display()))
                        .collect::<Vec<_>>()
                        .join(" ");

                    let comm_str = if tx.visible_community().is_empty() {
                        "`🂠 🂠 🂠` `🂠` `🂠`".to_string()
                    } else {
                        let mut visible = tx
                            .visible_community()
                            .iter()
                            .map(|c| format!("`{}`", c.display()))
                            .collect::<Vec<_>>()
                            .join(" ");
                        match tx.stage {
                            TexasStage::Flop => visible.push_str(" `🂠` `🂠`"),
                            TexasStage::Turn => visible.push_str(" `🂠`"),
                            _ => {}
                        }
                        visible
                    };

                    let mut embed = CreateEmbed::new();
                    let mut is_done = false;

                    if tx.stage == TexasStage::Showdown {
                        is_done = true;
                        let r = tx.result.as_ref().unwrap();
                        let pb = r
                            .player_best
                            .iter()
                            .map(|c| format!("`{}`", c.display()))
                            .collect::<Vec<_>>()
                            .join(" ");
                        let ab = r
                            .ai_best
                            .iter()
                            .map(|c| format!("`{}`", c.display()))
                            .collect::<Vec<_>>()
                            .join(" ");
                        let ai_hole = tx
                            .ai_hole
                            .iter()
                            .map(|c| format!("`{}`", c.display()))
                            .collect::<Vec<_>>()
                            .join(" ");

                        embed = embed
                            .title(format!("🏆 德州撲克結算 — {}", r.verdict))
                            .color(0x2ecc71)
                            .field("🧱 最終公共牌", comm_str, false)
                            .field(
                                "👤 你的底牌",
                                format!(
                                    "{} ➔ 最佳手牌: {}\n✨ 你的牌型: **【 {} 】**",
                                    hole_str,
                                    pb,
                                    r.player_rank.name()
                                ),
                                false,
                            )
                            .field(
                                "🤖 AI 底牌",
                                format!(
                                    "{} ➔ 最佳手牌: {}\n🤖 AI 牌型: **【 {} 】**",
                                    ai_hole,
                                    ab,
                                    r.ai_rank.name()
                                ),
                                false,
                            );
                    } else {
                        let (stage_label, color) = match tx.stage {
                            TexasStage::Flop => ("🟢 Flop (翻牌圈)", 0x3498db),
                            TexasStage::Turn => ("🔵 Turn (轉牌圈)", 0x9b59b6),
                            TexasStage::River => ("🔴 River (河牌圈)", 0xe74c3c),
                            _ => ("🃏 德州撲克", 0xd35400),
                        };

                        embed = embed
                            .title(format!("🂡 德州撲克 — {}", stage_label))
                            .color(color)
                            .field("👤 你的專屬底牌", hole_str, true)
                            .field("🧱 桌面公共牌", comm_str, true)
                            .footer(serenity::builder::CreateEmbedFooter::new(
                                "💬 請點擊按鈕推動牌局進入下一階段。",
                            ));
                    };

                    let mut resp = CreateInteractionResponseMessage::new().embed(embed);
                    if !is_done {
                        resp = resp
                            .button(
                                CreateButton::new("tx_flop")
                                    .label("翻牌 Flop")
                                    .style(ButtonStyle::Primary)
                                    .disabled(tx.stage != TexasStage::PreFlop),
                            )
                            .button(
                                CreateButton::new("tx_turn")
                                    .label("轉牌 Turn")
                                    .style(ButtonStyle::Secondary)
                                    .disabled(tx.stage != TexasStage::Flop),
                            )
                            .button(
                                CreateButton::new("tx_river")
                                    .label("河牌 River")
                                    .style(ButtonStyle::Secondary)
                                    .disabled(tx.stage != TexasStage::Turn),
                            )
                            .button(
                                CreateButton::new("tx_show")
                                    .label("🏆 開牌比較")
                                    .style(ButtonStyle::Success)
                                    .disabled(tx.stage != TexasStage::River),
                            );
                    }
                    let _ = component
                        .create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(resp))
                        .await;
                    return;
                }

                // ── 黑傑克按鈕互動 ────────────────────────────────────────────
                if component.data.custom_id.starts_with("bj_") {
                    let bj_lock = data.get::<BlackjackState>().unwrap().clone();
                    let mut bj = bj_lock.write().await;
                    if !bj.is_game_over {
                        if component.data.custom_id == "bj_hit" {
                            bj.hit();
                        } else if component.data.custom_id == "bj_stand" {
                            bj.stand();
                        }
                    }
                    let mut resp =
                        CreateInteractionResponseMessage::new().embed(render_bj_embed(&bj));
                    if !bj.is_game_over {
                        resp = resp
                            .button(
                                CreateButton::new("bj_hit")
                                    .label("🟢 要牌")
                                    .style(ButtonStyle::Success),
                            )
                            .button(
                                CreateButton::new("bj_stand")
                                    .label("🔴 停牌")
                                    .style(ButtonStyle::Danger),
                            );
                    }
                    let _ = component
                        .create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(resp))
                        .await;
                    return;
                }

                // ── 猜拳按鈕互動 ────────────────────────────────────────────
                if component.data.custom_id.starts_with("btn_") {
                    let p_val = component
                        .data
                        .custom_id
                        .replace("btn_", "")
                        .parse::<u32>()
                        .unwrap_or(1);
                    let (res, b_val) = GuessGame::play(p_val);
                    let emojis = ["", "✊ 石頭", "✌️ 剪刀", "✋ 布"];

                    let (title, color) = match res {
                        GameResult::Win => ("🎉 恭喜！你贏了！ 🏆", 0x2ecc71),
                        GameResult::Lose => ("💀 殘念！你輸給機器人了...", 0xe74c3c),
                        GameResult::Draw => ("🤝 勢均力敵！平手！", 0x95a5a6),
                    };

                    let embed = CreateEmbed::new()
                        .title(title)
                        .color(color)
                        .description(format!(
                            "\n👤 你出：**{}** \n\n  vs  \n\n🤖 機器人出：**{}**",
                            emojis[p_val as usize], emojis[b_val as usize]
                        ));

                    let _ = component
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().embed(embed),
                            ),
                        )
                        .await;
                    return;
                }

                // ── 圈圈叉叉按鈕互動 ──────────────────────────────────────────
                if component.data.custom_id.starts_with("ttt_play_") {
                    let ttt_lock = data.get::<TttState>().unwrap().clone();
                    drop(data);
                    let mut ttt = ttt_lock.write().await;

                    let pos = component
                        .data
                        .custom_id
                        .replace("ttt_play_", "")
                        .parse::<usize>()
                        .unwrap_or(0);

                    let mut error_msg = None;
                    if ttt.game_over {
                        error_msg = Some("🛑 本局遊戲已結束！請輸入 `/ttt` 重啟新局。");
                    } else if !ttt.player_move(pos) {
                        error_msg = Some("❌ 這個位置已經有棋子了，請換別格落子！");
                    }

                    if let Some(err) = error_msg {
                        let _ = component
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .content(err)
                                        .ephemeral(true),
                                ),
                            )
                            .await;
                        return;
                    }

                    let (status_str, color) = match ttt.winner {
                        1 => ("🏆 戰果揭曉：恭喜你贏了！ 🎉", 0x2ecc71),
                        2 => ("🤖 戰果揭曉：AI 獲勝！再接再厲！ 💀", 0xe74c3c),
                        3 => ("🤝 戰果揭曉：雙方平手！ ⚖️", 0x95a5a6),
                        _ => (
                            "❌⭕ 圈圈叉叉\n雙方交戰中... 請繼續點擊九宮格落子！",
                            0x34495e,
                        ),
                    };

                    let embed = CreateEmbed::new()
                        .title("❌⭕ 圈圈叉叉（Tic-Tac-Toe）")
                        .color(color)
                        .description(status_str);

                    // 修正點：錯誤原為 .components(row) 改為傳入集合。
                    // 這裡的 render_ttt_buttons 已返回 Vec<CreateActionRow>，所以直接傳入即可
                    let resp = CreateInteractionResponseMessage::new()
                        .embed(embed)
                        .components(render_ttt_buttons(&ttt));

                    let _ = component
                        .create_response(&ctx.http, CreateInteractionResponse::UpdateMessage(resp))
                        .await;
                    return;
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("找不到 TOKEN");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("建立失敗");
    {
        let mut data = client.data.write().await;
        data.insert::<BombState>(Arc::new(RwLock::new(NumberBomb::new())));
        data.insert::<RouletteState>(Arc::new(RwLock::new(Roulette::new())));
        data.insert::<BlackjackState>(Arc::new(RwLock::new(BlackjackCore::new())));
        data.insert::<TttState>(Arc::new(RwLock::new(TicTacToe::new())));
        data.insert::<WordleState>(Arc::new(RwLock::new(WordleGame::new())));
        data.insert::<TexasState>(Arc::new(RwLock::new(TexasHoldem::new())));
    }
    let _ = client.start().await;
}
