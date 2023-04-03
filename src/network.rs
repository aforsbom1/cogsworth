use crate::layer::Layer;

pub struct Network {
    pub layers: Vec<Layer>,
}

impl Network {
    pub fn new(
        num_inputs: usize,
        num_outputs: usize,
        num_hidden_layers: usize,
        num_neurons_per_hidden_layer: usize,
    ) -> Self {
        let mut layers: Vec<Layer> = Vec::with_capacity(num_hidden_layers + 1);

        // Create input layer
        layers.push(Layer::new(num_neurons_per_hidden_layer, num_inputs));
        // Create hidden layers
        for _ in 1..num_hidden_layers {
            layers.push(Layer::new(num_neurons_per_hidden_layer, num_inputs));
        }
        // Create output layer
        layers.push(Layer::new(num_outputs, num_inputs));
        Self { layers }
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

    pub fn back_propagate(&mut self, outputs: &[f64]) {
        let output_layer = self.layers.last_mut().unwrap();

        // Calculate the error of the output layer
        for (i, neuron) in output_layer.neurons.iter_mut().enumerate() {
            neuron.error = outputs[i] - neuron.output;
        }

        // Calculate the error of the hidden layers
        for i in (0..self.layers.len() - 2).rev() {
            let num_neurons = self.layers[i].num_neurons;
            for j in 0..num_neurons - 1 {
                let error = &self.layers[i + 1].get_error(j);
                self.layers[i].neurons[j].error = *error;
            }
        }

        // Update the weights of the output layer
        for neuron in self.layers.last_mut().unwrap().neurons.iter_mut() {
            neuron.update_weights();
        }
    }

    pub fn get_outputs(&self) -> Vec<f64> {
        self.layers
            .last()
            .unwrap()
            .neurons
            .iter()
            .map(|n| n.output)
            .collect()
    }
}
