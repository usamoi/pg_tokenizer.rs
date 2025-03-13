# pg_tokenizer (WIP)

A PostgreSQL extension that provides tokenizers for full-text search.

## Example

```sql
SELECT tokenizer_catalog.create_tokenizer('tokenizer1', $$
model = "bert_base_uncased"
pre_tokenizer.regex = '(?u)\b\w\w+\b'
[[character_filters]]
to_lowercase = {}
[[token_filters]]
stopwords = "nltk"
[[token_filters]]
stemmer = "english_porter2"
$$);

SELECT tokenizer_catalog.tokenize('PostgreSQL is a powerful, open-source object-relational database system. It has over 15 years of active development.', 'tokenizer1');
----
{2015:1, 2095:1, 2128:1, 2140:1, 2278:1, 2291:1, 2321:1, 2330:1, 2373:1, 2552:1, 2695:1, 2951:1, 4160:1, 4503:1, 4874:1, 12848:1, 14768:1, 17603:1, 20051:1, 22083:1}
```

```sql
CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    passage TEXT,
    embedding INT[]
);

SELECT create_text_analyzer('text_analyzer1', $$
pre_tokenizer = "unicode_segmentation"
[[character_filters]]
to_lowercase = {}
[[character_filters]]
unicode_normalization = "nfkd"
[[token_filters]]
skip_non_alphanumeric = {}
[[token_filters]]
stopwords = "nltk"
[[token_filters]]
stemmer = "english_porter2"
$$);

SELECT create_custom_model_tokenizer_and_trigger(
    tokenizer_name => 'tokenizer1',
    model_name => 'model1',
    text_analyzer_name => 'text_analyzer1',
    table_name => 'documents',
    source_column => 'passage',
    target_column => 'embedding'
);

INSERT INTO documents (passage) VALUES 
('PostgreSQL is a powerful, open-source object-relational database system. It has over 15 years of active development.'),
('Full-text search is a technique for searching in plain-text documents or textual database fields. PostgreSQL supports this with tsvector.'),
('BM25 is a ranking function used by search engines to estimate the relevance of documents to a given search query.'),
('PostgreSQL provides many advanced features like full-text search, window functions, and more.'),
('Search and ranking in databases are important in building effective information retrieval systems.'),
('The BM25 ranking algorithm is derived from the probabilistic retrieval framework.'),
('Full-text search indexes documents to allow fast text queries. PostgreSQL supports this through its GIN and GiST indexes.'),
('The PostgreSQL community is active and regularly improves the database system.'),
('Relational databases such as PostgreSQL can handle both structured and unstructured data.'),
('Effective search ranking algorithms, such as BM25, improve search results by understanding relevance.');

SELECT embedding FROM documents ORDER BY id;
----
{1:1, 2:1, 3:1, 4:1, 5:1, 6:1, 7:1, 8:1, 9:1, 10:1, 11:1, 12:1}
{1:1, 7:1, 13:1, 14:2, 15:2, 16:1, 18:1, 20:1, 21:1, 23:1, 25:1, 26:1}
{15:2, 20:1, 27:1, 28:1, 29:1, 30:1, 32:1, 33:1, 34:1, 36:1, 38:1}
{1:1, 13:1, 14:1, 15:1, 29:1, 40:1, 41:1, 42:1, 43:1, 44:1, 48:1}
{7:1, 8:1, 15:1, 28:1, 53:1, 54:1, 55:1, 56:1, 57:1}
{27:1, 28:1, 57:1, 61:1, 62:1, 63:1, 65:1}
{1:1, 13:1, 14:2, 15:1, 20:1, 25:1, 38:1, 69:2, 71:1, 72:1, 77:1, 78:1}
{1:1, 7:1, 8:1, 11:1, 81:1, 83:1, 84:1}
{1:1, 6:1, 7:1, 90:1, 91:1, 92:1, 93:1}
{15:2, 27:1, 28:1, 34:1, 55:1, 61:1, 84:1, 101:1, 102:1}
```

