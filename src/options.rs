use clap::Parser;

#[derive(Debug, Parser)]
#[command(about)]
pub struct Options {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)] 
    pub filter: String, 
    #[arg(short, long)] 
    pub output: String
}