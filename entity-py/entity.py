import csv
import jellyfish
import pandas as pd
import re
import logging
from itertools import combinations


FORMAT = logging.Formatter("%(asctime)s %(message)s")
log = logging.getLogger(__name__)
log.setLevel("INFO")
ch = logging.StreamHandler()
ch.setFormatter(FORMAT)
log.addHandler(ch)


def read_files():
    dblp = pd.read_csv("data/DBLP2utf8.csv")
    scholar = pd.read_csv("data/ACM.csv")
    return (dblp, scholar)


def tokenize(string, n=10):
    """Tokenize a string into ngrams
    after removing punctuation and spaces but
    keeping the comma since author names are
    comma-separated."""
    if pd.isnull(string):
        return set()
    regex = re.compile("")
    string = regex.sub("", string)
    string = string.lower().split(",")
    tokens = set()
    for name in string:
        if len(name) < n:
            tokens.add(name)
        for i in range(len(name) - n + 1):
            tokens.add(name[i : i + n])
    return tokens


def generate_ngrams(dblp, scholar):
    dblp["ngrams"] = dblp["authors"].apply(tokenize)
    scholar["ngrams"] = scholar["authors"].apply(tokenize)
    return pd.concat([dblp, scholar])


def split_sentence(sentence):
    """Split a sentence into words and remove
    punctuation and spaces."""
    regex = re.compile("[^\w]")
    sentence = regex.sub(" ", sentence)
    return sentence.lower().split()


def create_reverse_index(df):
    reverse_index = {}
    for row in df.itertuples():
        for token in row.ngrams:
            if token in reverse_index:
                reverse_index[token].append(row.Index)
            else:
                reverse_index[token] = [row.Index]
    return reverse_index


def score_lev_distance(name1, name2):
    """Score the similarity between two names
    using Levenshtein distance."""
    return 1 - jellyfish.levenshtein_distance(name1, name2) / max(
        len(name1), len(name2)
    )


def score_blocking_key(df, reverse_index):
    result = []
    for key, indexes in reverse_index.items():
        log.debug("Processing key %s of size %s", key, len(indexes))
        for comb in combinations(indexes, 2):
            score = round(
                score_lev_distance(df.iloc[comb[0]].title, df.iloc[comb[1]].title), 2
            )
            result.append((key, comb[0], comb[1], score))
    return result


def benchmark():
    log.setLevel(logging.WARN)
    dblp, scholar = read_files()
    df = generate_ngrams(dblp, scholar)
    reverse_index = create_reverse_index(df)
    score_blocking_key(df, reverse_index)


if __name__ == "__main__":
    log.info("Reading files")
    dblp, scholar = read_files()
    log.info("Finished reading files.")

    log.info("Generating ngrams")
    df = generate_ngrams(dblp, scholar)
    log.info("Finished generating ngrams.")

    log.info("Creating reverse index")
    reverse_index = create_reverse_index(df)
    log.info("Finished creating reverse index.")

    log.info("Scoring matches within blocks")
    scores = score_blocking_key(df, reverse_index)
    log.info(
        "Finished scoring matches within blocks. {} matches found.".format(len(scores))
    )
