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
            Command::new("generate_spm")
                .about("Generate sentencepiece model from .txt file")
                .arg(
                    arg!(-i --input <FILE> "Path to .txt file")
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
    GenerateSpm,
    Operate,
    Unknown,
}

pub fn get_mode(matches: &ArgMatches) -> (Mode, &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("train") {
        return (Mode::Train, matches);
    }

    if let Some(matches) = matches.subcommand_matches("generate_spm") {
        return (Mode::GenerateSpm, matches);
    }

    if let Some(matches) = matches.subcommand_matches("operate") {
        return (Mode::Operate, matches);
    }

    return (Mode::Unknown, matches);
}

pub fn get_arg(matches: &ArgMatches) -> String {
    if let Some(config_path) = matches.get_one::<PathBuf>("input") {
        return config_path.display().to_string();
    } else {
        return "None.".to_string();
    }
}
