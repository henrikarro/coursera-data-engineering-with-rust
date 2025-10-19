use clap::Parser;

use word_frequency::frequency::FrequencyCounter;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[clap(short, long, default_value = "25", help = "The number of words to print")]
    num_words: usize,
    #[clap(help = "The name of the file to read")]
    file_name: String,
}

fn main() {
    handle_file(Args::parse().file_name.as_str(), Args::parse().num_words);
}

fn handle_file(file_name: &str, num_words: usize) {
    let mut frequency_counter = FrequencyCounter::from_file(file_name).unwrap_or_else(|e| print_error_and_exit(e, 2));
    frequency_counter
        .read_stop_words("stop-words.txt")
        .unwrap_or_else(|e| print_error_and_exit(e, 3));
    let word_frequencies = frequency_counter.count_frequencies();
    let sorted_frequencies = FrequencyCounter::sort_frequencies(&word_frequencies);
    for i in 0..std::cmp::min(num_words, sorted_frequencies.len()) {
        println!("{} - {}", sorted_frequencies[i].word, sorted_frequencies[i].count);
    }
}

fn print_error_and_exit(error: std::io::Error, exit_code: i32) -> ! {
    eprintln!("Error opening file: {}", error);
    std::process::exit(exit_code);
}
