mod game;
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

struct TodoState;
impl TypeMapKey for TodoState {
    type Value = Arc<RwLock<todo::TodoList>>;
}

struct BombState;
impl TypeMapKey for BombState {
    type Value = Arc<RwLock<game::NumberBomb>>;
}

struct RouletteState;
impl TypeMapKey for RouletteState {
    type Value = Arc<RwLock<game::Roulette>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("🤖 {} 準備就緒！", ready.user.name);

        let commands = vec![
            CreateCommand::new("games").description("列出所有可玩的遊戲與指令"),
            CreateCommand::new("game").description("猜拳對決"),
            CreateCommand::new("guess")
                .description("數字炸彈遊戲")
                .add_option(CreateCommandOption::new(CommandOptionType::Integer, "number", "你猜的數字").required(true)),
            CreateCommand::new("dice")
                .description("擲骰子")
                .add_option(CreateCommandOption::new(CommandOptionType::Integer, "sides", "面數 (預設 6)"))
                .add_option(CreateCommandOption::new(CommandOptionType::Integer, "count", "個數 (預設 1)")),
            CreateCommand::new("fortune").description("抽取今日運勢"),
            CreateCommand::new("coinflip").description("硬幣翻轉 (正面/反面)"),
            CreateCommand::new("roulette").description("俄羅斯輪盤 (生存挑戰)"),
            CreateCommand::new("reset_roulette").description("重置俄羅斯輪盤"),
            CreateCommand::new("add")
                .description("新增事項")
                .add_option(CreateCommandOption::new(CommandOptionType::String, "content", "任務內容").required(true)),
            CreateCommand::new("list").description("查看清單"),
        ];

        let _ = Command::set_global_commands(&ctx.http, commands).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let data = ctx.data.read().await;

                match command.data.name.as_str() {
                    "games" => {
                        let content = "## 🎮 機器人遊戲清單\n\
                        - `/game` : 🖐️ 猜拳大賽 (使用按鈕互動)\n\
                        - `/guess` : 💣 數字炸彈 (1-100 猜數字)\n\
                        - `/dice` : 🎲 擲骰子 (可自定義面數與數量)\n\
                        - `/fortune` : 🔮 今日運勢占卜\n\
                        - `/coinflip` : 🟡 硬幣翻轉 (正面或反面)\n\
                        - `/roulette` : 🔫 俄羅斯輪盤 (6 發 1 中，挑戰生存)\n\
                        - `/reset_roulette` : 🔄 重置輪盤子彈位置\n\n\
                        *快選擇一個遊戲來挑戰吧！*";
                        
                        let _ = command.create_response(&ctx.http, 
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(content)
                            )
                        ).await;
                    }
                    "coinflip" => {
                        let (side, emoji) = game::CoinFlip::flip();
                        let content = format!("🪙 **硬幣翻轉結果**\n## {} {}\n> *結果是：{}！*", emoji, side, side);
                        let _ = command.create_response(&ctx.http, 
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(content)
                            )
                        ).await;
                    }
                    "roulette" => {
                        let roulette_lock = data.get::<RouletteState>().unwrap().clone();
                        let mut roulette = roulette_lock.write().await;
                        let (dead, msg) = roulette.pull_trigger();
                        
                        if dead {
                            *roulette = game::Roulette::new(); // 死了就重開
                        }

                        let _ = command.create_response(&ctx.http, 
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(format!("🔫 **俄羅斯輪盤**\n{}", msg))
                            )
                        ).await;
                    }
                    "reset_roulette" => {
                        let roulette_lock = data.get::<RouletteState>().unwrap().clone();
                        *roulette_lock.write().await = game::Roulette::new();
                        let _ = command.create_response(&ctx.http, 
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content("🔄 **輪盤已重新裝彈！** 子彈位置已隨機變更。")
                            )
                        ).await;
                    }
                    "dice" => {
                        let sides = command.data.options.get(0).and_then(|o| o.value.as_i64()).unwrap_or(6) as u32;
                        let count = command.data.options.get(1).and_then(|o| o.value.as_i64()).unwrap_or(1) as u32;
                        
                        if sides == 0 || count == 0 || count > 20 {
                            let _ = command.create_response(&ctx.http, 
                                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content("❌ 無效參數"))
                            ).await;
                            return;
                        }

                        let (rolls, total) = game::Dice::roll(sides, count);
                        let rolls_str = rolls.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(", ");
                        let content = format!("🎲 **擲骰子結果**\n結果: `[{}]` \n**總和: {}**", rolls_str, total);

                        let _ = command.create_response(&ctx.http, 
                            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(content))
                        ).await;
                    }
                    "fortune" => {
                        let (luck, emoji, desc) = game::Fortune::draw();
                        let content = format!("🔮 **今日運勢占卜**\n## {} {}\n> *{}*", luck, emoji, desc);
                        let _ = command.create_response(&ctx.http, 
                            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(content))
                        ).await;
                    }
                    "guess" => {
                        let bomb_lock = data.get::<BombState>().unwrap().clone();
                        let guess_val = command.data.options[0].value.as_i64().unwrap() as u32;
                        let mut bomb = bomb_lock.write().await;
                        let (msg, exploded) = bomb.guess(guess_val);
                        if exploded { *bomb = game::NumberBomb::new(); }
                        let _ = command.create_response(&ctx.http, 
                            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(msg))
                        ).await;
                    }
                    "game" => {
                        let resp = CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("### 🖐️ 猜拳大賽！")
                                .button(CreateButton::new("btn_1").label("✊ 石頭").style(ButtonStyle::Secondary))
                                .button(CreateButton::new("btn_2").label("✌️ 剪刀").style(ButtonStyle::Primary))
                                .button(CreateButton::new("btn_3").label("✋ 布").style(ButtonStyle::Success))
                        );
                        let _ = command.create_response(&ctx.http, resp).await;
                    }
                    "add" => {
                        let todo_lock = data.get::<TodoState>().unwrap().clone();
                        let content = command.data.options[0].value.as_str().unwrap().to_string();
                        let msg = todo_lock.write().await.add_task(content);
                        let _ = command.create_response(&ctx.http, 
                            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(msg))
                        ).await;
                    }
                    "list" => {
                        let todo_lock = data.get::<TodoState>().unwrap().clone();
                        let msg = todo_lock.read().await.list_tasks();
                        let _ = command.create_response(&ctx.http, 
                            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(msg))
                        ).await;
                    }
                    _ => {}
                }
            }
            Interaction::Component(component) => {
                let player_val = component.data.custom_id.replace("btn_", "").parse::<u32>().unwrap_or(1);
                let (result, bot_val, quote) = game::GuessGame::play(player_val);
                let title = match result {
                    game::GameResult::Win => "🎉 你贏了！",
                    game::GameResult::Lose => "💀 你輸了...",
                    game::GameResult::Draw => "🤝 平手！",
                };
                let content = format!("## {}\n你出 **{}** vs 機器人出 **{}**\n> *「{}」*", 
                    title, game::GuessGame::get_emoji(player_val), game::GuessGame::get_emoji(bot_val), quote);
                let _ = component.create_response(&ctx.http, 
                    CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(content))
                ).await;
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
        data.insert::<TodoState>(Arc::new(RwLock::new(todo::TodoList::new())));
        data.insert::<BombState>(Arc::new(RwLock::new(game::NumberBomb::new())));
        data.insert::<RouletteState>(Arc::new(RwLock::new(game::Roulette::new())));
    }

    let _ = client.start().await;
}
