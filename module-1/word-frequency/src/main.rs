use clap::Parser;

use word_frequency::frequency::{Count, FrequencyCounter};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[clap(short, long, default_value = "25", help = "The number of words to print")]
    num_words: usize,
    #[clap(short, long, default_value = "false", help = "Print counts as relative between 0 and 100")]
    relative: bool,
    #[clap(help = "The name of the file to read")]
    file_name: String,
}

fn main() {
    handle_file(Args::parse().file_name.as_str(), Args::parse().num_words, Args::parse().relative);
}

fn handle_file(file_name: &str, num_words: usize, relative: bool) {
    let mut frequency_counter = FrequencyCounter::from_file(file_name).unwrap_or_else(|e| print_error_and_exit(e, 2));
    frequency_counter
        .read_stop_words("stop-words.txt")
        .unwrap_or_else(|e| print_error_and_exit(e, 3));
    let word_frequencies = frequency_counter.count_frequencies();
    let sorted_frequencies = FrequencyCounter::sort_frequencies(&word_frequencies, relative);
    print_results(num_words, sorted_frequencies);
}

fn print_results(num_words: usize, sorted_frequencies: Vec<Count<'_>>) {
    for i in 0..std::cmp::min(num_words, sorted_frequencies.len()) {
        println!("{} - {}", sorted_frequencies[i].word, sorted_frequencies[i].count);
    }
}

fn print_error_and_exit(error: std::io::Error, exit_code: i32) -> ! {
    eprintln!("Error opening file: {}", error);
    std::process::exit(exit_code);
}
