// src/todo.rs
use std::io;

pub struct TodoItem {
    pub id: u32,
    pub content: String,
    pub completed: bool,
}

pub struct TodoList {
    pub items: Vec<TodoItem>,
    next_id: u32,
}

impl TodoList {
    // 就像 Java 的建構子
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self) {
        println!("輸入任務內容：");
        let mut content = String::new();
        io::stdin().read_line(&mut content).expect("讀取失敗");

        let item = TodoItem {
            id: self.next_id,
            content: content.trim().to_string(),
            completed: false,
        };
        self.items.push(item);
        self.next_id += 1;
        println!("✅ 已新增！");
    }

    pub fn complete_task(&mut self) {
        println!("請輸入要完成的任務 ID：");
        let mut id_str = String::new();
        io::stdin().read_line(&mut id_str).expect("讀取失敗");

        if let Ok(target_id) = id_str.trim().parse::<u32>() {
            match self.items.iter_mut().find(|item| item.id == target_id) {
                Some(item) => {
                    item.completed = true;
                    println!("🎯 任務「{}」標記為完成！", item.content);
                }
                None => println!("❌ 找不到該 ID 的任務"),
            }
        }
    }

    pub fn list_tasks(&self) {
        println!("\n--- 目前清單 ---");
        for item in &self.items {
            let status = if item.completed { "[V]" } else { "[ ]" };
            println!("{} ID:{} - {}", status, item.id, item.content);
        }
    }
}
