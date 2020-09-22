import tqdm
from collections import defaultdict

with open("kotus_sanat.txt", "r", encoding="utf-8") as fp:
    words = [w.strip() for w in fp if w[0].isalpha()]

words_by_letters = defaultdict(set)
for word in words:
    for letter in word:
        words_by_letters[letter].add(word)

for w1 in tqdm.tqdm(words):
    ends = [w1[-n:] for n in range(3, min(len(w1), 15))]

    cand_words = set()
    for letter in w1:
        cand_words.update(words_by_letters[letter])
    cand_words.discard(w1)

    for w2 in cand_words:
        for end in ends:
            # if end not in words:
            # 	continue
            if w2.startswith(end):
                nw = w1[: -len(end)] + w2
                if nw not in (w1, w2):
                    print(nw)