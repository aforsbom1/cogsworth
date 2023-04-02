mod layer;
mod network;
mod neuron;

fn main() {
    let num_inputs = 2; // Number of inputs to the network
    let num_outputs = 1; // Number of outputs from the network
    let num_hidden_layers = 1; // Number of hidden layers in the network
    let num_neurons_per_hidden_layer = 3; // Number of neurons in each hidden layer

    network::initialize_network(
        num_inputs,
        num_outputs,
        num_hidden_layers,
        num_neurons_per_hidden_layer,
    );

    println!("Hello, world!");
}
