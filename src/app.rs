use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    pub name: String,
    pub symbols: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct StockInfo {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub change_amount: f64,
    pub change_percent: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SortType {
    None,
    Name,
    ChangePercent,
}

pub struct App {
    pub stocks: Vec<StockInfo>,
    pub categories: Vec<Category>,
    pub active_tab: usize,
    pub should_quit: bool,
    pub sort_type: SortType,
    pub sort_descending: bool,
}

impl App {
    pub fn new(categories: Vec<Category>) -> Self {
        Self {
            stocks: Vec::new(),
            categories,
            active_tab: 0,
            should_quit: false,
            sort_type: SortType::None,
            sort_descending: true,
        }
    }
    
    pub fn toggle_sort(&mut self, new_sort: SortType) {
        if self.sort_type == new_sort {
            self.sort_descending = !self.sort_descending;
        } else {
            self.sort_type = new_sort;
            self.sort_descending = true; // Default to descending when newly selected
        }
    }

    pub fn next_tab(&mut self) {
        self.active_tab = (self.active_tab + 1) % self.categories.len();
    }

    pub fn previous_tab(&mut self) {
        if self.active_tab > 0 {
            self.active_tab -= 1;
        } else {
            self.active_tab = self.categories.len() - 1;
        }
    }

    pub fn all_symbols(&self) -> Vec<String> {
        let mut symbols = Vec::new();
        for cat in &self.categories {
            for sym in &cat.symbols {
                symbols.push(sym.clone());
            }
        }
        symbols
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
