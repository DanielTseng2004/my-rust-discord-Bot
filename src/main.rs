mod game;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::application::{ Interaction, ComponentType };
use serenity::builder::{
    CreateMessage,
    CreateButton,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::prelude::*;
use std::sync::Arc;

// --- 簡約的 Todo 狀態 (就像遊戲存檔) ---
struct TodoApp {
    tasks: Vec<String>,
}

struct TodoState;
impl TypeMapKey for TodoState {
    type Value = Arc<RwLock<TodoApp>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // 取得全域狀態
        let data_lock = {
            let data_read = ctx.data.read().await;
            data_read.get::<TodoState>().unwrap().clone()
        };

        if msg.content.starts_with("!add ") {
            let task = msg.content.replace("!add ", "");
            let mut app = data_lock.write().await;
            app.tasks.push(task);
            let _ = msg.reply(&ctx.http, "✅ 已加入清單！").await;
        }

        if msg.content == "!list" {
            let app = data_lock.read().await;
            let content = if app.tasks.is_empty() { "空的".into() } else { app.tasks.join("\n") };
            let _ = msg.channel_id.say(&ctx.http, format!("📋 你的 Todo：\n{}", content)).await;
        }

        // 遊戲啟動指令 (發送按鈕)
        if msg.content == "!game" {
            let builder = CreateMessage::new()
                .content("🎲 猜拳比大小遊戲！")
                .button(CreateButton::new("btn_1").label("數字 1"))
                .button(CreateButton::new("btn_2").label("數字 2"));

            let _ = msg.channel_id.send_message(&ctx.http, builder).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Some(component) = interaction.as_message_component() {
            // 1. 解析玩家點了哪個按鈕
            let choice = match component.data.custom_id.as_str() {
                "btn_1" => 1,
                "btn_2" => 2,
                _ => 0,
            };

            // 2. 調用 game.rs 裡的 play 邏輯 (這會消除警告！)
            let (result, bot_num) = game::GuessGame::play(choice);

            let response_text = match result {
                game::GameResult::Win => format!("🏆 你贏了！我出 {}，你出 {}", bot_num, choice),
                game::GameResult::Lose => format!("💀 你輸了... 我出 {}，你出 {}", bot_num, choice),
                game::GameResult::Draw => format!("🤝 平手！都是 {}", bot_num),
            };

            let data = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(response_text)
            );
            let _ = component.create_response(&ctx.http, data).await;
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("找不到 TOKEN 環境變數");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler).await
        .expect("建立失敗");

    // 初始化狀態
    {
        let mut data = client.data.write().await;
        data.insert::<TodoState>(Arc::new(RwLock::new(TodoApp { tasks: vec![] })));
    }

    println!("🤖 Bot 已啟動！");
    client.start().await.unwrap();
}
