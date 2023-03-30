#include <math.h>
#include "network.h"

void forward_propagation(Network *network, double *inputs)
{
    // Set input layer outputs
    Layer *input_layer = &network->layers[0];
    for (int i = 0; i < input_layer->num_neurons; i++)
    {
        Neuron *neuron = &input_layer->neurons[i];
        neuron->output = inputs[i];
    }

    // Feedforward through hidden layers and output layer
    for (int i = 1; i < network->num_layers; i++)
    {
        Layer *prev_layer = &network->layers[i - 1];
        Layer *cur_layer = &network->layers[i];

        for (int j = 0; j < cur_layer->num_neurons; j++)
        {
            Neuron *neuron = &cur_layer->neurons[j];

            double sum = 0;
            for (int k = 0; k < prev_layer->num_neurons; k++)
            {
                Neuron *prev_neuron = &prev_layer->neurons[k];
                sum += prev_neuron->output * prev_neuron->weights[j];
            }
            sum += neuron->bias;
            neuron->output = neuron->activation_function(sum);
        }
    }
}

void back_propagation(Network *network, double *expected_outputs)
{
    // Calculate output layer errors
    Layer *output_layer = &network->layers[network->num_layers - 1];
    for (int i = 0; i < output_layer->num_neurons; i++)
    {
        Neuron *neuron = &output_layer->neurons[i];
        double output = neuron->output;
        double error = expected_outputs[i] - output;
        neuron->error = error * output * (1 - output);
    }

    // Calculate hidden layer errors
    for (int i = network->num_layers - 2; i > 0; i--)
    {
        Layer *cur_layer = &network->layers[i];
        Layer *next_layer = &network->layers[i + 1];

        for (int j = 0; j < cur_layer->num_neurons; j++)
        {
            Neuron *neuron = &cur_layer->neurons[j];

            double error = 0;
            for (int k = 0; k < next_layer->num_neurons; k++)
            {
                Neuron *next_neuron = &next_layer->neurons[k];
                error += next_neuron->error * neuron->weights[k];
            }
            neuron->error = neuron->output * (1 - neuron->output) * error;
        }
    }

    // Update weights and biases
    for (int i = network->num_layers - 1; i > 0; i--)
    {
        Layer *cur_layer = &network->layers[i];
        Layer *prev_layer = &network->layers[i - 1];

        for (int j = 0; j < cur_layer->num_neurons; j++)
        {
            Neuron *neuron = &cur_layer->neurons[j];

            // Update biases
            neuron->bias += neuron->error;

            // Update weights
            for (int k = 0; k < prev_layer->num_neurons; k++)
            {
                Neuron *prev_neuron = &prev_layer->neurons[k];
                double delta = prev_neuron->output * neuron->error;
                neuron->weights[k] += delta;
            }
        }
    }
}
