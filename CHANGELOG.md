# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
    Add new changelog entries here.
    Each entry may be annotated with "Added", "Changed", "Removed", and "Fixed" titles.

    Example:

    ## [1.0.0] - May 16, 2022

    ### Added
    - New visual identity.

    ### Changed
    - Start using "changelog" over "change log" since it's the common usage.

    ### Removed
    - Section about "changelog" vs "CHANGELOG".

    ### Fixed
    - Fix typos in recent README changes.
    - Update outdated unreleased diff link.
-->

## Unreleased

### Added

- The crate dependencies `serde`, `serde_yaml`, `rusqlite`, `actix`, `actix-web` and `actix-web-actors` for configuration, data storage and web server functionality, respectively.
- The `colored` dependency so we can provide some nice debug output.
- The `Controller` struct, which represents a microcontroller client.
- A helper method for console output.
- A basic WebSocket server implementation.