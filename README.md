#Viola

A string instrument for rust.

---

## TODO

- for each Word, determine which part of speech it is
- check spelling
- divide text into sentences
- determine the parts of each sentence
- provide meaningful errors for incorrect sentences
- read file & parse into collection of Paragraphs & Sentences.
- find superfluous words and unnecessary repetition of information/ideas
- print pdf with annotations
- build linter for atom for text editing w/ grammar/spelling/etc. help

Usage:

```
use viola::instrument;

viola::instrument::book(directory_path); // runs analysis on entire directory
viola::instrument::document(file_path); // runs analysis on single file
viola::orchestrate::book(directory_path); // builds pdf with annotations
```

Directory of Documents/"Book"
  |
Document (most common phrases/words, )
  |
Paragraph (flow of ideas, guess subject, most common phrases/words)
  |
Sentence (Grammar, Usage, )
  |
  + -- Phrase
  |?     |
  + -- Words
         |
         + -- CharTypes

Words
  - part of sentence
    - subject, predicate, object (direct/indirect), predicate nominative, predicate adjective
  - part of speech ?
    - noun, pronoun, adjective, verb, adverb, conjunction, preposition, interjection
