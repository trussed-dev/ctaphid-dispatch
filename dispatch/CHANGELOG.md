# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
- Optimize stack usage of `Dispatch::poll`
- Replace `trussed` dependency with `trussed-core`.
- Replace `heapless` dependency with `heapless-bytes`.
- Move the `app` and `command` modules into a separate crate, `ctaphid-app`, and re-export it.
- Make `App` trait generic over the response size.
- Remove unused `ShortMessage` type.

## [0.1.1] - 2022-08-22
- adjust to `interchange` API change

## [0.1.0] - 2022-03-05

- make a first proper release
