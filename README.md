# pg_tokenizer (WIP)

A PostgreSQL extension that provides tokenizers for full-text search.

## Quick Start

TODO

## Example

```sql
SELECT create_tokenizer('tokenizer1', $$
model = "llmlingua2"
$$);

SELECT tokenize('PostgreSQL is a powerful, open-source object-relational database system. It has over 15 years of active development.', 'tokenizer1');
```

More examples can be found in [docs/03-examples.md](docs/03-examples.md).

## Documentation

- [Installation](docs/01-installation.md)
- [Development](docs/02-development.md)
- [Examples](docs/03-examples.md)
- [Usage](docs/04-usage.md)
- [Text Analyzer](docs/05-text-analyzer.md)
- [Model](docs/06-model.md)
- [Limitation](docs/07-limitation.md)
- [Reference](docs/00-reference.md)
