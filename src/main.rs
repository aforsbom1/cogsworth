mod args;
mod layer;
mod network;
mod neuron;

use crate::args::{get_arg, get_matches, get_mode, Mode};
use crate::network::Network;
use crate::sentencepiece::SentencePieceTrainer;

fn main() {
    let matches = get_matches();
    let mode = get_mode(&matches);

    match mode.0 {
        Mode::Train => {
            let num_inputs = 4; // Number of inputs to the network
            let num_outputs = 4; // Number of outputs from the network
            let num_hidden_layers = 5; // Number of hidden layers in the network
            let num_neurons_per_hidden_layer = 5; // Number of neurons in each hidden layer

            let mut network = Network::new(
                num_inputs,
                num_outputs,
                num_hidden_layers,
                num_neurons_per_hidden_layer,
            );

            let epochs = 100000;

            let inputs = [[0.0, 1.0], [1.0, 0.0], [1.0, 1.0], [0.0, 0.0]];
            let outputs: [f64; 4] = [1.0, 1.0, 1.0, 0.0];

            for i in 0..epochs {
                print!(
                    "\rTraining progress: {:.2}%",
                    (i as f64 / epochs as f64) * 100.0
                );

                for input in inputs {
                    network.feed_forward(&input);
                }
                network.back_propagate(&outputs);
            }

            print_outputs(&network, inputs);
        }
        Mode::GenerateSpm => {
            let input = get_arg(mode.1);

            let mut trainer = SentencePieceTrainer::new();

            // Set the input file path
            trainer.set_input_file(input);

            // Set the output model path
            trainer.set_model_output("./output");

            // Set other training options as desired
            // For example, you can set the size of the vocabulary:
            trainer.set_vocab_size(10000);

            // Train the SentencePiece model
            let result = trainer.train();

            // Check if the training was successful
            if let Err(e) = result {
                eprintln!("Error training SentencePiece model: {}", e);
            } else {
                println!("SentencePiece model trained successfully!");
            }
        }
        Mode::Operate => {
            println!("Stub.")
        }
        Mode::Unknown => {
            println!("Invalid parameters.")
        }
    }
}

fn print_outputs(network: &Network, inputs: [[f64; 2]; 4]) {
    for (i, output) in network.get_outputs().iter().enumerate() {
        println!(
            "\nInput: {:.0},{:.0}, Output: {:.0}",
            inputs[i][0], inputs[i][1], output
        );
    }
}
