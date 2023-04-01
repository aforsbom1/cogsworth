#include "network.h"

void forward_propagation(Network *network, double *inputs);

void back_propagation(Network *network, double *expected_outputs);

double sigmoid(double x);

int get_error_rate(double *output, double *expected_outputs);

void train_network(Network *network);