#include <stdlib.h>
#include "network.h"

void initialize_network(Network *network, int num_inputs, int num_outputs, int num_hidden_layers, int num_neurons_per_hidden_layer)
{
    // Allocate memory for layers
    network->layers = malloc(sizeof(Layer) * (num_hidden_layers + 1)); // +1 for output layer
    network->num_layers = num_hidden_layers + 1;

    // Create input layer
    network->layers[0] = create_layer(num_inputs, num_neurons_per_hidden_layer);

    // Create hidden layers
    for (int i = 1; i < num_hidden_layers; i++)
    {
        network->layers[i] = create_layer(num_neurons_per_hidden_layer, num_neurons_per_hidden_layer);
    }

    // Create output layer
    network->layers[num_hidden_layers] = create_layer(num_neurons_per_hidden_layer, num_outputs);
}