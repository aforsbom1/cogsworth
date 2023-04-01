#include "utils.h"

int main()
{
    // Initialize the network
    Network network = initialize_network(3, 1, 5, 4);

    // Train the network
    train_network(&network);

    return 0;
}