#include <stdio.h>
#include <stdlib.h>
#include "propagation.h"

int main()
{
    // Initialize the network
    Network network = initialize_network(3, 1, 5, 4);

    // Define the test dataset
    double inputs[4][2] = {
        {0, 0},
        {0, 1},
        {1, 0},
        {1, 1}};
    double expected_outputs[4] = {0, 0, 0, 1};
    int isTrained = 0;
    printf("Training the network...\n");

    // Test the network
    while (!isTrained)
    {
        forward_propagation(&network, *inputs);

        Layer *output_layer = &network.layers[network.num_layers - 1];
        double output[] = {output_layer->neurons[0].output, output_layer->neurons[1].output, output_layer->neurons[2].output, output_layer->neurons[3].output};
        back_propagation(&network, expected_outputs);

        isTrained = get_error_rate(&output, &expected_outputs);
    }

    Layer *output_layer = &network.layers[network.num_layers - 1];
    double output[] = {output_layer->neurons[0].output, output_layer->neurons[1].output, output_layer->neurons[2].output, output_layer->neurons[3].output};

    printf("\rOutput: %.2f, Expected: %f | Output: %.2f, Expected: %f | Output: %.2f, Expected: %f | Output: %.2f, Expected: %f", output[0], expected_outputs[0], output[1], expected_outputs[1], output[2], expected_outputs[2], output[3], expected_outputs[3]);

    return 0;
}

int get_error_rate(double *output, double *expected_outputs)
{
    int good_enough = 1;
    for (int i = 0; i < 4; i++)
    {
        double eoutput = expected_outputs[i];
        double aoutput = output[i];
        double err = eoutput - aoutput;
        printf("\rError: %.4f", err);

        if (fabs(err) > 0.0005)
        {
            good_enough = 0;
        }
    }
    return good_enough;
}