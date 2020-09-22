words = open("kotus_sanat.txt", "r", encoding="utf-8").read().splitlines()
words = [w for w in words if w[0].isalpha()]

for w1 in words:
    ends = [w1[-n:] for n in range(3, min(len(w1), 15))]
    for w2 in words:
        if w1 == w2:
            continue
        for end in ends:
            # if end not in words:
            # 	continue
            if w2.startswith(end):
                nw = w1[: -len(end)] + w2
                if nw not in (w1, w2):
                    print(nw)
