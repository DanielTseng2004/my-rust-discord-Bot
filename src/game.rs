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

// ==========================================
// 6. 德州撲克（Texas Hold'em，無下注比牌版）
//    流程：荷官發底牌(2) → Flop(3) → Turn(1) → River(1)
//    各自從 2張底牌 + 5張公牌 中選最佳5張比大小
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
            PokerRank::HighCard       => "高牌",
            PokerRank::OnePair        => "一對",
            PokerRank::TwoPair        => "兩對",
            PokerRank::ThreeOfAKind   => "三條",
            PokerRank::Straight       => "順子",
            PokerRank::Flush          => "同花",
            PokerRank::FullHouse      => "葫蘆",
            PokerRank::FourOfAKind    => "四條",
            PokerRank::StraightFlush  => "同花順",
            PokerRank::RoyalFlush     => "皇家同花順",
        }
    }
}

/// 一張牌：value 1=A, 2-10, 11=J, 12=Q, 13=K；suit 0=♠ 1=♥ 2=♦ 3=♣
#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub value: u8,
    pub suit: u8,
}

impl Card {
    pub fn display(self) -> String {
        let suits = ["♠", "♥", "♦", "♣"];
        let face = match self.value {
            1  => "A".to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            n  => n.to_string(),
        };
        format!("{}{}", suits[self.suit as usize], face)
    }
}

/// 德州撲克牌局狀態（無下注，純比牌型）
#[derive(Clone)]
pub struct TexasHoldem {
    pub player_hole: Vec<Card>,   // 玩家底牌 2張
    pub ai_hole:     Vec<Card>,   // AI 底牌 2張
    pub community:   Vec<Card>,   // 公共牌，最多 5 張
    pub stage: TexasStage,
    pub result: Option<TexasResult>,
}

#[derive(Clone, PartialEq)]
pub enum TexasStage {
    PreFlop,  // 剛發底牌，等待翻牌
    Flop,     // 已翻 3 張公牌
    Turn,     // 已翻 4 張公牌
    River,    // 已翻 5 張公牌，可開牌比較
    Showdown, // 已結算
}

#[derive(Clone)]
pub struct TexasResult {
    pub player_best: Vec<Card>,
    pub ai_best:     Vec<Card>,
    pub player_rank: PokerRank,
    pub ai_rank:     PokerRank,
    pub verdict:     &'static str,
}

impl TexasHoldem {
    pub fn new() -> Self {
        let mut deck = Self::make_deck();
        Self::shuffle(&mut deck);
        // 交替發底牌：玩家1、AI1、玩家2、AI2
        let player_hole = vec![deck[0], deck[2]];
        let ai_hole     = vec![deck[1], deck[3]];
        // 剩餘牌從 index 4 開始備用（燒牌略過）
        let mut s = Self {
            player_hole,
            ai_hole,
            community: Vec::new(),
            stage: TexasStage::PreFlop,
            result: None,
        };
        // 預先把整副牌存起來以便後續發公牌
        // 為簡化，直接把 deck[4..9] 留著作為公牌來源
        s.community = deck[4..9].to_vec(); // 暫存，實際按 stage 揭露
        s
    }

    /// 推進到下一個 stage，回傳目前已揭露的公牌數
    pub fn next_stage(&mut self) {
        self.stage = match self.stage {
            TexasStage::PreFlop  => TexasStage::Flop,
            TexasStage::Flop     => TexasStage::Turn,
            TexasStage::Turn     => TexasStage::River,
            TexasStage::River    => {
                self.showdown();
                TexasStage::Showdown
            }
            TexasStage::Showdown => TexasStage::Showdown,
        };
    }

    /// 目前應顯示幾張公牌
    pub fn visible_community(&self) -> &[Card] {
        match self.stage {
            TexasStage::PreFlop  => &self.community[..0],
            TexasStage::Flop     => &self.community[..3],
            TexasStage::Turn     => &self.community[..4],
            TexasStage::River | TexasStage::Showdown => &self.community[..5],
        }
    }

