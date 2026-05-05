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

// 新增：註冊炸彈遊戲狀態
struct BombState;
impl TypeMapKey for BombState {
    type Value = Arc<RwLock<game::NumberBomb>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("🤖 {} 準備就緒！", ready.user.name);

        let commands = vec![
            CreateCommand::new("game").description("猜拳對決"),
            CreateCommand::new("add")
                .description("新增事項")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "content",
                        "任務內容"
                    ).required(true)
                ),
            CreateCommand::new("list").description("查看清單"),
            // 新增：數字炸彈指令
            CreateCommand::new("guess")
                .description("猜一個數字，踩到炸彈就輸了！")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Integer,
                        "number",
                        "你猜的數字"
                    ).required(true)
                ),
            CreateCommand::new("reset_bomb").description("重置炸彈範圍")
        ];

        let _ = Command::set_global_commands(&ctx.http, commands).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let data = ctx.data.read().await;

                match command.data.name.as_str() {
                    "guess" => {
                        let bomb_lock = data.get::<BombState>().unwrap().clone();
                        let guess_val = command.data.options[0].value.as_i64().unwrap() as u32;

                        let mut bomb = bomb_lock.write().await;
                        let (msg, exploded) = bomb.guess(guess_val);

                        if exploded {
                            *bomb = game::NumberBomb::new();
                        } // 炸了就自動重開

                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(msg)
                            )
                        ).await;
                    }
                    "reset_bomb" => {
                        let bomb_lock = data.get::<BombState>().unwrap().clone();
                        *bomb_lock.write().await = game::NumberBomb::new();
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(
                                    "💣 炸彈已重新放置！範圍回到 1 ~ 100"
                                )
                            )
                        ).await;
                    }
                    "game" => {
                        let resp = CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
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
                                )
                        );
                        let _ = command.create_response(&ctx.http, resp).await;
                    }
                    "add" => {
                        let todo_lock = data.get::<TodoState>().unwrap().clone();
                        let content = command.data.options[0].value.as_str().unwrap().to_string();
                        let msg = todo_lock.write().await.add_task(content);
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(msg)
                            )
                        ).await;
                    }
                    "list" => {
                        let todo_lock = data.get::<TodoState>().unwrap().clone();
                        let msg = todo_lock.read().await.list_tasks();
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(msg)
                            )
                        ).await;
                    }
                    _ => {}
                }
            }
            Interaction::Component(component) => {
                let player_val = component.data.custom_id
                    .replace("btn_", "")
                    .parse::<u32>()
                    .unwrap_or(1);
                let (result, bot_val, quote) = game::GuessGame::play(player_val);
                let title = match result {
                    game::GameResult::Win => "🎉 你贏了！",
                    game::GameResult::Lose => "💀 你輸了...",
                    game::GameResult::Draw => "🤝 平手！",
                };
                let content = format!(
                    "## {}\n你出 **{}** vs 機器人出 **{}**\n> *「{}」*",
                    title,
                    game::GuessGame::get_emoji(player_val),
                    game::GuessGame::get_emoji(bot_val),
                    quote
                );
                let _ = component.create_response(
                    &ctx.http,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(content)
                    )
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
        data.insert::<BombState>(Arc::new(RwLock::new(game::NumberBomb::new()))); // 初始化炸彈狀態
    }

    let _ = client.start().await;
}
