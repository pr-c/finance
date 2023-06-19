use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders},
    Frame,
};

pub trait View {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect);
}

pub struct BooksView {}

impl BooksView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for BooksView {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let b = Block::default().title("Books").borders(Borders::ALL);
        f.render_widget(b, area);
    }
}

pub struct AccountsView {}

impl AccountsView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for AccountsView {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let b = Block::default().title("Accounts").borders(Borders::ALL);
        f.render_widget(b, area);
    }
}

pub struct CurrenciesView {}

impl CurrenciesView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for CurrenciesView {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let b = Block::default().title("Currencies").borders(Borders::ALL);
        f.render_widget(b, area);
    }
}

pub struct TransactionsView {}

impl TransactionsView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for TransactionsView {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let b = Block::default().title("Transactions").borders(Borders::ALL);
        f.render_widget(b, area);
    }
}

pub struct PostingsView {}

impl PostingsView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for PostingsView {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let b = Block::default().title("Postings").borders(Borders::ALL);
        f.render_widget(b, area);
    }
}
