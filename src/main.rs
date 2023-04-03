mod args;
mod layer;
mod network;
mod neuron;

use crate::args::{get_arg, get_matches, get_mode, Mode};
use crate::network::Network;
use sentencepiece::SentencePieceProcessor;

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
        Mode::TestSpm => {
            let model = get_arg(mode.1, "model");
            let input = get_arg(mode.1, "input");

            let spp = SentencePieceProcessor::open(model).unwrap();
            let pieces = spp.encode(input.as_str()).unwrap().into_iter();

            for piece in pieces {
                println!(" id:{} token:{}", piece.id, piece.piece);
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
