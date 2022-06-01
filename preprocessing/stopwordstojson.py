import json

with open("stopwordlist.txt", "r") as file:
    stopwords = [line.replace("\n", "") for line in file]
    dictionary = { "stopwords": stopwords }

    with open("stopwords.json", "w") as wordfile:
        wordfile.write(json.dumps(dictionary))
