// src/game.rs
use rand::Rng;

pub enum GameResult {
    Win,
    Lose,
    Draw,
}

pub struct GuessGame;

impl GuessGame {
    pub fn play(player_choice: u32) -> (GameResult, u32, &'static str) {
        let mut rng = rand::thread_rng();
        let bot_choice = rng.gen_range(1..=3);

        // 判定勝負邏輯
        let result = if player_choice == bot_choice {
            GameResult::Draw
        } else if
            (player_choice == 1 && bot_choice == 2) || // 石頭勝剪刀
            (player_choice == 2 && bot_choice == 3) || // 剪刀勝布
            (player_choice == 3 && bot_choice == 1) // 布勝石頭
        {
            GameResult::Win
        } else {
            GameResult::Lose
        };

        let remarks = match result {
            GameResult::Win => "可惡... 被你看穿了我的出拳規律！",
            GameResult::Lose => "哼哼，這就是預讀的藝術。",
            GameResult::Draw => "竟然跟我出的一模一樣？你是我的複製人嗎？",
        };

        (result, bot_choice, remarks)
    }

    // 輔助函式：將數字轉為圖示
    pub fn get_emoji(choice: u32) -> &'static str {
        match choice {
            1 => "✊ 石頭",
            2 => "✌️ 剪刀",
            3 => "✋ 布",
            _ => "❓ 未知",
        }
    }
}
