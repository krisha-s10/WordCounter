use clap::{Parser, ValueEnum};
use std::fs;
use std::io;

//run like cargo run -- file.txt --count type

#[derive(Parser)]
#[command(name = "Word Counter")]
#[command(about = "Counts, words, characters, sentences, lines or paragraphs in a text file.")]

struct Args {
    file: String,
    #[arg(short, long, value_enum)]
    count: CountType,
}

#[derive(Clone, ValueEnum, Debug)]
enum CountType {
    Word,
    Char,
    Sentences,
    Paragraph,
    Line,
}

fn main() {
    let args = Args::parse();
    // println!("Enter a .txt file path: ");
    // let mut file_path = String::new();
    // io::stdin().read_line(&mut file_path).unwrap();
    let file_content = fs::read_to_string(&args.file).expect("Could not read file");
    let result = count(&args.count, &file_content);
    println!("{:?} count = {}", args.count, result);

    // loop {
    //     println!("What do you want to be counted? word/sentence/char/line/paragraph/quit");
    //     let mut to_count = String::new();
    //     io::stdin()
    //         .read_line(&mut to_count)
    //         .expect("Failed to read line");
    //     let to_count = to_count.trim().to_lowercase();
    //
    //     match to_count.trim().to_lowercase().as_str() {
    //         "word" | "sentence" | "char" | "line" | "paragraph" => {
    //             println!("{} count = {:?}", to_count, count(&to_count, &file_content))
    //         }
    //         "quit" => break,
    //         _ => println!("Could not be understood"),
    //     }
    // }
}

fn count(to_count: &CountType, text: &str) -> usize {
    match to_count {
        CountType::Word => text.split_whitespace().count(),
        CountType::Char => text.chars().count(),
        CountType::Sentences => count_sentences(text),
        CountType::Line => text.lines().count(),
        CountType::Paragraph => text.split("\n\n").count(),
    }
}

fn count_sentences(text: &str) -> usize {
    let mut counter = 0;
    for c in text.chars() {
        if c == '.' || c == '!' || c == '?' {
            counter += 1;
        };
    }
    counter
}
