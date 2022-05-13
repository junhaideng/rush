mod handle;
mod time;

use std::time::Instant;

use anyhow::{Ok, Result};
use clap::Parser;
use handle::handle;
use model::constants::{Haircut, Market};

use crate::time::get_current_time;

#[derive(clap::Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    #[clap(arg_required_else_help = true)]
    Market {
        #[clap(required = true)]
        cookie: String,
        #[clap(required = false, short, long, default_value_t = String::from("1,2,3"))]
        choices: String,
        #[clap(required = false, short, long,default_value_t = String::from("true"))]
        asc: String,
        #[clap(required = false, long,default_value_t = String::from("true"))]
        always: String,
    },
    #[clap(arg_required_else_help = true)]
    Haircut {
        #[clap(required = true)]
        cookie: String,
        #[clap(required = false, short, long,default_value_t = String::from("2,3,4"))]
        choices: String,
        #[clap(required = false, short, long,default_value_t = String::from("true"))]
        asc: String,
        #[clap(required = false, long,default_value_t = String::from("true"))]
        always: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("开始预约[{}]场次", get_current_time());
    let now = Instant::now();
    match args.action {
        Action::Market {
            cookie,
            choices,
            asc,
            always,
        } => loop {
            handle::<Market>(&cookie, &choices, &asc).await?;
            if always.to_lowercase() == "false" {
                break;
            }
        },
        Action::Haircut {
            cookie,
            choices,
            asc,
            always,
        } => loop {
            handle::<Haircut>(&cookie, &choices, &asc).await?;
            if always.to_lowercase() == "false" {
                break;
            }
        },
    }

    println!("spend: {:?}", now.elapsed());
    Ok(())
}
