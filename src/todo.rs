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
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }

    // 改寫：接收外部傳入的內容，而不是從 stdin 讀取
    pub fn add_task(&mut self, content: String) -> String {
        let item = TodoItem {
            id: self.next_id,
            content: content.clone(),
            completed: false,
        };
        self.items.push(item);
        self.next_id += 1;
        format!("✅ 已新增任務：**{}**", content)
    }

    // 改寫：接收 ID 參數
    pub fn complete_task(&mut self, target_id: u32) -> String {
        match self.items.iter_mut().find(|item| item.id == target_id) {
            Some(item) => {
                item.completed = true;
                format!("🎯 任務「{}」標記為完成！", item.content)
            }
            None => format!("❌ 找不到 ID 為 {} 的任務", target_id),
        }
    }

    // 改寫：回傳格式化後的字串供 Discord 顯示
    pub fn list_tasks(&self) -> String {
        if self.items.is_empty() {
            return "📋 清單是空的，放假囉！".into();
        }
        let list = self.items
            .iter()
            .map(|item| {
                let status = if item.completed { "[V]" } else { "[ ]" };
                format!("{} ID:{} - {}", status, item.id, item.content)
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!("📋 **你的待辦清單：**\n{}", list)
    }
}
