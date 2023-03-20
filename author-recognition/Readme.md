# Author Recognition:


The goal of this experiment is to amuse myself by trying various techniques to recognize the author of a text in Rust (Usually I would do that in Python but yeah, this helps me learn a bit how to do data science stuff in Rust which is something that I don't do regullarly)


## Single Char Frequencies

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
