// src/game.rs
use rand::Rng;

pub enum GameResult {
    Win,
    Lose,
    Draw,
}

pub struct GuessGame;

impl GuessGame {
    // 遊戲邏輯：比大小
    pub fn play(player_choice: u32) -> (GameResult, u32) {
        let mut rng = rand::thread_rng();
        let bot_choice = rng.gen_range(1..=3);

        if player_choice == bot_choice {
            (GameResult::Draw, bot_choice)
        } else if player_choice > bot_choice {
            (GameResult::Win, bot_choice)
        } else {
            (GameResult::Lose, bot_choice)
        }
    }
}
