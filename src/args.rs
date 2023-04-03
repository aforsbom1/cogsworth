use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use std::path::PathBuf;

pub fn get_matches() -> ArgMatches {
    command!() // requires `cargo` feature
        .subcommand(
            Command::new("train")
                .about("Trains the neural network on input with expected output")
                .arg(
                    arg!(-i --input <FILE> "Path to training input file")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("test_spm")
                .about("Generate sentencepiece model from .txt file")
                .arg(
                    arg!(-i --input <FILE> "test input string")
                        .value_parser(value_parser!(PathBuf)),
                )
                .arg(
                    arg!(-m --model <FILE> "Path to .model file")
                        .value_parser(value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("operate")
                .about("Operational mode. Use this to generate answers based on trained model.")
                .arg(
                    arg!(-i --input "Word, sentences, questions, go ahead! Ask!")
                        .action(ArgAction::SetTrue),
                ),
        )
        .get_matches()
}

pub enum Mode {
    Train,
    TestSpm,
    Operate,
    Unknown,
}

pub fn get_mode(matches: &ArgMatches) -> (Mode, &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("train") {
        return (Mode::Train, matches);
    }

    if let Some(matches) = matches.subcommand_matches("test_spm") {
        return (Mode::TestSpm, matches);
    }

    if let Some(matches) = matches.subcommand_matches("operate") {
        return (Mode::Operate, matches);
    }

    return (Mode::Unknown, matches);
}

pub fn get_arg(matches: &ArgMatches, input_name: &str) -> String {
    if let Some(config_path) = matches.get_one::<PathBuf>(input_name) {
        return config_path.display().to_string();
    } else {
        return "None.".to_string();
    }
}