```sql
SELECT tokenizer_catalog.create_text_analyzer('test_german', $$
pre_tokenizer = "unicode_segmentation"
[[token_filters]]
pg_dict = "german_stem"
$$);

SELECT tokenizer_catalog.apply_text_analyzer('Aus so krummen Holze, als woraus der Mensch gemacht ist, kann nichts ganz Gerades gezimmert werden.', 'test_german');
----
{krumm,holz,woraus,mensch,gemacht,ganz,gerad,gezimmert}
```

## Installation

```bash
CREATE EXTENSION pg_tokenizer;
```

## Usage

The extension is mainly composed by 2 parts, `text analyzer` and `model`. `text analyzer` is used to parse the text and generate token arrays, which has similar functionality as `tsvector`. While `model` is used to generate token embeddings(token id array, can be casted to `bm25vector`), which is used for similarity search.

### Text Analyzer

`text analyzer` has 3 main components, `character filters`, `pre tokenizer`, `token filters`.

- character_filters: It is used to filter out some characters before tokenization. e.g. `to_lowercase`, `unicode_normalization`.
- pre-tokenizer: It is used to split the text into tokens. For example, `unicode segmentation` will split texts on Grapheme Cluster, Word or Sentence boundaries, according to the [`Unicode Standard Annex #29`](https://unicode.org/reports/tr29/)
- token_filters: It is used to filter out some tokens after tokenization. e.g. `stopwords`, `stemmer`.

### Model

`model` has 3 main types, `pre-trained`, `custom`, `external`.
- `pre-trained` models have pre-trained vocab lists and some pre-defined tokenization rules. e.g. [`bert_base_uncased`](https://huggingface.co/google-bert/bert-base-uncased), `wiki_tocken`(https://huggingface.co/datasets/iohadrubin/wikitext-103-raw-v1).
- `custom` models will maintain their own vocab mapping. You can build a custom model based on your own corpus easily.
- There are some useful tokenizers that cannot be decoupled into `text_analyzer` and `model`. We provide them as `external` models, and customize all configurations in the `model` part.

## Reference

### Functions

#### Text Analyzer
- `tokenizer_catalog.create_text_analyzer(name TEXT, config TEXT)`: Create a text analyzer.
- `tokenizer_catalog.drop_text_analyzer(name TEXT)`: Drop a text analyzer.
- `tokenizer_catalog.apply_text_analyzer(text TEXT, text_analyzer_name TEXT) RETURNS TEXT[]`: Apply a text analyzer to a text.

<br/>

- `tokenizer_catalog.create_synonym(name TEXT, config TEXT)`: Create a synonym dictionary.
- `tokenizer_catalog.drop_synonym(name TEXT)`: Drop a synonym dictionary.

#### Model

- `tokenizer_catalog.create_custom_model(name TEXT, config TEXT)`: Create a custom model.
- `tokenizer_catalog.create_custom_model_tokenizer_and_trigger(tokenizer_name TEXT, model_name TEXT, text_analyzer_name TEXT, table_name TEXT, source_column TEXT, target_column TEXT)`: Create a custom model tokenizer and trigger to update the target column automatically.
- `tokenizer_catalog.drop_custom_model(name TEXT)`: Drop a custom model.

<br/>

- `tokenizer_catalog.create_lindera_model(name TEXT, config TEXT)`: Create a lindera model.
- `tokenizer_catalog.drop_lindera_model(name TEXT)`: Drop a lindera model.

<br/>

- `tokenizer_catalog.create_huggingface_model(name TEXT, config TEXT)`: Create a huggingface model.
- `tokenizer_catalog.drop_huggingface_model(name TEXT)`: Drop a huggingface model.

#### Tokenizer

- `tokenizer_catalog.create_tokenizer(name TEXT, config TEXT)`: Create a tokenizer.
- `tokenizer_catalog.drop_tokenizer(name TEXT)`: Drop a tokenizer.
- `tokenizer_catalog.tokenize(text TEXT, tokenizer_name TEXT) RETURNS INT[]`: Tokenize a text.


## Configuration

> We utilize `TOML` syntax to express all configurations.

### Options for `text analyzer`

| Key               | Type           | Description                                                                            |
| ----------------- | -------------- | -------------------------------------------------------------------------------------- |
| character_filters | Array of Table | Character filters, see [Options for `character_filter`](#options-for-character_filter) |
| pre_tokenizer     | Table          | Pre-tokenizer, see [Options for `pre_tokenizer`](#options-for-pre_tokenizer)           |
| token_filters     | Array of Table | Token filters, see [Options for `token_filter`](#options-for-token_filter)             |

### Options for `character_filter`

| Key                   | Type        | Description                                                                                                                                      |
| --------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| to_lowercase          | Empty Table | Convert all characters to lowercase                                                                                                              |
| unicode_normalization | String      | Unicode normalization form, see [Unicode Normalization Forms](https://unicode.org/reports/tr15/), supported values: `nfkc`, `nfkd`, `nfc`, `nfd` |

You can choose only one of the above options for each character filter.

### Options for `pre_tokenizer`

| Key                  | Type        | Description                                                                                                    |
| -------------------- | ----------- | -------------------------------------------------------------------------------------------------------------- |
| regex                | String      | It will generate all tokens that match the regex pattern                                                       |
| unicode_segmentation | Empty Table | Split the text into tokens based on the Unicode Standard Annex #29                                             |
| jieba                | Table       | Split the text into tokens based on the Jieba Chinese tokenizer, see [Options for `jieba`](#options-for-jieba) |

#### Options for `jieba`

| Key        | Type    | Description                                                                            |
| ---------- | ------- | -------------------------------------------------------------------------------------- |
| mode       | String  | Jieba tokenizer mode, supported values: `full`, `precise`, `search`. default: `search` |
| enable_hmm | Boolean | Whether to enable HMM, default: `true`                                                 |

### Options for `token_filter`

| Key                   | Type        | Description                                                                                                                                                                         |
| --------------------- | ----------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| skip_non_alphanumeric | Empty Table | Skip tokens that all characters are non-alphanumeric                                                                                                                                |
| stemmer               | String      | Stemmer, see [Supported values for `stemmer`](#supported-values-for-stemmer)                                                                                                        |
| stopwords             | String      | Stopwords, supported values: `lucene`, `nltk`, `iso`                                                                                                                                |
| pg_dict               | String      | Using [postgres text search dictionary](https://www.postgresql.org/docs/current/textsearch-dictionaries.html). We currently support all dictionaries except `Thesaurus Dictionary`. |

You can choose only one of the above options for each token filter.

#### Supported values for `stemmer`

arabic, armenian, basque, catalan, danish, dutch, english_porter, english_porter2, estonian, finnish, french, german, greek, hindi, hungarian, indonesian, irish, italian, lithuanian, nepali, norwegian, portuguese, romanian, russian, serbian, spanish, swedish, tamil, turkish, yiddish

### Options for `tokenizer`

| Key           | Type   | Description                                                                       |
| ------------- | ------ | --------------------------------------------------------------------------------- |
| text_analyzer | String | Text analyzer name. If you are using an external model, you can just ignore this. |
| model         | String | Model name. We have some builtin models, see [Builtin models](#builtin-models)    |

#### Builtin models

- [bert-base-uncased](https://huggingface.co/google-bert/bert-base-uncased)
- [wiki_tocken](https://huggingface.co/datasets/iohadrubin/wikitext-103-raw-v1)
- [gemma2b](https://huggingface.co/google/gemma-2b)
- [llmlingua2](https://huggingface.co/microsoft/llmlingua-2-xlm-roberta-large-meetingbank)

### Options for `custom model`

| Key           | Type   | Description         |
| ------------- | ------ | ------------------- |
| table         | String | Table name.         |
| column        | String | Column name.        |
| text_analyzer | String | Text analyzer name. |

### Options for `lindera model`

It's totally the same as lindera tokenizer configs, see [Lindera](https://github.com/lindera/lindera).
