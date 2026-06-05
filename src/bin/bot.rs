// 宣告同層目錄的模組
#[path = "../game.rs"]
mod game;
#[path = "../todo.rs"]
mod todo;

use serenity::async_trait;
use serenity::builder::{
    CreateButton,
    CreateCommand,
    CreateCommandOption,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::model::application::{ Command, Interaction, CommandOptionType, ButtonStyle };
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::sync::Arc;

use game::{
    BlackjackCore, NumberBomb, GuessGame, SlotMachine, Roulette, GameResult, BombResult,
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
    let api_url = std::env
        ::var("API_URL")
        .unwrap_or_else(|_| "http://localhost:8000/api".to_string());
    let url = format!("{}/games/{}/stat", api_url, game_name);
    if let Ok(client) = reqwest::Client::new().post(&url).send().await {
        if client.status().is_success() {
            println!("✅ 已同步遊戲統計: {}", game_name);
        }
    }
}

fn render_bj(bj: &BlackjackCore) -> String {
    let p_score = BlackjackCore::get_score(&bj.player_cards);
    let b_score = BlackjackCore::get_score(&bj.bot_cards);
    let p_str = bj.player_cards
        .iter()
        .map(|c| format!("`[{}]` ", c))
        .collect::<String>();

    if !bj.is_game_over {
        format!(
            "🃏 **Discord 黑傑克**\n👤 **你的手牌**: {} (總分: **{}**)\n🤖 **莊家手牌**: `[{}]` `[?]` \n\n*請選擇【要牌】或【停牌】！*",
            p_str,
            p_score,
            bj.bot_cards[0]
        )
    } else {
        let b_str = bj.bot_cards
            .iter()
            .map(|c| format!("`[{}]` ", c))
            .collect::<String>();
        let res = if p_score > 21 {
            "💥 **你爆牌了！莊家獲勝！** 💀"
        } else if b_score > 21 {
            "🎉 **莊家爆牌了！你贏了！** 🏆"
        } else if p_score > b_score {
            "🏆 **你贏了！** 🎉"
        } else if p_score < b_score {
            "❌ **你輸了...** 💀"
        } else {
            "🤝 **平手！**"
        };
        format!(
            "🃏 **Discord 黑傑克 (結算)**\n👤 **你的手牌**: {} (總分: **{}**)\n🤖 **莊家手牌**: {} (總分: **{}**)\n\n{}",
            p_str,
            p_score,
            b_str,
            b_score,
            res
        )
    }
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
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "number",
                        "你猜的數字"
                    ).required(true)
                ),
            CreateCommand::new("roulette").description("俄羅斯輪盤"),
            CreateCommand::new("dice").description("擲骰子"),
            CreateCommand::new("fortune").description("抽取今日運勢"),
            CreateCommand::new("coinflip").description("硬幣翻轉"),
            CreateCommand::new("web").description("開啟網頁版遊戲中心"),
            CreateCommand::new("poker").description("德州撲克比牌（無下注）"),
            CreateCommand::new("ttt").description("圈圈叉叉 對抗 AI（開新局）")
                .add_option(CreateCommandOption::new(
                    CommandOptionType::Integer, "pos", "落子位置 1-9"
                ).required(false)),
            CreateCommand::new("wordle").description("Wordle 猜字（新局）"),
            CreateCommand::new("wguess")
                .description("Wordle 猜一個五字母英文單字")
                .add_option(CreateCommandOption::new(
                    CommandOptionType::String, "word", "五字母英文單字"
                ).required(true)),
        ];
        let _ = Command::set_global_commands(&ctx.http, commands).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let data = ctx.data.read().await;
                match command.data.name.as_str() {
                    "games" => {
                        let content =
                            "## 🎮 機器人遊戲清單\n- `/game` : 🖐️ 猜拳\n- `/blackjack` : 🃏 黑傑克\n- `/guess` : 💣 數字炸彈 (對抗 AI)\n- `/slot` : 🎰 老虎機\n- `/roulette` : 🔫 輪盤\n- `/poker` : 🂡 德州撲克比牌\n- `/ttt [pos]` : ❌⭕ 圈圈叉叉 (AI)\n- `/wordle` + `/wguess` : 📝 Wordle 猜字";
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(content)
                            )
                        ).await;
                    }
                    "slot" => {
                        let (cards, level) = SlotMachine::spin();
                        let comments = [
                            "☁️ 殘念未中獎。",
                            "✨ 拿到一對雙胞胎！",
                            "🔥 大勝利！三連線！",
                            "🎉 👑 JACKPOT 終極大獎！",
                        ];
                        let content = format!(
                            "🎰 **[ {} | {} | {} ]**\n> *{}*",
                            cards[0],
                            cards[1],
                            cards[2],
                            comments[level as usize]
                        );
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(content)
                            )
                        ).await;
                        tokio::spawn(async { sync_game_stat("slotMachine").await });
                    }
                    "guess" => {
                        let bomb_lock = data.get::<BombState>().unwrap().clone();
                        let val = command.data.options[0].value.as_i64().unwrap() as u32;
                        let mut bomb = bomb_lock.write().await;
                        let mut msg = String::new();

                        match bomb.guess(val) {
                            BombResult::Invalid => {
                                msg = format!("⚠️ 請輸入 {} 到 {} 之間的數字", bomb.min, bomb.max);
                            }
                            BombResult::Exploded => {
                                msg =
                                    format!("💥 **BOMMMM!!** 答案是 {}！你踩到炸彈敗北了 💀", val);
                                *bomb = NumberBomb::new();
                            }
                            BombResult::Safe => {
                                msg = format!(
                                    "✅ 玩家安全！目前範圍：**{} ~ {}**\n",
                                    bomb.min,
                                    bomb.max
                                );
                                let ai_val = bomb.ai_guess();
                                msg.push_str(&format!("🤖 機器人猜了：**{}**\n", ai_val));
                                match bomb.guess(ai_val) {
                                    BombResult::Exploded => {
                                        msg.push_str(
                                            &format!("💥 **BOMMMM!!** 機器人踩到炸彈 ({})！你贏了！🏆", ai_val)
                                        );
                                        *bomb = NumberBomb::new();
                                    }
                                    BombResult::Safe => {
                                        msg.push_str(
                                            &format!(
                                                "🔒 機器人也安全！目前範圍：**{} ~ {}**",
                                                bomb.min,
                                                bomb.max
                                            )
                                        );
                                    }
                                    BombResult::Invalid => {
                                        msg.push_str("⚠️ 機器人猜測範圍外。");
                                    }
                                }
                            }
                        }
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(msg)
                            )
                        ).await;
                    }
                    "blackjack" => {
                        let bj_lock = data.get::<BlackjackState>().unwrap().clone();
                        let mut bj = bj_lock.write().await;
                        *bj = BlackjackCore::new();
                        let resp = CreateInteractionResponseMessage::new()
                            .content(render_bj(&bj))
                            .button(
                                CreateButton::new("bj_hit")
                                    .label("🟢 要牌")
                                    .style(ButtonStyle::Success)
                            )
                            .button(
                                CreateButton::new("bj_stand")
                                    .label("🔴 停牌")
                                    .style(ButtonStyle::Danger)
                            );
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(resp)
                        ).await;
                    }
                    "game" => {
                        let resp = CreateInteractionResponseMessage::new()
                            .content("### 🖐️ 猜拳大賽！")
                            .button(
                                CreateButton::new("btn_1")
                                    .label("✊ 石頭")
                                    .style(ButtonStyle::Secondary)
                            )
                            .button(
                                CreateButton::new("btn_2")
                                    .label("✌️ 剪刀")
                                    .style(ButtonStyle::Primary)
                            )
                            .button(
                                CreateButton::new("btn_3")
                                    .label("✋ 布")
                                    .style(ButtonStyle::Success)
                            );
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(resp)
                        ).await;
                    }
                    "roulette" => {
                        let r_lock = data.get::<RouletteState>().unwrap().clone();
                        let mut r = r_lock.write().await;
                        let dead = r.pull_trigger();
                        let msg = if dead {
                            *r = Roulette::new();
                            "💥 **BANG!** 你中彈了... 遊戲重置。"
                        } else {
                            "🔒 *咔噠*... 安全！"
                        };
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(msg)
                            )
                        ).await;
                    }
                    "web" => {
                        let web_url = std::env
                            ::var("WEB_URL")
                            .unwrap_or_else(|_| "http://localhost:8080".to_string());
                        let content =
                            format!("🌐 **網頁版遊戲中心**\n點擊以下連結開啟：{}\n\n> 與 Discord Bot 共享完全一致的底層遊戲運算核心！", web_url);
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(content)
                            )
                        ).await;
                    }
                    // ── 德州撲克 ────────────────────────────────────────────
                    "poker" => {
                        let tx_lock = data.get::<TexasState>().unwrap().clone();
                        drop(data);
                        let mut tx = tx_lock.write().await;

                        // 每次 /poker 開新局
                        *tx = TexasHoldem::new();

                        let hole_str = tx.player_hole.iter()
                            .map(|c| format!("`{}`", c.display()))
                            .collect::<Vec<_>>().join(" ");

                        let content = format!(
                            "## 🂡 德州撲克（Texas Hold'em）\n\
                             荷官已發底牌！\n\
                             👤 **你的底牌**：{}\n\
                             🃏 **公共牌**：（尚未翻牌）\n\n\
                             用以下按鈕推進牌局：",
                            hole_str
                        );
                        let resp = CreateInteractionResponseMessage::new()
                            .content(content)
                            .button(CreateButton::new("tx_flop").label("翻牌 Flop (3張)").style(ButtonStyle::Primary))
                            .button(CreateButton::new("tx_turn").label("轉牌 Turn (1張)").style(ButtonStyle::Secondary))
                            .button(CreateButton::new("tx_river").label("河牌 River (1張)").style(ButtonStyle::Secondary))
                            .button(CreateButton::new("tx_show").label("🏆 開牌比較").style(ButtonStyle::Success));
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(resp)
                        ).await;
                    }

                    // ── 圈圈叉叉 ────────────────────────────────────────────
                    "ttt" => {
                        let ttt_lock = data.get::<TttState>().unwrap().clone();
                        drop(data);
                        let pos_opt = command.data.options.first()
                            .and_then(|o| o.value.as_i64())
                            .map(|v| (v - 1) as usize);

                        let content = {
                            let mut ttt = ttt_lock.write().await;
                            if pos_opt.is_none() {
                                // 開新局
                                *ttt = TicTacToe::new();
                                format!("## ❌⭕ 圈圈叉叉（新局）\n你是 X，AI 是 O\n\n```\n{}\n```\n\n用 `/ttt pos:1-9` 落子（位置如鍵盤數字鍵）", ttt.render_text())
                            } else {
                                let pos = pos_opt.unwrap();
                                if ttt.game_over {
                                    "⚠️ 遊戲已結束，請用 `/ttt` 開始新局。".to_string()
                                } else if !ttt.player_move(pos) {
                                    format!("⚠️ 位置 {} 無效或已有棋子，請重新輸入 1-9。", pos + 1)
                                } else {
                                    let status = match ttt.winner {
                                        1 => "🏆 **你贏了！**",
                                        2 => "🤖 **AI 獲勝！**",
                                        3 => "🤝 **平手！**",
                                        _ => "繼續落子中...",
                                    };
                                    format!("## ❌⭕ 圈圈叉叉\n```\n{}\n```\n\n{}", ttt.render_text(), status)
                                }
                            }
                        };
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(content)
                            )
                        ).await;
                    }

                    // ── Wordle 新局 ──────────────────────────────────────────
                    "wordle" => {
                        let wl = data.get::<WordleState>().unwrap().clone();
                        drop(data);
                        let mut w = wl.write().await;
                        *w = WordleGame::new();
                        let content = "## 📝 Wordle 猜字（新局開始！）\n\
                            猜一個 **5 字母英文單字**，共 **6** 次機會。\n\
                            🟩 = 正確位置　🟨 = 有但位置錯　⬛ = 不存在\n\n\
                            用 `/wguess word:XXXXX` 開始猜！";
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(content)
                            )
                        ).await;
                    }

                    // ── Wordle 猜測 ──────────────────────────────────────────
                    "wguess" => {
                        let wl = data.get::<WordleState>().unwrap().clone();
                        drop(data);
                        let word = command.data.options[0].value
                            .as_str().unwrap_or("").to_string();
                        let content = {
                            let mut w = wl.write().await;
                            if w.is_over() {
                                "⚠️ 遊戲已結束，請用 `/wordle` 開新局。".to_string()
                            } else {
                                match w.guess(&word) {
                                    None => format!("⚠️ `{}` 不是合法的五字母單字。", word),
                                    Some(hints) => {
                                        let mut lines = String::new();
                                        for (i, (g, h)) in w.guesses.iter().zip(w.hints.iter()).enumerate() {
                                            lines.push_str(&format!(
                                                "第{}次：`{}` {}\n",
                                                i + 1, g, WordleGame::hints_to_emoji(h)
                                            ));
                                        }
                                        if w.solved {
                                            format!("## 📝 Wordle\n{}\n🎉 **答對了！答案就是 `{}`**", lines, w.answer)
                                        } else if w.is_over() {
                                            format!("## 📝 Wordle\n{}\n💀 **用完次數！答案是 `{}`**", lines, w.answer)
                                        } else {
                                            let remaining = w.max_guesses as usize - w.guesses.len();
                                            format!("## 📝 Wordle\n{}\n還剩 **{}** 次機會，繼續猜！", lines, remaining)
                                        }
                                    }
                                }
                            }
                        };
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(content)
                            )
                        ).await;
                    }

                    _ => {}
                }
            }
            Interaction::Component(component) => {
                let data = ctx.data.read().await;

                // ── 德州撲克按鈕 ──────────────────────────────────────────
                if component.data.custom_id.starts_with("tx_") {
                    let tx_lock = data.get::<TexasState>().unwrap().clone();
                    drop(data);
                    let mut tx = tx_lock.write().await;

                    let action = component.data.custom_id.as_str();

                    // 推進 stage
                    match action {
                        "tx_flop"  if tx.stage == TexasStage::PreFlop  => tx.next_stage(),
                        "tx_turn"  if tx.stage == TexasStage::Flop     => tx.next_stage(),
                        "tx_river" if tx.stage == TexasStage::Turn     => tx.next_stage(),
                        "tx_show"  if tx.stage == TexasStage::River    => tx.next_stage(),
                        _ => {}
                    }

                    let hole_str = tx.player_hole.iter()
                        .map(|c| format!("`{}`", c.display()))
                        .collect::<Vec<_>>().join(" ");
                    let comm_str = if tx.visible_community().is_empty() {
                        "（尚未翻牌）".to_string()
                    } else {
                        tx.visible_community().iter()
                            .map(|c| format!("`{}`", c.display()))
                            .collect::<Vec<_>>().join(" ")
                    };

                    let (content, is_done) = if tx.stage == TexasStage::Showdown {
                        let r = tx.result.as_ref().unwrap();
                        let pb = r.player_best.iter().map(|c| format!("`{}`", c.display())).collect::<Vec<_>>().join(" ");
                        let ab = r.ai_best.iter()    .map(|c| format!("`{}`", c.display())).collect::<Vec<_>>().join(" ");
                        let ai_hole = tx.ai_hole.iter().map(|c| format!("`{}`", c.display())).collect::<Vec<_>>().join(" ");
                        (format!(
                            "## 🂡 德州撲克 — 開牌結算！\n\
                             🃏 **公共牌**：{}\n\n\
                             👤 **你的底牌**：{}　最佳5張：{}\n　　牌型：**{}**\n\
                             🤖 **AI 底牌**：{}　最佳5張：{}\n　　牌型：**{}**\n\n\
                             {}",
                            comm_str,
                            hole_str, pb, r.player_rank.name(),
                            ai_hole,  ab, r.ai_rank.name(),
                            r.verdict
                        ), true)
                    } else {
                        let stage_label = match tx.stage {
                            TexasStage::Flop  => "Flop（翻牌）",
                            TexasStage::Turn  => "Turn（轉牌）",
                            TexasStage::River => "River（河牌）",
                            _                 => "",
                        };
                        (format!(
                            "## 🂡 德州撲克 — {}\n\
                             👤 **你的底牌**：{}\n\
                             🃏 **公共牌**：{}\n",
                            stage_label, hole_str, comm_str
                        ), false)
                    };

                    let mut resp = CreateInteractionResponseMessage::new().content(content);
                    if !is_done {
                        if tx.stage == TexasStage::Flop {
                            resp = resp.button(CreateButton::new("tx_turn").label("轉牌 Turn").style(ButtonStyle::Secondary));
                        }
                        if tx.stage == TexasStage::Turn {
                            resp = resp.button(CreateButton::new("tx_river").label("河牌 River").style(ButtonStyle::Secondary));
                        }
                        if tx.stage == TexasStage::River {
                            resp = resp.button(CreateButton::new("tx_show").label("🏆 開牌比較").style(ButtonStyle::Success));
                        }
                    }
                    let _ = component.create_response(
                        &ctx.http,
                        CreateInteractionResponse::UpdateMessage(resp)
                    ).await;
                    return;
                }
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
                    let mut resp = CreateInteractionResponseMessage::new().content(render_bj(&bj));
                    if !bj.is_game_over {
                        resp = resp
                            .button(
                                CreateButton::new("bj_hit")
                                    .label("🟢 要牌")
                                    .style(ButtonStyle::Success)
                            )
                            .button(
                                CreateButton::new("bj_stand")
                                    .label("🔴 停牌")
                                    .style(ButtonStyle::Danger)
                            );
                    }
                    let _ = component.create_response(
                        &ctx.http,
                        CreateInteractionResponse::UpdateMessage(resp)
                    ).await;
                    return;
                }
                if component.data.custom_id.starts_with("btn_") {
                    let p_val = component.data.custom_id
                        .replace("btn_", "")
                        .parse::<u32>()
                        .unwrap_or(1);
                    let (res, b_val) = GuessGame::play(p_val);
                    let emojis = ["", "✊ 石頭", "✌️ 剪刀", "✋ 布"];
                    let title = match res {
                        GameResult::Win => "🎉 你贏了！",
                        GameResult::Lose => "💀 你輸了...",
                        GameResult::Draw => "🤝 平手！",
                    };
                    let _ = component.create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new().content(
                                format!(
                                    "## {}\n你：{} vs 機器人：{}",
                                    title,
                                    emojis[p_val as usize],
                                    emojis[b_val as usize]
                                )
                            )
                        )
                    ).await;
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
        .event_handler(Handler).await
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
