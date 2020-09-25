import envparse
import tweepy
import os
import random


def get_tweepy() -> tweepy.API:
    auth = tweepy.OAuthHandler(
        consumer_key=os.environ["TWITTER_CONSUMER_KEY"],
        consumer_secret=os.environ["TWITTER_CONSUMER_SECRET"],
    )
    auth.set_access_token(
        key=os.environ["TWITTER_ACCESS_TOKEN"],
        secret=os.environ["TWITTER_ACCESS_TOKEN_SECRET"],
    )
    return tweepy.API(auth)


def get_words():
    with open("jatkoyhdyssanat.txt", "rb") as f:
        f.seek(0, os.SEEK_END)
        end = f.tell()
        while True:
            offset = random.randint(0, end)
            f.seek(offset)
            f.readline()  # skip the first one, it's likely in the middle of a word
            word = f.readline().strip().decode("utf-8")
            if word:
                yield word


def get_nice_word():
    for word in get_words():
        if " " in word:
            continue
        if not word.islower():
            continue
        return word


envparse.env.read_envfile()
tw = get_tweepy()
word = get_nice_word()
tw.update_status(status=word)
print("Tweeted:", word)
