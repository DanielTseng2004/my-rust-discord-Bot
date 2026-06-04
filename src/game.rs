use rand::Rng;

// ==========================================
// 1. 猜拳遊戲
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameResult {
    Win,
    Lose,
    Draw,
}

pub struct GuessGame;

impl GuessGame {
    pub fn play(player_choice: u32) -> (GameResult, u32) {
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
        (result, bot_choice)
    }
}

// ==========================================
// 2. 數字炸彈 (新增：AI 猜測邏輯)
// ==========================================
#[derive(Clone)]
pub struct NumberBomb {
    pub target: u32,
    pub min: u32,
    pub max: u32,
}

pub enum BombResult {
    Invalid,
    Exploded,
    Safe,
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

    pub fn guess(&mut self, val: u32) -> BombResult {
        if val <= self.min || val >= self.max {
            return BombResult::Invalid;
        }

        if val == self.target {
            BombResult::Exploded
        } else if val < self.target {
            self.min = val;
            BombResult::Safe
        } else {
            self.max = val;
            BombResult::Safe
        }
    }

    pub fn ai_guess(&self) -> u32 {
        let mut rng = rand::thread_rng();
        if self.max - self.min <= 2 {
            self.min + 1
        } else if rng.gen_bool(0.6) {
            (self.min + self.max) / 2
        } else {
            rng.gen_range(self.min + 1..self.max)
        }
    }
}

// ==========================================
// 3. 黑傑克 (全新：單人純邏輯版)
// ==========================================
#[derive(Clone)]
pub struct BlackjackCore {
    pub player_cards: Vec<u32>,
    pub bot_cards: Vec<u32>,
    pub is_game_over: bool,
}

impl BlackjackCore {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            player_cards: vec![rng.gen_range(1..=10), rng.gen_range(1..=10)],
            bot_cards: vec![rng.gen_range(1..=10), rng.gen_range(1..=10)],
            is_game_over: false,
        }
    }

    pub fn get_score(cards: &[u32]) -> u32 {
        cards.iter().sum()
    }

    pub fn hit(&mut self) {
        let mut rng = rand::thread_rng();
        self.player_cards.push(rng.gen_range(1..=10));
        if Self::get_score(&self.player_cards) > 21 {
            self.is_game_over = true;
        }
    }

    pub fn stand(&mut self) {
        let mut rng = rand::thread_rng();
        self.is_game_over = true;
        while Self::get_score(&self.bot_cards) < 17 {
            self.bot_cards.push(rng.gen_range(1..=10));
        }
    }
}

// ==========================================
// 4. 老虎機 (全新)
// ==========================================
pub struct SlotMachine;

impl SlotMachine {
    pub fn spin() -> (Vec<&'static str>, u32) {
        let icons = ["🍎", "🍋", "🍇", "🍒", "💎", "7️⃣"];
        let mut rng = rand::thread_rng();

        let c1 = icons[rng.gen_range(0..icons.len())];
        let c2 = icons[rng.gen_range(0..icons.len())];
        let c3 = icons[rng.gen_range(0..icons.len())];

        let prize_level = if c1 == "7️⃣" && c2 == "7️⃣" && c3 == "7️⃣" {
            3 // 天降大獎
        } else if c1 == c2 && c2 == c3 {
            2 // 三連線
        } else if c1 == c2 || c2 == c3 || c1 == c3 {
            1 // 一對
        } else {
            0 // 沒中
        };

        (vec![c1, c2, c3], prize_level)
    }
}

// ==========================================
// 5. 其他既有小遊戲（不變）
// ==========================================
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

pub struct Fortune;
impl Fortune {
    pub fn draw() -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..6)
    }
}

pub struct CoinFlip;
impl CoinFlip {
    pub fn flip() -> bool {
        let mut rng = rand::thread_rng();
        rng.gen_bool(0.5)
    }
}

pub struct Roulette {
    pub chamber: [bool; 6],
    pub current_pos: usize,
}
impl Roulette {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut chamber = [false; 6];
        chamber[rng.gen_range(0..6)] = true;
        Self { chamber, current_pos: 0 }
    }
    pub fn pull_trigger(&mut self) -> bool {
        let is_bullet = self.chamber[self.current_pos];
        self.current_pos = (self.current_pos + 1) % 6;
        is_bullet
    }
}
