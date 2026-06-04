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

use game::{ BlackjackCore, NumberBomb, GuessGame, SlotMachine, Roulette, GameResult, BombResult };

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
            CreateCommand::new("web").description("開啟網頁版遊戲中心")
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
                            "## 🎮 機器人遊戲清單\n- `/game` : 🖐️ 猜拳\n- `/blackjack` : 🃏 黑傑克\n- `/guess` : 💣 數字炸彈 (對抗 AI)\n- `/slot` : 🎰 老容機\n- `/roulette` : 🔫 輪盤";
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
                    _ => {}
                }
            }
            Interaction::Component(component) => {
                let data = ctx.data.read().await;
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
    }
    let _ = client.start().await;
}
