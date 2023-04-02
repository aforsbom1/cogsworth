use crate::layer::Layer;

pub struct Network {
    pub layers: Vec<Layer>,
}

impl Network {
    pub fn new (num_inputs: usize,
        num_outputs: usize,
        num_hidden_layers: usize,
        num_neurons_per_hidden_layer: usize) -> Network {
            let mut layers: Vec<Layer> = Vec::with_capacity(num_hidden_layers + 1);

            // Create input layer
            layers.push(Layer::new(
                num_neurons_per_hidden_layer,
                num_inputs
            ));
            // Create hidden layers
            for _ in 1..num_hidden_layers {
                layers.push(Layer::new(
                    num_neurons_per_hidden_layer,
                    num_inputs
                ));
            }
            // Create output layer
            layers.push(Layer::new(
                num_outputs,
                num_inputs
            ));
            Network { layers }
        }

    pub fn feed_forward(&mut self, inputs: &[f64]) {
        // Set the inputs of the first layer
        self.layers[0].set_inputs(inputs);

        // Set the inputs of the remaining layers
        for i in 1..self.layers.len() {
            let previous_layer_outputs: Vec<f64> = self.layers[i - 1]
                .neurons
                .iter()
                .map(|n| n.output)
                .collect();
            self.layers[i].set_inputs(&previous_layer_outputs);
        }
    }
}