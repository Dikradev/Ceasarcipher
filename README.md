# Ceasarcipher

## Basic Idea:
Each letter in the plaintext is replaced by a letter a fixed number of positions down the alphabet.
This fixed number of positions is called the key or shift.
## Example:
Let’s say our key is 3.
If we apply the Caesar cipher to the English alphabet:
A becomes D
B becomes E
C becomes F
…and so on.
## Encryption Process:
Given a plaintext message (e.g., “HELLO”), we shift each letter by the key value.
Using a key of 3:
H → K
E → H
L → O
L → O
O → R
So, “HELLO” encrypted with a Caesar cipher and a key of 3 becomes “KHOOR.”
## Decryption:
To decrypt, we reverse the process:
Given the ciphertext “KHOOR,” we shift each letter back by 3 positions to get “HELLO.”
