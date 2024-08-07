# Changelog

## [unreleased]

### 🚀 Features

- TrackingScope now reacts to component removals.
- Add ability to specify either a `Handle` or an `AssetPath` for `StyleBuilder` methods.
- Add `Icon::from_handle` to allow passing handle instead of path.
- Replace all `String` typed `Icon`s with `HandleOrOwnedPath<Image>`.
- Switch statement.
- New crate: bevy_quill_overlays.

### 🐛 Bug Fixes

- Switch was missing a fallback raze in some cases.
- Fix computation of AABBs for overlays.

### 🚜 Refactor

- Removed dependency on impl_trait_for_tuples for styles.

### ⚙️ Miscellaneous Tasks

- Add changelog, git-cliff configuration.
- Release

## [0.1.3] - 2024-07-25

### 🚀 Features

- Rectangle drag select.
- Vortex: Reading of input terminal values.
- Vortex: Hooked up bricks node.
- Added ListRow widget.

### 🐛 Bug Fixes

- Removed unnecessary dependency that was causing publish to fail.
- Remove some additional deps that are no longer needed.
- Renamed SimplexNoise to Noise (more accurate).
- Fix some more names.
- Dialogs now restore focus upon exit.

### 🚜 Refactor

- Get rid of NodeSpan.

### ⚙️ Miscellaneous Tasks

- Release

<!-- generated by git-cliff -->
