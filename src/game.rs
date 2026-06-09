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
        } else if (player_choice == 1 && bot_choice == 2)
            || (player_choice == 2 && bot_choice == 3)
            || (player_choice == 3 && bot_choice == 1)
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
// 3. 卡牌結構 (共享)
// ==========================================
/// 一張牌：value 1=A, 2-10, 11=J, 12=Q, 13=K；suit 0=♠ 1=♥ 2=♦ 3=♣
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub value: u8,
    pub suit: u8,
}

impl Card {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            value: rng.gen_range(1..=13),
            suit: rng.gen_range(0..4),
        }
    }

    pub fn display(&self) -> String {
        let suits = ["♠", "♥", "♦", "♣"];
        let face = match self.value {
            1 => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            n => n.to_string(),
        };
        format!("{}{}", suits[self.suit as usize], face)
    }

    pub fn suit_color(&self) -> &'static str {
        if self.suit == 1 || self.suit == 2 {
            "#e74c3c"
        } else {
            "#2c3e50"
        }
    }

    /// 取得黑傑克中的點數
    pub fn bj_value(&self) -> u32 {
        match self.value {
            1 => 11, // 預設為 11，後續在 get_score 處理調整
            11..=13 => 10,
            n => n as u32,
        }
    }
}

// ==========================================
// 4. 黑傑克 (重構：使用 Card)
// ==========================================
#[derive(Clone)]
pub struct BlackjackCore {
    pub player_cards: Vec<Card>,
    pub bot_cards: Vec<Card>,
    pub is_game_over: bool,
}

impl BlackjackCore {
    pub fn new() -> Self {
        Self {
            player_cards: vec![Card::new_random(), Card::new_random()],
            bot_cards: vec![Card::new_random(), Card::new_random()],
            is_game_over: false,
        }
    }

    pub fn get_score(cards: &[Card]) -> u32 {
        let mut score = cards.iter().map(|c| c.bj_value()).sum::<u32>();
        let mut ace_count = cards.iter().filter(|c| c.value == 1).count();

        // 如果爆牌且手上有 A (11點)，則將 A 視為 1 點
        while score > 21 && ace_count > 0 {
            score -= 10;
            ace_count -= 1;
        }
        score
    }

    pub fn hit(&mut self) {
        if self.is_game_over {
            return;
        }
        self.player_cards.push(Card::new_random());
        if Self::get_score(&self.player_cards) > 21 {
            self.is_game_over = true;
        }
    }

    pub fn stand(&mut self) {
        if self.is_game_over {
            return;
        }
        self.is_game_over = true;
        while Self::get_score(&self.bot_cards) < 17 {
            self.bot_cards.push(Card::new_random());
        }
    }
}

// ==========================================
// 5. 老虎機
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
// 6. 其他小遊戲
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
        Self {
            chamber,
            current_pos: 0,
        }
    }
    pub fn pull_trigger(&mut self) -> bool {
        let is_bullet = self.chamber[self.current_pos];
        self.current_pos = (self.current_pos + 1) % 6;
        is_bullet
    }
}

// ==========================================
// 7. 德州撲克 (Texas Hold'em)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerRank {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl PokerRank {
    pub fn name(self) -> &'static str {
        match self {
            PokerRank::HighCard => "高牌",
            PokerRank::OnePair => "一對",
            PokerRank::TwoPair => "兩對",
            PokerRank::ThreeOfAKind => "三條",
            PokerRank::Straight => "順子",
            PokerRank::Flush => "同花",
            PokerRank::FullHouse => "葫蘆",
            PokerRank::FourOfAKind => "鐵支",
            PokerRank::StraightFlush => "同花順",
            PokerRank::RoyalFlush => "皇家同花順",
        }
    }
}

#[derive(Clone)]
pub struct TexasHoldem {
    pub player_hole: Vec<Card>,
    pub ai_hole: Vec<Card>,
    pub community: Vec<Card>,
    pub stage: TexasStage,
    pub result: Option<TexasResult>,
}

#[derive(Clone, PartialEq)]
pub enum TexasStage {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

#[derive(Clone)]
pub struct TexasResult {
    pub player_best: Vec<Card>,
    pub ai_best: Vec<Card>,
    pub player_rank: PokerRank,
    pub ai_rank: PokerRank,
    pub verdict: &'static str,
}

impl TexasHoldem {
    pub fn new() -> Self {
        let mut deck = Self::make_deck();
        Self::shuffle(&mut deck);
        let player_hole = vec![deck[0], deck[2]];
        let ai_hole = vec![deck[1], deck[3]];

        // 🌟 修正警告：這裡不需要 mut 宣告，直接拿掉
        let s = Self {
            player_hole,
            ai_hole,
            community: deck[4..9].to_vec(),
            stage: TexasStage::PreFlop,
            result: None,
        };
        s
    }

