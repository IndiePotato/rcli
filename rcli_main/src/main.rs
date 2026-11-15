use clap::{Args, Parser, Subcommand};
use wr_rankings::{GenderParam, get_int_rankings};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct RugbyCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// See rankings for international teams
    #[command(subcommand)]
    Rankings(RankingGenders),
    /// See current standings for competitions
    #[command(subcommand)]
    Standings(CompetitionStandings),
}

#[derive(Subcommand)]
enum RankingGenders {
    /// See men's international rankings
    Mens {
        #[command(flatten)]
        rankings_args: RankingArgs,
    },
    /// See women's international rankings
    Womens {
        #[command(flatten)]
        rankings_args: RankingArgs,
    },
}

#[derive(Args, Debug)]
struct RankingArgs {
    /// The Maximum Number of Teams to Show. Defaults to 30.
    #[arg(short, long, default_value_t = 30)]
    max: u8,
}

#[derive(Subcommand)]
enum CompetitionStandings {
    Prem,
    Champ,
    Mlr,
    Top14,
    Urc,
    Jtlr,
    SuperRugby,
    Rsa,
    CurrieCup,
}

fn main() {
    let rugby_cli = RugbyCli::parse();

    match &rugby_cli.command {
        Commands::Rankings(gender) => match &gender {
            RankingGenders::Mens { rankings_args } => {
                let _ = get_int_rankings(GenderParam::MRU, rankings_args.max);
            }
            RankingGenders::Womens { rankings_args } => {
                let _ = get_int_rankings(GenderParam::WRU, rankings_args.max);
            }
        },
        Commands::Standings(_competition) => {
            todo!();
        }
    }
}
