
# pba-cryptography-1

```
# Group 1
Anasse EL HANANI
Daniel Perez
Jules Leidelinger
Abhijana Agung Ramanda


****
## Steps followed

1. Start with string "The ".
2. Convert to binary
3. Convert first line from hex to binary.
4. Apply XOR with binary of "The " and binary of line 1 (padded with zeros to the right). Produce key
 that decodes first 4 characters of each line.
5. With the key, XOR the second line to achieve plain text of line 2. Decode "Gove".
6. "Gove" must be "Government " iterate untill achieving a key that can decode them all.


With this activity we learned that it's very dangerous to reuse keys because if an attacker happens to find a coincidence you are basically done.