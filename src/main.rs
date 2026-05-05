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

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("🤖 {} 猜拳大師準備就緒！", ready.user.name);

        let commands = vec![
            CreateCommand::new("add")
                .description("新增事項")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "content",
                        "任務內容"
                    ).required(true)
                ),
            CreateCommand::new("complete")
                .description("完成事項")
                .add_option(
                    CreateCommandOption::new(CommandOptionType::Integer, "id", "任務 ID").required(
                        true
                    )
                ),
            CreateCommand::new("list").description("查看清單"),
            CreateCommand::new("game").description("跟我來一場熱血的猜拳對決！")
        ];

        let _ = Command::set_global_commands(&ctx.http, commands).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Command(command) => {
                let data_lock = {
                    let data_read = ctx.data.read().await;
                    data_read.get::<TodoState>().unwrap().clone()
                };

                match command.data.name.as_str() {
                    "game" => {
                        let data = CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content("### 🖐️ 猜拳大賽開始！\n請選擇你要出的拳：")
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
                        let _ = command.create_response(&ctx.http, data).await;
                    }
                    "add" => {
                        let content = command.data.options[0].value.as_str().unwrap().to_string();
                        let msg = data_lock.write().await.add_task(content);
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(msg)
                            )
                        ).await;
                    }
                    "complete" => {
                        let id = command.data.options[0].value.as_i64().unwrap() as u32;
                        let msg = data_lock.write().await.complete_task(id);
                        let _ = command.create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new().content(msg)
                            )
                        ).await;
                    }
                    "list" => {
                        let msg = data_lock.read().await.list_tasks();
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

                let (title, color) = match result {
                    game::GameResult::Win => ("🎉 你贏了！", "🌟"),
                    game::GameResult::Lose => ("💀 你輸了...", "🌑"),
                    game::GameResult::Draw => ("🤝 平手！", "⚔️"),
                };

                let content = format!(
                    "## {}\n{} 你出了 **{}**\n{} 機器人出了 **{}**\n\n> *「{}」*",
                    title,
                    color,
                    game::GuessGame::get_emoji(player_val),
                    color,
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
    }

    let _ = client.start().await;
}
