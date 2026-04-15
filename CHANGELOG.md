# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.1.0] - 2026-04-15

### Added

- `Tle` struct representing a parsed Two-Line Element set (raw line content).
- `TleParseError` enum for parsing errors.
- `Tle::parse` — parses a three-line TLE string into a `Tle`.
