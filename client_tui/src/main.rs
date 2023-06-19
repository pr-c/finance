use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use num_enum::IntoPrimitive;
use std::error::Error;
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal,
};
use views::{AccountsView, BooksView, CurrenciesView, PostingsView, TransactionsView, View};

mod views;

#[derive(IntoPrimitive, Clone)]
#[repr(usize)]
enum AppView {
    Books,
    Accounts,
    Currencies,
    Transactions,
    Postings,
}

impl AppView {
    fn get_as_string(&self) -> String {
        match self {
            Self::Books => "Books",
            Self::Currencies => "Currencies",
            Self::Transactions => "Transactions",
            Self::Postings => "Postings",
            Self::Accounts => "Accounts",
        }
        .to_owned()
    }
}

struct App {
    pub current_view: AppView,

    books_view: BooksView,
    accounts_view: AccountsView,
    currencies_view: CurrenciesView,
    transactions_view: TransactionsView,
    postings_view: PostingsView,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_view: AppView::Books,
            books_view: BooksView::new(),
            accounts_view: AccountsView::new(),
            currencies_view: CurrenciesView::new(),
            transactions_view: TransactionsView::new(),
            postings_view: PostingsView::new(),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);
        let block = Block::default();
        f.render_widget(block, size);
        let tab_titles = [
            AppView::Books,
            AppView::Accounts,
            AppView::Currencies,
            AppView::Transactions,
            AppView::Postings,
        ]
        .iter()
        .map(|t| Spans::from(Span::styled((*t).get_as_string(), Style::default())))
        .collect();
        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("Finance"))
            .style(Style::default())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .select(self.current_view.clone().into());
        f.render_widget(tabs, chunks[0]);

        match self.current_view {
            AppView::Books => self.books_view.draw(f, chunks[1]),
            AppView::Accounts => self.accounts_view.draw(f, chunks[1]),
            AppView::Currencies => self.currencies_view.draw(f, chunks[1]),
            AppView::Transactions => self.transactions_view.draw(f, chunks[1]),
            AppView::Postings => self.postings_view.draw(f, chunks[1]),
        };
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    run_app(&mut terminal, App::new())?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| App::draw(&mut app, f))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::F(1) => {
                    app.current_view = AppView::Books;
                }
                KeyCode::F(2) => {
                    app.current_view = AppView::Accounts;
                }
                KeyCode::F(3) => {
                    app.current_view = AppView::Currencies;
                }
                KeyCode::F(4) => {
                    app.current_view = AppView::Transactions;
                }
                KeyCode::F(5) => {
                    app.current_view = AppView::Postings;
                }
                _ => {}
            }
        }
    }
}
