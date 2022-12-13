use anyhow::Result;
use arandu::get_ai_completion;
use clap::Parser;
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

/// Arandu CLI - Get code suggestions using OpenAI models
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Prompt that will be processed by the AI model
    #[arg(short, long)]
    prompt: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut sp = Spinner::new(
        Spinners::Dots12,
        "The AI 🤖 is working on a response...".into(),
    );

    let suggestion_response = get_ai_completion(args.prompt)?;
    let suggestion_response_text = &suggestion_response
        .choices
        .get(0)
        .expect("Failed while indexing suggestion response")
        .text;

    sp.stop_with_newline();
    println!("The response is: {suggestion_response_text:?}");
    Ok(())
}
