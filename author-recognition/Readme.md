# Author Recognition:


The goal of this experiment is to amuse myself by trying various techniques to recognize the author of a text in Rust (Usually I would do that in Python but yeah, this helps me learn a bit how to do data science stuff in Rust which is something that I don't do regullarly)


# Single Char Frequencies

File: [single_char_recognition.rs](https://github.com/4nc3str4l/rust-experiments/blob/main/author-recognition/src/single_char_recognition.rs)

This is a very naive method that tries to count how many letters each author is using, then computing the average frecuency for each letter to then be able to do the same with any input text in order to compute the distance between the two average vectors.

The accuracy of this method for now is terrible (33%) taking in account that there are 3 authors, it is basically the same than choosing a random author xD

Maybe I will be able to tweak this but for now the results aren't very promising (as expected).

I just did this in order to setup the basis for more advanced methods.

**So here we have the terrible results for the naive system:**

| Author          | Initials | Num Correct | Total | Accuracy |
|-----------------|----------|-------------|-------|----------|
| HP Lovecraft    | HPL      | 135         | 454   | 29.7%    |
| Edgar Allan Poe | EAP      | 223         | 637   | 33.6%    |
| Mary Shelley    | MWS      | 168         | 488   | 35%      |

The tecnique being used in the end just tries to check what is the frecuency of every letter and this works well if the authors work in different languages, but as all of the m are using english the technique doesn't offer much.


## Trying to improve the method a little bit:

Even thought the method is quite naive, I would like to try to see if I can improve a bit it's performance, to achieve something that doesn't look as random.

## Single char frecuencies v2 (Not penalize for not using a character)

Now if the text doesn't contain a character the author get's penalized, I'll try to avoid this and check the results.


| Author          | Initials | Num Correct | Total | Accuracy |
|-----------------|----------|-------------|-------|----------|
| HP Lovecraft    | HPL      | 128         | 454   | 28.1%    |
| Edgar Allan Poe | EAP      | 223         | 637   | 35%    |
| Mary Shelley    | MWS      | 153         | 488   | 31.3%      |

Actually it is similar, nothing magical.

## Single char frecuencies v3 (Check against n most used characters)

This time I will try to evaluate just by checking the top 15 more used characters to check if it changes something

| Author          | Initials | Num Correct | Total | Accuracy |
|-----------------|----------|-------------|-------|----------|
| HP Lovecraft    | HPL      | 178         | 454   | 39%    |
| Edgar Allan Poe | EAP      | 268         | 637   | 42%    |
| Mary Shelley    | MWS      | 219         | 488   | 44.8%      |

MMmmm this technique seems to improve a bit the results which is interesting, less is more I guess...


# Bigrams

## Byte bigram based author recognition:

This time I'll do something very similar but by using the frecuency of the bigrams at byte level, this time I expect
to see an improvement over just analyzing character frecuencies, will see...