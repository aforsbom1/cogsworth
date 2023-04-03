import sentencepiece as spm
import io
import argparse

parser = argparse.ArgumentParser(description="Generate a sentencepiece model")
parser.add_argument("input_file_path", type=str, help="path to the training data file")
parser.add_argument("output_file_path", type=str, help="path to the output model file")
args = parser.parse_args()
model = io.BytesIO()
with open(args.output_file_path, 'wb') as output_file:
    spm.SentencePieceTrainer.train(input=args.input_file_path, vocab_size=1000, model_writer=model, input_sentence_size = 100000)
    output_file.write(model.getvalue())