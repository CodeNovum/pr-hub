use clap::{Parser, Subcommand};
use application::{dtos::{GitRepositoryDto, PullRequestDto}, git_repositories::get_git_repositories::GitRepositoriesQuery, pull_requests::get_open_pull_requests::GetOpenPullRequestsQuery};
use infrastructure::{
    azure_devops::repository::AzureDevOpsRestRepository,
    database::{connection::init_db_connection, repositories::GitRepositoryDatabaseRepository},
    secret_storage::KeyringRepository,
};
use std::sync::Arc;
use ratatui::{widgets::{Block, Borders, List, ListItem}, Terminal};
use ratatui::backend::CrosstermBackend;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "pr-hub-cli", about = "Terminal UI for pr-hub")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List imported git repositories
    Repositories,
    /// List open pull requests across active repositories
    PullRequests,
    /// Interactive TUI listing open pull requests
    Tui,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    // Resolve application data directory using dirs crate
    let data_dir = dirs::data_dir().unwrap_or_else(|| std::env::temp_dir());
    let data_dir = format!("{}/pr-hub", data_dir.display());
    std::fs::create_dir_all(&data_dir)?;

    // init dependencies similar to Tauri
    let db_pool = init_db_connection(&data_dir).await?;
    let db_pool = Arc::new(db_pool);

    let azure_devops_repo = AzureDevOpsRestRepository::default();
    let secret_repo = KeyringRepository::new("pr-hub".to_string());
    match cli.command {
        Commands::Repositories => {
            let git_repo_repo = GitRepositoryDatabaseRepository::new(Arc::clone(&db_pool));
            let query = GitRepositoriesQuery::new(git_repo_repo);
            let repos: Vec<GitRepositoryDto> = query.execute().await?;
            for r in repos {
                println!("{} ({}) - active: {}", r.name, r.context, r.is_active);
            }
        }
        Commands::PullRequests => {
            let git_repo_repo = GitRepositoryDatabaseRepository::new(Arc::clone(&db_pool));
            let query = GetOpenPullRequestsQuery::new(
                azure_devops_repo,
                git_repo_repo,
                secret_repo,
            );
            let prs: Vec<PullRequestDto> = query.execute().await?;
            for pr in prs {
                println!("{} - {} ({})", pr.repository_name, pr.title, pr.creator_name);
            }
        }
        Commands::Tui => {
            let git_repo_repo = GitRepositoryDatabaseRepository::new(Arc::clone(&db_pool));
            let query = GetOpenPullRequestsQuery::new(
                azure_devops_repo,
                git_repo_repo,
                secret_repo,
            );
            let prs: Vec<PullRequestDto> = query.execute().await?;
            run_tui(prs)?;
        }
    }
    Ok(())
}

fn run_tui(prs: Vec<PullRequestDto>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let items: Vec<ListItem> = prs
                .iter()
                .map(|p| {
                    let text = format!("{}: {}", p.repository_name, p.title);
                    ListItem::new(text)
                })
                .collect();
            let list = List::new(items).block(Block::default().title("Open Pull Requests").borders(Borders::ALL));
            f.render_widget(list, size);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let CEvent::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
