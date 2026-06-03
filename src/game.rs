use rand::Rng;

// --- 原有的猜拳邏輯 ---
#[derive(Debug, Clone, Copy)]
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
            GameResult::Win => {
                let quotes = ["可惡... 被你看穿了！", "算你厲害，下次不會輸了！", "竟然贏過了我這台超級電腦？"];
                quotes[rng.gen_range(0..quotes.len())]
            },
            GameResult::Lose => {
                let quotes = ["哼哼，這就是預讀的藝術。", "你還太嫩了，回去多練練吧！", "勝利女神似乎站在我這邊呢。"];
                quotes[rng.gen_range(0..quotes.len())]
            },
            GameResult::Draw => {
                let quotes = ["竟然跟我出的一模一樣？", "心有靈犀一點通？", "這就是平局的浪漫。"];
                quotes[rng.gen_range(0..quotes.len())]
            },
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

// --- 數字炸彈邏輯 ---
pub struct NumberBomb {
    pub target: u32,
    pub min: u32,
    pub max: u32,
}

impl NumberBomb {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            target: rng.gen_range(2..99),
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

// --- 擲骰子邏輯 ---
pub struct Dice;

impl Dice {
    pub fn roll(sides: u32, count: u32) -> (Vec<u32>, u32) {
        let mut rng = rand::thread_rng();
        let mut rolls = Vec::new();
        let mut total = 0;
        for _ in 0..count {
            let val = rng.gen_range(1..=sides);
            rolls.push(val);
            total += val;
        }
        (rolls, total)
    }
}

// --- 幸運占卜邏輯 ---
pub struct Fortune;

impl Fortune {
    pub fn draw() -> (&'static str, &'static str, &'static str) {
        let fortunes = [
            ("大吉", "🌟", "今天運氣爆棚！適合嘗試新事物。"),
            ("中吉", "✨", "平凡中帶點小驚喜，是個不錯的一天。"),
            ("小吉", "🍀", "踏實做事，幸運自然會降臨。"),
            ("末吉", "💠", "保持平常心，小心駛得萬年船。"),
            ("凶", "☁️", "今天適合宅在家裡，多喝熱水。"),
            ("大凶", "💀", "沒關係，最壞的情況已經過去了，明天會更好！"),
        ];
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..fortunes.len());
        fortunes[idx]
    }
}

// --- 新增：硬幣翻轉邏輯 ---
pub struct CoinFlip;

impl CoinFlip {
    pub fn flip() -> (&'static str, &'static str) {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            ("正面", "🟡")
        } else {
            ("反面", "⚪")
        }
    }
}

// --- 新增：俄羅斯輪盤邏輯 ---
pub struct Roulette {
    pub chamber: [bool; 6],
    pub current_pos: usize,
}

impl Roulette {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut chamber = [false; 6];
        chamber[rng.gen_range(0..6)] = true; // 隨機放入一顆子彈
        Self {
            chamber,
            current_pos: 0,
        }
    }

    pub fn pull_trigger(&mut self) -> (bool, String) {
        let is_bullet = self.chamber[self.current_pos];
        self.current_pos = (self.current_pos + 1) % 6;
        
        if is_bullet {
            (true, "💥 **BANG!** 你被擊中了... 遊戲結束。".to_string())
        } else {
            let remaining = 6 - self.current_pos;
            (false, format!("🔒 *咔噠*... 又是安全的一發。剩餘次數：**{}**", remaining))
        }
    }
}
