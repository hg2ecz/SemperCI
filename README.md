# Yalci
Yet another local continuous integration tool

## Motivation

I want a tool that can

* watch a local git repository (even on a server)
* watch multiple branches of a single repository
* run multiple build definitions on the branch being changed (i.e. CI, code analysis, security scan)
* run user-defines build/deploy steps

## Development
[ ] Configured git repository with Travis CI
[ ] Configured logger (env-logger)
[ ] Configuration reader
[ ] File watcher for a git repository
[ ] git repository handler
[ ] Build step executor
[ ] Notification through X
