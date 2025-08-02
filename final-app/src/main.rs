use clap::{Parser, Subcommand};

/// CLI tool with greeting and sum functionality
#[derive(Parser, Debug)]
#[command(name = "MyApp", version, about = "Greet or Add numbers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Greet a person
    Greet {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, default_value_t = 1)]
        count: u64,
    },
    /// Add two numbers
    Sum {
        #[arg(long)]
        num1: u128,

        #[arg(long)]
        num2: u128,
    },
    Multiply {
        #[arg(long)]
        num1: u128,

        #[arg(long)]
        num2: u128,
    },
    Divide {
        #[arg(long)]
        num1: u128,

        #[arg(long)]
        num2: u128,
    },
    Subtract {
        #[arg(long)]
        num1: u128,

        #[arg(long)]
        num2: u128,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Greet { name, count } => {
            for _ in 0..count {
                println!("Hello {}!", name);
            }
        }
        Commands::Sum { num1, num2 } => {
            println!("Sum: {}", num1 + num2);
        }
        Commands::Multiply { num1, num2 } => {
            println!("Product: {}", num1 * num2);
        }
        Commands::Divide { num1, num2 } => {
            if num2 == 0 {
                println!("Error: Division by zero is not allowed.");
            } else {
                println!("Quotient: {}", num1 / num2);
            }
        }
        Commands::Subtract { num1, num2 } => {
            println!("Difference: {}", num1 - num2);
        }
    }
}