    pub fn next_stage(&mut self) {
        self.stage = match self.stage {
            TexasStage::PreFlop => TexasStage::Flop,
            TexasStage::Flop => TexasStage::Turn,
            TexasStage::Turn => TexasStage::River,
            TexasStage::River => {
                self.showdown();
                TexasStage::Showdown
            }
            TexasStage::Showdown => TexasStage::Showdown,
        };
    }

    pub fn visible_community(&self) -> &[Card] {
        match self.stage {
            TexasStage::PreFlop => &self.community[..0],
            TexasStage::Flop => &self.community[..3],
            TexasStage::Turn => &self.community[..4],
            TexasStage::River | TexasStage::Showdown => &self.community[..5],
        }
    }

    fn showdown(&mut self) {
        let (pb, pr) = Self::best_five(&self.player_hole, &self.community);
        let (ab, ar) = Self::best_five(&self.ai_hole, &self.community);
        let verdict = if pr > ar {
            "🏆 你的牌型更強，你贏了！"
        } else if pr < ar {
            "💀 AI 牌型更強，你輸了..."
        } else {
            "🤝 相同牌型，平手！"
        };
        self.result = Some(TexasResult {
            player_best: pb,
            ai_best: ab,
            player_rank: pr,
            ai_rank: ar,
            verdict,
        });
    }

    fn best_five(hole: &[Card], community: &[Card]) -> (Vec<Card>, PokerRank) {
        let all: Vec<Card> = hole.iter().chain(community.iter()).copied().collect();
        let combos = [
            [0, 1, 2, 3, 4],
            [0, 1, 2, 3, 5],
            [0, 1, 2, 3, 6],
            [0, 1, 2, 4, 5],
            [0, 1, 2, 4, 6],
            [0, 1, 2, 5, 6],
            [0, 1, 3, 4, 5],
            [0, 1, 3, 4, 6],
            [0, 1, 3, 5, 6],
            [0, 1, 4, 5, 6],
            [0, 2, 3, 4, 5],
            [0, 2, 3, 4, 6],
            [0, 2, 3, 5, 6],
            [0, 2, 4, 5, 6],
            [0, 3, 4, 5, 6],
            [1, 2, 3, 4, 5],
            [1, 2, 3, 4, 6],
            [1, 2, 3, 5, 6],
            [1, 2, 4, 5, 6],
            [1, 3, 4, 5, 6],
            [2, 3, 4, 5, 6],
        ];
        let mut best_rank = PokerRank::HighCard;
        let mut best_cards = all[0..5].to_vec();
        for idx in &combos {
            let hand: Vec<Card> = idx.iter().map(|&i| all[i]).collect();
            let rank = Self::evaluate_five(&hand);
            if rank > best_rank {
                best_rank = rank;
                best_cards = hand;
            }
        }
        (best_cards, best_rank)
    }

    pub fn evaluate_five(cards: &[Card]) -> PokerRank {
        let mut values: Vec<u8> = cards.iter().map(|c| c.value).collect();
        values.sort_unstable();
        let suits: Vec<u8> = cards.iter().map(|c| c.suit).collect();
        let is_flush = suits.iter().all(|&s| s == suits[0]);
        let is_straight = {
            let normal = values.windows(2).all(|w| w[1] == w[0] + 1);
            let ace_low = values == [1, 2, 3, 4, 5];
            let ace_high = values == [1, 10, 11, 12, 13];
            normal || ace_low || ace_high
        };
        let mut counts = [0u8; 14];
        for &v in &values {
            counts[v as usize] += 1;
        }
        let mut freqs: Vec<u8> = counts.iter().filter(|&&c| c > 0).cloned().collect();
        freqs.sort_unstable_by(|a, b| b.cmp(a));

        match (is_flush, is_straight, freqs.as_slice()) {
            (true, true, _) => {
                if values == [1, 10, 11, 12, 13] {
                    PokerRank::RoyalFlush
                } else {
                    PokerRank::StraightFlush
                }
            }
            (_, _, [4, 1]) => PokerRank::FourOfAKind,
            (_, _, [3, 2]) => PokerRank::FullHouse,
            (true, false, _) => PokerRank::Flush,
            (false, true, _) => PokerRank::Straight,
            (_, _, [3, 1, 1]) => PokerRank::ThreeOfAKind,
            (_, _, [2, 2, 1]) => PokerRank::TwoPair,
            (_, _, [2, 1, 1, 1]) => PokerRank::OnePair,
            _ => PokerRank::HighCard,
        }
    }

