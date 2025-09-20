#!/usr/bin/env -S just --justfile

[group("dev")]
build:
    cargo build

[group("dev")]
run +args="":
    cargo run -- {{ args }}

[confirm]
[group("misc")]
install:
    cargo install --git https://codeberg.org/xarvex/yt-dli.git

[confirm]
[group("misc")]
install-bin:
    cargo-binstall --git https://codeberg.org/xarvex/yt-dli.git --no-confirm --disable-telemetry yt-dli

[group("doc")]
[parallel]
media: (media-usage-arguments "true") (media-usage-mixed "true") (media-usage-interactive "true")

[group("doc")]
media-usage-arguments silent="false": (_media "usage_arguments" silent)

[group("doc")]
media-usage-mixed silent="false": (_media "usage_mixed" silent)

[group("doc")]
media-usage-interactive silent="false": (_media "usage_interactive" silent)

[group("doc")]
_media target silent="false":
    vhs media/vhs/{{ target }}.tape{{ if silent == "true" { " --quiet" } else { "" } }}
