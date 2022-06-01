import nltk
import json
from nltk.stem import WordNetLemmatizer
from nltk.corpus import wordnet
import re

lemmatizer = WordNetLemmatizer()

def get_tag(word): 
    tag = nltk.pos_tag([word])[0][1][0].lower()
    tag_dict = {
        "j": wordnet.ADJ,
        "n": wordnet.NOUN,
        "v": wordnet.VERB,
        "r": wordnet.ADV
    }

    return tag_dict.get(tag, wordnet.NOUN)

def lemmatize(word):
    tag = get_tag(word)
    return lemmatizer.lemmatize(word, tag)

def repeat_zero(n):
    result = ""
    for _ in range(n):
        result += "0" 
    return result

file_paths = []
file_stem = "../data/reuters_test_set/reut2-"

for i in range(0, 22):
    prefix = repeat_zero(3 - len(str(i))) + str(i)
    file_paths.append(f"{file_stem}{prefix}.sgm")

with open("stopwords.json") as f:
    stopwords = json.load(f)["stopwords"]
    for i, file_path in enumerate(file_paths):
        with open(file_path) as file: 
            content = file.read()
            entries = content.split("</REUTERS>")
            
            for k, entry in enumerate(entries):
                start = entry.find("<BODY>") + len("<BODY>")
                end = entry.find("</BODY>")
                if start == -1 or end == -1:
                    continue
                #article = entry[start:end].translate(str.maketrans("", "", string.punctuation))
                #clean = re.sub(r"&.*;", "", entry[start:end].replace("\n", " "))
                trimmed = re.sub(r"[ ]{2,}", " ", entry[start:end].replace("\n", " "))
                no_comma = trimmed.replace(",", "")
                split = no_comma.split(" ")
                lemmatized = map(lemmatize, map(lambda x: re.sub(r"&.*;", "", x), filter(lambda x: not x in stopwords, split)))
                #no_punct = [item.translate(str.maketrans("", "", string.punctuation)) for item in split if not item.isnumeric()]
                f_articles = open(f"../preprocessed_data/articles_{i}_{k}", "x")
                f_articles.write(" ".join(lemmatized))
        print(f"File {i + 1}/{len(file_paths)}") 
