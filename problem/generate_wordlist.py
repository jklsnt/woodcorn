from nltk import pos_tag
from tqdm import tqdm
from subprocess import check_output
import csv

INPUT_FILE = 'words_alpha.txt'

input_len = int(check_output(['wc', '-l', INPUT_FILE]).split()[0])

with open(INPUT_FILE, 'r') as words_alpha, open('wordlist.txt', 'w+') as wf:
    writer = csv.writer(wf)
    writer.writerow(['word', 'pos'])
    for word in tqdm(words_alpha, total=input_len):
        word, pos = pos_tag([word.strip()])[0]  # OPTM: batch the input
        mask = 0
        if pos == 'NN':
            # print('NOUN', word)
            mask += 1
        if pos == 'VB':
            # print('VERB', word)
            mask += 2
        writer.writerow([word, mask])
