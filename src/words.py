from nltk.corpus import words
word_list = words.words()
f = open("words.txt", "w")
for x in range(len(word_list)):
    f.write(word_list[x])
    f.write(" ")
f.close()