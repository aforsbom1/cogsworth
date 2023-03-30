#include <stdlib.h>
#include "network.h"

Network initialize_network(int num_inputs, int num_outputs, int num_hidden_layers, int num_neurons_per_hidden_layer)
{
    Network network = {
        .layers = malloc(sizeof(Layer) * (num_hidden_layers + 1)),
        .num_layers = num_hidden_layers + 1,
    };

    // Create input layer
    network.layers[0] = create_layer(num_inputs, num_neurons_per_hidden_layer);

    // Create hidden layers
    for (int i = 1; i < num_hidden_layers; i++)
    {
        network.layers[i] = create_layer(num_neurons_per_hidden_layer, num_neurons_per_hidden_layer);
    }

    // Create output layer
    network.layers[num_hidden_layers] = create_layer(num_neurons_per_hidden_layer, num_outputs);

    return network;
}