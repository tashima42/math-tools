use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, name = "math-tools")]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Average {
        #[clap(required=true)]
        values: Vec<f32>,
    },
    Median {
        #[clap(required=true)]
        values: Vec<f32>,
    },
    // Mode {
    //     #[clap(required=true)]
    //     values: Vec<f32>,
    // },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Average { values } => println!("Average: {}", average(values)),
        Commands::Median { values } => println!("Median: {}", median(values)),
        // Commands::Mode { values } => println!("Mode: {}", mode(values)),
    }
}

fn average(values: Vec<f32>) -> f32 {
    let mut sum: f32 = 0.0;
    for v in values.iter() {
        sum += v
    } 
    sum / (values.len() as f32)
}

fn median(values: Vec<f32>) -> f32 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let half_point = sorted.len() / 2;
    if (sorted.len() % 2) != 0 {
        return sorted[half_point as usize]  
    }
    let first = sorted[half_point];
    let second = sorted[half_point - 1];
    average(vec![first, second])
}

// fn mode(values: Vec<f32>) -> f32 {
//     let mut sorted = values.to_vec();
//     sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
//     let mut occurrences: Vec<f32>;
//     for v in values.iter() {
//     
//     }

//     println!("{:?}", m);
//     1.0
// }