    fn make_deck() -> Vec<Card> {
        let mut deck = Vec::with_capacity(52);
        for suit in 0..4u8 {
            for value in 1..=13u8 {
                deck.push(Card { value, suit });
            }
        }
        deck
    }

    fn shuffle(deck: &mut Vec<Card>) {
        let mut rng = rand::thread_rng();
        for i in (1..deck.len()).rev() {
            let j = rng.gen_range(0..=i);
            deck.swap(i, j);
        }
    }
}

pub struct PokerHand;

// ==========================================
// 8. 圈圈叉叉 (TicTacToe)
// ==========================================
#[derive(Clone)]
pub struct TicTacToe {
    pub board: [u8; 9],
    pub game_over: bool,
    pub winner: u8,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [0; 9],
            game_over: false,
            winner: 0,
        }
    }

    // 🌟 新增功能：渲染 Discord 終端文字版棋盤 (支援 bot.rs 第 364 與 378 行)
    pub fn render_text(&self) -> String {
        let mut result = String::new();
        for i in 0..9 {
            let cell = match self.board[i] {
                1 => "X",
                2 => "O",
                _ => "-",
            };
            result.push_str(cell);
            result.push(' ');
            if (i + 1) % 3 == 0 && i < 8 {
                result.push('\n');
            }
        }
        result
    }

    fn check_winner(board: &[u8; 9]) -> u8 {
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        for &[a, b, c] in &LINES {
            if board[a] != 0 && board[a] == board[b] && board[b] == board[c] {
                return board[a];
            }
        }
        if board.iter().all(|&v| v != 0) {
            3
        } else {
            0
        }
    }

    fn minimax(board: &mut [u8; 9], is_ai: bool) -> i32 {
        match Self::check_winner(board) {
            2 => {
                return 10;
            }
            1 => {
                return -10;
            }
            3 => {
                return 0;
            }
            _ => {}
        }
        let mut best = if is_ai { i32::MIN } else { i32::MAX };
        for i in 0..9 {
            if board[i] == 0 {
                board[i] = if is_ai { 2 } else { 1 };
                let score = Self::minimax(board, !is_ai);
                board[i] = 0;
                best = if is_ai {
                    best.max(score)
                } else {
                    best.min(score)
                };
            }
        }
        best
    }

    pub fn ai_move(&mut self) {
        if self.game_over {
            return;
        }
        let mut best_score = i32::MIN;
        let mut best_pos = 0;
        for i in 0..9 {
            if self.board[i] == 0 {
                self.board[i] = 2;
                let score = Self::minimax(&mut self.board, false);
                self.board[i] = 0;
                if score > best_score {
                    best_score = score;
                    best_pos = i;
                }
            }
        }
        self.board[best_pos] = 2;
        self.winner = Self::check_winner(&self.board);
        if self.winner != 0 {
            self.game_over = true;
        }
    }

    pub fn player_move(&mut self, pos: usize) -> bool {
        if self.game_over || pos >= 9 || self.board[pos] != 0 {
            return false;
        }
        self.board[pos] = 1;
        self.winner = Self::check_winner(&self.board);
        if self.winner != 0 {
            self.game_over = true;
            return true;
        }
        self.ai_move();
        true
    }
}

// ==========================================
// 9. Wordle 猜字遊戲
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LetterHint {
    Correct,
    Present,
    Absent,
}

pub struct WordleGame {
    pub answer: &'static str,
    pub guesses: Vec<String>,
    pub hints: Vec<Vec<LetterHint>>,
    pub max_guesses: u8,
    pub solved: bool,
}

