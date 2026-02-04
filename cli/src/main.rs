use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "git-trainer CLI")]
#[command(about = "git-trainer CLI", long_about = None)]
// #[command(disable_help_flag = )]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Перезагрузить текущее задание
    Restart,

    /// Показать текущее задание
    Help,

    /// Отправить текущее задание
    Submit,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Restart => {
            fs::write("/etc/git-trainer/status", "1").unwrap();
        }
        Commands::Submit => {
            fs::write("/etc/git-trainer/status", "2").unwrap();
            println!("Попытка отправлена! Вы можете посмотреть оценку в менеджере попыток");
        }
        Commands::Help => {
            fs::write("/etc/git-trainer/status", "0").unwrap();
            let bytes = fs::read("/etc/git-trainer/description").unwrap();
            let description = String::from_utf8(bytes).unwrap();
            println!("{}", description);
        }
    }
}
