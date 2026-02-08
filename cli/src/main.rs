use clap::{Parser, Subcommand};
use std::io;
use std::io::Write;
use std::fs;

#[derive(Parser)]
#[command(name = "git-trainer CLI")]
#[command(about = "git-trainer CLI", long_about = None)]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Перезагрузить текущее задание
    Restart {
 #[arg(short = 'y', long = "yes")]
        yes: bool,
    },

    /// Показать формулировку текущего задания
    Task,

    /// Отправить текущее задание на проверку
    Submit,
}

fn confirm(question: &str) -> io::Result<bool> {
    loop {
        print!("{question} [Y/n]: ");
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        match line.trim().to_lowercase().as_str() {
            ""| "y" | "yes" => return Ok(true),
            _  => return Ok(false),
        }
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Restart { yes } => {
            if !yes && !confirm("Перезагрузить текущее задание? Вы потеряете весь текущий прогресс.")? {
                return Ok(());
            }
            fs::write("/etc/git-trainer/status", "1")?;
        }
        Commands::Submit => {
            fs::write("/etc/git-trainer/status", "2")?;
            println!("Попытка отправлена! Вы можете посмотреть оценку в менеджере попыток");
        }
        Commands::Task => {
            fs::write("/etc/git-trainer/status", "0")?;
            let bytes = fs::read("/etc/git-trainer/description")?;
            let description = String::from_utf8(bytes).unwrap();
            println!("{description}");
        }
    }

    Ok(())
}