const WORDLE_WORDS: &[&str] = &[
    // --- 你原本的清單 ---
    "CRANE", "SLATE", "TRACE", "AUDIO", "RAISE", "STARE", "ARISE", "SNARE", "LEAST", "IRATE",
    "CRATE", "TRAIN", "TRAIL", "REALM", "PEARL", "FLAME", "BLEND", "CRISP", "DWARF", "EXPEL",
    "FROST", "GRILL", "HARSH", "INKED", "JOUST", "KNEEL", "LEMON", "MOIST", "NOBLE", "OLIVE",
    "PLUME", "QUEST", "RISKY", "SCORN", "TIGER", "UNITY", "VENOM", "WRING", "YACHT", "ZONAL",
    // --- 新增：強力開局字與高頻母音字 ---
    "ADIEU", "ROAST", "CLONE", "SHARE", "TEARS", "RELAX", "ALIEN", "GUIDE", "HOUSE", "PLANT",
    "HEART", "SMART", "STORE", "ALIVE", "BLAST",
    // --- 新增：常見常用字（補強不同子音開頭） ---
    "BEACH", "BRICK", "CHAIR", "CHIEF", "CLOCK", "CROWD", "DRIVE", "DREAM", "FAINT", "FLUTE",
    "FRONT", "GHOST", "GRAPH", "GRAPE", "GREEN", "HEAVY", "LIGHT", "MATCH", "NIGHT", "OCEAN",
    "PILOT", "PRIDE", "SHARK", "SHIRT", "SMILE", "SNAKE", "SPARK", "SPOON", "STAGE", "STORM",
    "SWEET", "THINK", "THROW", "TOWEL", "TRUCK", "VOICE", "WATCH", "WATER", "WHEAT", "WORLD",
    // --- 新增：趣味與特殊字母組合 ---
    "BLIMP", "CLERK", "FLICK", "FLOCK", "GLOVE", "JUICY", "LUNCH", "MANGO", "PIZZA", "PROUD",
    "SHAVE", "SKATE", "TRASH", "VAPOR", "WHALE",
];

impl WordleGame {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let answer = WORDLE_WORDS[rng.gen_range(0..WORDLE_WORDS.len())];
        Self {
            answer,
            guesses: Vec::new(),
            hints: Vec::new(),
            max_guesses: 6,
            solved: false,
        }
    }

    // 🌟 新增功能：把 Wordle 提示轉換成 Discord 繪文字 (支援 bot.rs 第 426 行)
    pub fn hints_to_emoji(hints: &[LetterHint]) -> String {
        hints
            .iter()
            .map(|h| match h {
                LetterHint::Correct => "🟩",
                LetterHint::Present => "🟨",
                LetterHint::Absent => "⬛",
            })
            .collect()
    }

    pub fn is_valid_word(word: &str) -> bool {
        word.len() == 5 && word.chars().all(|c| c.is_ascii_alphabetic())
    }

    pub fn guess(&mut self, word: &str) -> Option<Vec<LetterHint>> {
        if self.solved || self.guesses.len() >= (self.max_guesses as usize) {
            return None;
        }
        let word = word.to_uppercase();
        if !Self::is_valid_word(&word) {
            return None;
        }

        let answer_chars: Vec<char> = self.answer.chars().collect();
        let guess_chars: Vec<char> = word.chars().collect();
        let mut hints = vec![LetterHint::Absent; 5];
        let mut used = [false; 5];

        for i in 0..5 {
            if guess_chars[i] == answer_chars[i] {
                hints[i] = LetterHint::Correct;
                used[i] = true;
            }
        }
        for i in 0..5 {
            if hints[i] == LetterHint::Correct {
                continue;
            }
            for j in 0..5 {
                if !used[j] && guess_chars[i] == answer_chars[j] {
                    hints[i] = LetterHint::Present;
                    used[j] = true;
                    break;
                }
            }
        }

        self.guesses.push(word.clone());
        self.hints.push(hints.clone());
        if hints.iter().all(|h| *h == LetterHint::Correct) {
            self.solved = true;
        }
        Some(hints)
    }

    pub fn is_over(&self) -> bool {
        self.solved || self.guesses.len() >= (self.max_guesses as usize)
    }
    // 🌟 新增功能：渲染出像網頁遊戲一樣的封閉式精美終端盤面
    pub fn render_board_text(&self) -> String {
        let mut board = String::new();
        board.push_str("```yaml\n");
        board.push_str("┌─────────────────────────────┐\n");
        board.push_str("│       WORDLE GAME ZONE      │\n");
        board.push_str("├─────────────────────────────┤\n");

        // 顯示已經猜過的每一行
        for i in 0..self.max_guesses as usize {
            if i < self.guesses.len() {
                let word = &self.guesses[i];
                let hint = &self.hints[i];
                let mut emoji_line = String::new();
                for h in hint {
                    match h {
                        LetterHint::Correct => emoji_line.push_str("🟩"),
                        LetterHint::Present => emoji_line.push_str("🟨"),
                        LetterHint::Absent => emoji_line.push_str("⬛"),
                    }
                }
                // 格式化排列
                board.push_str(&format!(
                    "│  TRY {}:  {}   [ {} ]  │\n",
                    i + 1,
                    word,
                    emoji_line
                ));
            } else {
                // 尚未猜測的空白行
                board.push_str(&format!(
                    "│  TRY {}:  _ _ _ _ _   [ ⬜⬜⬜⬜⬜ ]  │\n",
                    i + 1
                ));
            }
        }

        board.push_str("└─────────────────────────────┘\n");
        board.push_str("```");
        board
    }
}
