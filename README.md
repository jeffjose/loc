# loc

Count lines of code by language.

```bash
loc                      # current directory
loc src/ lib/            # specific paths
loc -l rust,ts           # only these languages
loc -i json,yaml,md      # exclude these
loc -H                   # show history over git commits
loc -H --samples 20      # sample 20 commits
loc --no-gitignore       # include files ignored by .gitignore
```

Respects `.gitignore` by default. Use `--no-gitignore` to include ignored files.
