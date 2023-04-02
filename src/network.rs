use crate::layer::Layer;

pub struct Network {
    pub layers: Vec<Layer>,
}

pub fn initialize_network(
    num_inputs: usize,
    num_outputs: usize,
    num_hidden_layers: usize,
    num_neurons_per_hidden_layer: usize,
) -> Network {
    let mut layers: Vec<Layer> = Vec::with_capacity(num_hidden_layers + 1);

    // Create input layer
    layers.push(Layer(
        num_inputs,
        num_neurons_per_hidden_layer,
    ));

    // Create hidden layers
    for _ in 1..num_hidden_layers {
        layers.push(Layer(
            num_neurons_per_hidden_layer,
            num_neurons_per_hidden_layer,
        ));
    }

    // Create output layer
    layers.push(Layer(
        num_neurons_per_hidden_layer,
        num_outputs,
    ));

    Network { layers }
}
