#include <argp.h>
#include <stdbool.h>
#include "utils.h"

const char *argp_program_version = "Cogsworth 0.1";
const char *argp_program_bug_address = "aforsbom1 at github.com";
static char doc[] = "Cogsworth, customizable Neural Network";

static struct argp_option options[] = {
    {"train", 't', 0, 0, "Train the network on a dataset.", 0},
    {"output", 'a', 0, 0, "Use trained network to output answers.", 1},
    {0},
};

struct arguments
{
    enum
    {
        TRAIN_MODE,
        WORK_MODE,
    } mode;
    char *filename;
};

static error_t parse_opt(int key, char *arg, struct argp_state *state)
{
    struct arguments *arguments = state->input;
    switch (key)
    {
    case 't':
        arguments->mode = TRAIN_MODE;
        break;
    case 'a':
        arguments->mode = WORK_MODE;
        break;
    case 0:
        arguments->filename = arg;
        break;
    default:
        return ARGP_ERR_UNKNOWN;
    }
    return 0;
}

static struct argp argp = {options, parse_opt, "[/path/to/file]", doc, 0, 0, 0};

int main(int argc, char *argv[])
{
    struct arguments arguments;
    argp_parse(&argp, argc, argv, 0, 0, &arguments);
    Network network;

    switch (arguments.mode)
    {
    case TRAIN_MODE:
        network = initialize_network(3, 1, 5, 4);

        printf("%s", arguments.filename);
        FILE *fp = fopen(arguments.filename, "wb+");
        // Train the network
        train_network(&network);

        fwrite(&network, sizeof(Network), 1, fp);

        fclose(fp);

        break;
    case WORK_MODE:
        printf("Work mode");
        break;
    }

    return 0;
}