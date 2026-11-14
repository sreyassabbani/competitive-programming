# WOOO!! 🎉

Thought I would discipline myself while working through the [USACO Guide](https://usaco.guide). Now also home to USACO contests, LeetCode, and Advent of Code.

This repo is really just for personal growth, but feel free to judge me for premature optimization, naming conventions, commit messages—no really, open an issue or PR and I'll try to better myself; standards/requirements are basically just maintaining the repo structure and commit style (following sections).

> 🔎 *Please note: I’d only like feedback on problems I’ve already solved. That's the part meant for collaboration.*

## Repo Structure

### Overview
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

### Playground

Moved to its own repo: [playground](https://github.com/sreyassabbani/playground).

## Version Tracking

Okay, as the main reason for this repo anyway is to keep myself accountable, I have this weird urge to keep everything organized when I'm locked in. Doing so, I've adopted a spin-off of [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/). I thought this would be nice and helpful. You can see some examples of this for yourself.

I've recently modified the message template to:

```
level(year contest, category, P#): name
```
So for example:
```
bronze(2016 open, complete search, P1): diamond collector
leetcode(1722): minimize hamming distance after swap operations
```

Run `git config --local commit.template commit-template.txt`

## Thoughts on Language

While this was initially all supposed to be written in C++, I gave up and couldn't resist the urge and power of sort by keys, easy functional programming, printing lists, etc.
