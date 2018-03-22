# SemperCI
Yet another local continuous integration tool.

## Motivation

My problems with another CI tools:
* They are either too complex and bloatware (Jenkins)
* ... or too primitive

I want a tool that can

* run even on my Raspberry
* watch a local git repository
* watch multiple branches of a single repository
* run multiple (independent) build definitions on the branch that's been being changed (i.e. CI, code analysis, code metrics, security scan)
* run user-defines build/deploy steps
* read configuration from an SQLite database

```
GIT REPO
  |
  +-- Branch 1 -- [CI-1] Build Definition 1 (i.e. CI build)
  |     |
  |     +-------- [CA-1] Build Definition 2 (i.e. static code analysis)
  |     |
  |     +-------- [RT-1] Build Definition 3 (i.e. report generation)
  |
  +-- Branch 2 -- [CI-2] Build Definition 1 (different from CI-1)
        |
        +-------- [CA-1] Build Definition 2 (same as CA-1)
```

In the previos configuragion there are two branches, each branch triggers their build definitions, CA-1 is a shared build definition: why would we duplicate a definition if it is not necessary?

## Development status

Please see the [board](https://github.com/fuszenecker/SemperCI/projects/1).

## Build status

[![Build Status](https://travis-ci.org/fuszenecker/SemperCI.svg?branch=master)](https://travis-ci.org/fuszenecker/SemperCI)
[![Build Status](https://travis-ci.org/fuszenecker/SemperCI.svg?branch=stable)](https://travis-ci.org/fuszenecker/SemperCI)
