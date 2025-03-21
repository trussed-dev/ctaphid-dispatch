# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

-

## [0.3.0] - 2025-03-21

- Make `Dispatch` generic over the buffer size, rename `MESSAGE_SIZE` to `DEFAULT_MESSAGE_SIZE`, remove the `Message` type and add a `DefaultDispatch` type alias for `Dispatch<_, _, DEFAULT_MESSAGE_SIZE>`.

## [0.2.0] - 2025-01-08

- Optimize stack usage of `Dispatch::poll`
- Replace `trussed` dependency with `trussed-core`.
- Replace `heapless` dependency with `heapless-bytes`.
- Move the `app` and `command` modules into a separate crate, `ctaphid-app`, and re-export it.
- Make `App` trait generic over the response size.
- Remove unused `ShortMessage` type.
- Flatten the public module structure and remove unnecessary re-exports.

## [0.1.1] - 2022-08-22
- adjust to `interchange` API change

## [0.1.0] - 2022-03-05

- make a first proper release
