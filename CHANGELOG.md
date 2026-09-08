# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

## [Unreleased]

### Fixed

- `transactions list/search --all` no longer discards everything when a page
  errors mid-walk. It now retries the failing page once, then returns the rows
  it already has, warns on stderr naming the page and row count, sets
  `"partial": true` in JSON output, and exits non-zero. Previously a 500 near
  the tail of the feed threw away ~2 minutes of successful paging and returned
  nothing. (Live: `--reviewed --all` now recovers 5,550 rows instead of 0.)
- `transactions list/search --limit N` is honoured again. The API ignores the
  GraphQL `first` argument and caps pages at its own size (25), so `--limit 200`
  silently returned 25 rows. `--limit` is now a **row target**: the CLI pages
  internally until it is met.

### Changed

- **Breaking (JSON output):** the `--page-info` key is now `pageInfo`, matching
  the camelCase of the fields inside it (`endCursor`, `hasNextPage`, …). It was
  `page_info`, and that snake/camel mismatch caused `jq '.pageInfo'` to return
  `null` against a perfectly good response — misread as a broken flag twice.
- `--limit` applies only when neither `--pages` nor `--all` is given, and may
  overshoot the target by up to one page rather than truncating (truncating
  would leave `pageInfo.endCursor` pointing past the dropped rows and silently
  break `--after`). `--pages N` keeps its existing meaning.

## [0.1.2] - 2025-12-20

- Fix: `copilot auth login` no longer fails when the browser helper can't run (falls back to manual token entry).
- UX: enable `copilot --version` and include the numeric version in `copilot version`.

## [0.1.1] - 2025-12-20

- Fix: improve token helper discovery for Homebrew / release tarballs.
- Release: bundle the Playwright token helper script in GitHub release artifacts.

## [0.1.0] - 2025-12-20

- Initial public release.