    /// 結算：從 7 張（2 底牌 + 5 公牌）中選最佳 5 張
    fn showdown(&mut self) {
        let (pb, pr) = Self::best_five(&self.player_hole, &self.community);
        let (ab, ar) = Self::best_five(&self.ai_hole,     &self.community);
        let verdict = if pr > ar {
            "🏆 你的牌型更強，你贏了！"
        } else if pr < ar {
            "💀 AI 牌型更強，你輸了..."
        } else {
            "🤝 相同牌型，平手！"
        };
        self.result = Some(TexasResult {
            player_best: pb,
            ai_best:     ab,
            player_rank: pr,
            ai_rank:     ar,
            verdict,
        });
    }

    /// 從 hole(2) + community(5) 的 7 張中，窮舉 C(7,5)=21 種組合，取最高牌型
    fn best_five(hole: &[Card], community: &[Card]) -> (Vec<Card>, PokerRank) {
        let all: Vec<Card> = hole.iter().chain(community.iter()).copied().collect();
        // C(7,5) 的所有索引組合
        let combos = [
            [0,1,2,3,4],[0,1,2,3,5],[0,1,2,3,6],
            [0,1,2,4,5],[0,1,2,4,6],[0,1,2,5,6],
            [0,1,3,4,5],[0,1,3,4,6],[0,1,3,5,6],
            [0,1,4,5,6],[0,2,3,4,5],[0,2,3,4,6],
            [0,2,3,5,6],[0,2,4,5,6],[0,3,4,5,6],
            [1,2,3,4,5],[1,2,3,4,6],[1,2,3,5,6],
            [1,2,4,5,6],[1,3,4,5,6],[2,3,4,5,6],
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

    /// 評估恰好 5 張牌的牌型
    pub fn evaluate_five(cards: &[Card]) -> PokerRank {
        let mut values: Vec<u8> = cards.iter().map(|c| c.value).collect();
        values.sort_unstable();
        let suits: Vec<u8> = cards.iter().map(|c| c.suit).collect();
        let is_flush = suits.iter().all(|&s| s == suits[0]);
        let is_straight = {
            let normal   = values.windows(2).all(|w| w[1] == w[0] + 1);
            let ace_low  = values == [1, 2, 3, 4, 5];
            let ace_high = values == [1, 10, 11, 12, 13];
            normal || ace_low || ace_high
        };
        let mut counts = [0u8; 14];
        for &v in &values { counts[v as usize] += 1; }
        let mut freqs: Vec<u8> = counts.iter().filter(|&&c| c > 0).cloned().collect();
        freqs.sort_unstable_by(|a, b| b.cmp(a));

        match (is_flush, is_straight, freqs.as_slice()) {
            (true, true, _)       => {
                if values == [1, 10, 11, 12, 13] { PokerRank::RoyalFlush }
                else { PokerRank::StraightFlush }
            }
            (_, _, [4, 1])        => PokerRank::FourOfAKind,
            (_, _, [3, 2])        => PokerRank::FullHouse,
            (true, false, _)      => PokerRank::Flush,
            (false, true, _)      => PokerRank::Straight,
            (_, _, [3, 1, 1])     => PokerRank::ThreeOfAKind,
            (_, _, [2, 2, 1])     => PokerRank::TwoPair,
            (_, _, [2, 1, 1, 1])  => PokerRank::OnePair,
            _                     => PokerRank::HighCard,
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

// 保留舊名稱作為別名，讓 bot.rs 的 PokerHand 引用不需改動
pub struct PokerHand;

// ==========================================
// 7. 圈圈叉叉 (AI: minimax)
// ==========================================
/// 棋盤格：0=空, 1=玩家(X), 2=AI(O)
#[derive(Clone)]
pub struct TicTacToe {
    pub board: [u8; 9],
    pub game_over: bool,
    pub winner: u8,  // 0=無, 1=玩家, 2=AI, 3=平手
}

impl TicTacToe {
    pub fn new() -> Self {
        Self { board: [0; 9], game_over: false, winner: 0 }
    }

    fn check_winner(board: &[u8; 9]) -> u8 {
        const LINES: [[usize; 3]; 8] = [
            [0,1,2],[3,4,5],[6,7,8], // 橫
            [0,3,6],[1,4,7],[2,5,8], // 直
            [0,4,8],[2,4,6],          // 斜
        ];
        for &[a,b,c] in &LINES {
            if board[a] != 0 && board[a] == board[b] && board[b] == board[c] {
                return board[a];
            }
        }
        if board.iter().all(|&v| v != 0) { 3 } else { 0 }
    }

    fn minimax(board: &mut [u8; 9], is_ai: bool) -> i32 {
        match Self::check_winner(board) {
            2 => return  10,
            1 => return -10,
            3 => return   0,
            _ => {}
        }
        let mut best = if is_ai { i32::MIN } else { i32::MAX };
        for i in 0..9 {
            if board[i] == 0 {
                board[i] = if is_ai { 2 } else { 1 };
                let score = Self::minimax(board, !is_ai);
                board[i] = 0;
                best = if is_ai { best.max(score) } else { best.min(score) };
            }
        }
        best
    }

    pub fn ai_move(&mut self) {
        if self.game_over { return; }
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
        if self.winner != 0 { self.game_over = true; }
    }

    /// 玩家下棋，成功後自動觸發 AI，回傳是否合法
    pub fn player_move(&mut self, pos: usize) -> bool {
        if self.game_over || pos >= 9 || self.board[pos] != 0 { return false; }
        self.board[pos] = 1;
        self.winner = Self::check_winner(&self.board);
        if self.winner != 0 {
            self.game_over = true;
            return true;
        }
        self.ai_move();
        true
    }

    pub fn render_text(&self) -> String {
        let symbols = ["·", "X", "O"];
        let mut rows = Vec::new();
        for row in 0..3 {
            let r: Vec<&str> = (0..3).map(|col| symbols[self.board[row * 3 + col] as usize]).collect();
            rows.push(r.join(" | "));
        }
        rows.join("\n---------\n")
    }
}

// ==========================================
// 8. Wordle 猜字遊戲
// ==========================================
/// 每個字母的狀態
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LetterHint {
    Correct,  // 🟩 正確位置
    Present,  // 🟨 字母存在但位置錯
    Absent,   // ⬛ 字母不存在
}

pub struct WordleGame {
    pub answer: &'static str,
    pub guesses: Vec<String>,
    pub hints: Vec<Vec<LetterHint>>,
    pub max_guesses: u8,
    pub solved: bool,
}

const WORDLE_WORDS: &[&str] = &[
    "CRANE", "SLATE", "TRACE", "AUDIO", "RAISE",
    "STARE", "ARISE", "SNARE", "LEAST", "IRATE",
    "CRATE", "TRAIN", "TRAIL", "REALM", "PEARL",
    "FLAME", "BLEND", "CRISP", "DWARF", "EXPEL",
    "FROST", "GRILL", "HARSH", "INKED", "JOUST",
    "KNEEL", "LEMON", "MOIST", "NOBLE", "OLIVE",
    "PLUME", "QUEST", "RISKY", "SCORN", "TIGER",
    "UNITY", "VENOM", "WRING", "YACHT", "ZONAL",
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

    pub fn is_valid_word(word: &str) -> bool {
        word.len() == 5 && word.chars().all(|c| c.is_ascii_alphabetic())
    }

    /// 回傳 None = 非法輸入, Some(hints) = 本次結果
    pub fn guess(&mut self, word: &str) -> Option<Vec<LetterHint>> {
        if self.solved || self.guesses.len() >= self.max_guesses as usize { return None; }
        let word = word.to_uppercase();
        if !Self::is_valid_word(&word) { return None; }

        let answer_chars: Vec<char> = self.answer.chars().collect();
        let guess_chars: Vec<char>  = word.chars().collect();
        let mut hints = vec![LetterHint::Absent; 5];
        let mut used = [false; 5];

        // 第一輪：找完全正確
        for i in 0..5 {
            if guess_chars[i] == answer_chars[i] {
                hints[i] = LetterHint::Correct;
                used[i] = true;
            }
        }
        // 第二輪：找位置錯誤
        for i in 0..5 {
            if hints[i] == LetterHint::Correct { continue; }
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

    pub fn hints_to_emoji(hints: &[LetterHint]) -> String {
        hints.iter().map(|h| match h {
            LetterHint::Correct => "🟩",
            LetterHint::Present => "🟨",
            LetterHint::Absent  => "⬛",
        }).collect()
    }

    pub fn is_over(&self) -> bool {
        self.solved || self.guesses.len() >= self.max_guesses as usize
    }
}
