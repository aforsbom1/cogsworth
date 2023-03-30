typedef struct Neuron
{
    double *weights;
    double bias;
    double output;
    double error;
    int num_inputs;
    double (*activation_function)(double);
} Neuron;

Neuron createNeuron();
