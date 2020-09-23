import argparse
import os
import sys
import time
from collections import defaultdict
from typing import Dict, List

import tqdm

INPUT_FILE = os.environ.get("INPUT_FILE", "kotus_sanat.txt")

words: List[str] = []
words_by_letters: Dict[str, set] = None
all_letters = set()


def load_words():
    global words, words_by_letters

    if words:  # We've already done this initialization (maybe in a parent process)
        return

    with open(INPUT_FILE, "r", encoding="utf-8") as fp:
        words = [w.strip() for w in fp if w[0].isalpha()]

    words_by_letters = defaultdict(set)
    for word in words:
        for letter in word:
            words_by_letters[letter].add(word)
            all_letters.update(set(word))


def process_word(w1: str):
    ends = [w1[-n:] for n in range(3, min(len(w1), 15))]
    cand_words_by_initial_letter = get_cand_words_by_initial(w1)
    results = find_results(w1, cand_words_by_initial_letter, ends)
    return (w1, set(results))


def find_results(w1, cand_words_by_initial_letter, ends):
    results = []
    for end in ends:
        for w2 in cand_words_by_initial_letter[end[0]]:
            # if end not in words:
            # 	continue
            if w2.startswith(end):
                nw = w1[: -len(end)] + w2
                if nw != w1 and nw != w2:
                    results.append(nw)
    return results


def get_cand_words_by_initial(w1):
    cand_words = set()
    for letter in set(w1):
        cand_words.update(words_by_letters[letter])
    cand_words.discard(w1)
    cand_words_by_initial_letter = {letter: set() for letter in all_letters}
    for word in cand_words:
        cand_words_by_initial_letter[word[0]].add(word)
    return cand_words_by_initial_letter


def single_process(output=False):
    load_words()
    n = 0
    t0 = time.clock_gettime(time.CLOCK_MONOTONIC)
    for w1 in tqdm.tqdm(words):
        w1, results = process_word(w1)
        for result in results:
            n += 1
            if output:
                print(result)
    t1 = time.clock_gettime(time.CLOCK_MONOTONIC)
    print(f"{n} jatkoyhdyssanas found in {t1 - t0}", file=sys.stderr)


def multi_process():
    import multiprocessing

    load_words()
    multiprocessing.set_start_method("fork")
    with multiprocessing.Pool(initializer=load_words) as p:
        for w1, results in tqdm.tqdm(
            p.imap_unordered(process_word, words, chunksize=25), total=len(words)
        ):
            for result in results:
                print(result)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--mode", "-m", choices=("single", "multi", "bench"), required=True)
    args = ap.parse_args()
    if args.mode == "bench":
        single_process(output=False)
    elif args.mode == "single":
        single_process(output=True)
    else:
        multi_process()


if __name__ == "__main__":
    main()
