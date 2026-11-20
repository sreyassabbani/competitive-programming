```
.
├── usaco-guide
│   └── [level]
├── usaco-contests
│   └── [year]/[month]
├── leetcode
│   └── [number]-[name]
├── advent-of-code
│   └── [year]  # rust
├── .clang-format
├── .gitignore
├── README.md
└── compile_flags.txt
```

### USACO Guide
```
usaco-guide/[level]
├── [category]
│   ├── README.md
│   ├── [problem]
│   │   └── main.cpp
│   └── x
└── x
```

`[level]` — Canonical USACO levels

`[category]` — As outlined by the [USACO Guide](https://usaco.guide)

`[problem]` — USACO problems (mostly those suggested by the guide)

For every category, there is a `README.md` listing all the problems in the category I've attempted and wish to attempt soon. It's named that so that GitHub opens it automatically when you select a category folder.

---

You might also want to cf. [playground](https://github.com/sreyassabbani/playground).

## Version Tracking

Run `git config --local commit.template commit-template.txt`

Example commit message.

```
level(year contest, category, P#): name
```
So for example:
```
bronze(2016 open, complete search, P1): diamond collector
leetcode(1722): minimize hamming distance after swap operations
```
