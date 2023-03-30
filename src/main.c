#include <stdio.h>
#include <stdlib.h>
#include "propagation.h"

int main()
{
    // Initialize the network
    Network network = initialize_network(2, 1, 1, 3);

    // Define the test dataset
    double inputs[4][2] = {
        {0, 0},
        {0, 1},
        {1, 0},
        {1, 1}};
    double expected_outputs[4][1] = {
        {0},
        {1},
        {1},
        {0}};

    // Test the network
    for (int i = 0; i < 4; i++)
    {
        // Feed the inputs forward through the network
        forward_propagation(&network, inputs[i]);

        // Get the outputs
        Layer *output_layer = &network.layers[network.num_layers - 1];
        double output = output_layer->neurons[0].output;

        // Print the inputs, expected outputs, and obtained outputs
        printf("Input: [%f, %f], Expected Output: %f, Obtained Output: %f\n", inputs[i][0], inputs[i][1], expected_outputs[i][0], output);
    }

    // Free the memory allocated for the network
    for (int i = 0; i < network.num_layers; i++)
    {
        Layer *layer = &network.layers[i];
        for (int j = 0; j < layer->num_neurons; j++)
        {
            Neuron *neuron = &layer->neurons[j];
            free(neuron->weights);
        }
        free(layer->neurons);
    }
    free(network.layers);

    return 0;
}
