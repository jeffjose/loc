# loc

Count lines of code by language.

```bash
loc                      # current directory
loc src/ lib/            # specific paths
loc -l rust,ts           # only these languages
loc -i json,yaml,md      # exclude these
loc -H                   # show history over git commits
loc -H --samples 20      # sample 20 commits
```

Respects `.gitignore`.
