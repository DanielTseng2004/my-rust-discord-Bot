use rand::Rng;

// --- 原有的猜拳邏輯 ---
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
        let result = if player_choice == bot_choice {
            GameResult::Draw
        } else if
            (player_choice == 1 && bot_choice == 2) ||
            (player_choice == 2 && bot_choice == 3) ||
            (player_choice == 3 && bot_choice == 1)
        {
            GameResult::Win
        } else {
            GameResult::Lose
        };
        let remarks = match result {
            GameResult::Win => "可惡... 被你看穿了！",
            GameResult::Lose => "哼哼，這就是預讀的藝術。",
            GameResult::Draw => "竟然跟我出的一模一樣？",
        };
        (result, bot_choice, remarks)
    }
    pub fn get_emoji(choice: u32) -> &'static str {
        match choice {
            1 => "✊ 石頭",
            2 => "✌️ 剪刀",
            3 => "✋ 布",
            _ => "❓ 未知",
        }
    }
}

// --- 新增：數字炸彈邏輯 ---
pub struct NumberBomb {
    pub target: u32,
    pub min: u32,
    pub max: u32,
}

impl NumberBomb {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            target: rng.gen_range(2..99), // 炸彈在 2-98 之間
            min: 1,
            max: 100,
        }
    }

    pub fn guess(&mut self, val: u32) -> (String, bool) {
        if val <= self.min || val >= self.max {
            return (
                format!("⚠️ 出錯了！請輸入 **{}** 到 **{}** 之間的數字", self.min, self.max),
                false,
            );
        }

        if val == self.target {
            (format!("💥 **BOMMMM!!** 數字就是 **{}**！你踩到炸彈了！", val), true)
        } else if val < self.target {
            self.min = val;
            (format!("✅ 安全通過！目前的範圍縮小為：**{} ~ {}**", self.min, self.max), false)
        } else {
            self.max = val;
            (format!("✅ 安全通過！目前的範圍縮小為：**{} ~ {}**", self.min, self.max), false)
        }
    }
}
