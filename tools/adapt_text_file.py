import argparse
import re

# Define the command-line arguments
parser = argparse.ArgumentParser(description="Process a large text file to create a sentence-per-line file")
parser.add_argument("input_file_path", type=str, help="path to the input file")
parser.add_argument("output_file_path", type=str, help="path to the output file")
parser.add_argument("--chunk_size", type=int, default=1000000, help="chunk size for processing the input file")

# Parse the command-line arguments
args = parser.parse_args()

# Open the output file in append mode
with open(args.output_file_path, "a", encoding="utf-8") as output_file:
    # Process the input file in chunks
    with open(args.input_file_path, "r", encoding="utf-8") as input_file:
        while True:
            # Read a chunk of the input file
            input_chunk = input_file.read(args.chunk_size)

            # If we've reached the end of the input file, break out of the loop
            if not input_chunk:
                break

            # Split the input chunk into sentences
            sentences = re.findall(r"[^.!?]+[.!?]", input_chunk, re.MULTILINE|re.DOTALL)

            # Write each sentence to a new line in the output file
            for sentence in sentences:
                output_file.write(sentence.strip() + "\n")

