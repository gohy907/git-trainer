use clap::{Parser, Subcommand};

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
            println!("Restarting task...");
        }
        Commands::Submit => {
            println!("Submitting task...");
            // if let Some(msg) = message {
            //     println!("Message: {}", msg);
            // }
        }
        Commands::Help => {
            println!("help")
        }
    }
}
