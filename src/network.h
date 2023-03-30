#include "layer.h"

typedef struct Network
{
    Layer *layers;
    int num_layers;
} Network;

Network initialize_network(int num_inputs, int num_outputs, int num_hidden_layers, int num_neurons_per_hidden_layer);