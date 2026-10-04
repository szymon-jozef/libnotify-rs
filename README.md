![Tests](https://github.com/szymon-jozef/libnotify-rs/actions/workflows/tests.yml/badge.svg)
![GitHub License](https://img.shields.io/github/license/szymon-jozef/libnotify-rs)
![GitHub Release](https://img.shields.io/github/v/release/szymon-jozef/libnotify-rs)

<!--toc:start-->
- [Rust bindings for libnotify](#rust-bindings-for-libnotify)
  - [Structure](#structure)
  - [Documentation](#documentation)
  - [Add to project](#add-to-project)
    - [Nix flake](#nix-flake)
  - [Dependencies](#dependencies)
  - [Examples](#examples)
  - [Why?](#why)
  - [AI use](#ai-use)
  - [Roadmap](#roadmap)
<!--toc:end-->

# Rust bindings for libnotify

This crate exposes a user friendly (at least I hope), Rust api for [libnotify](https://gitlab.gnome.org/GNOME/libnotify). Made using [bindgen](https://github.com/rust-lang/rust-bindgen).

It comes with basic libnotify calls bindings and higher level abstraction over it.

## Documentation
Available at [github pages](https://szymon-jozef.github.io/libnotify-rs/libnotify_rs/index.html);

## Add to project
Type in:
```bash
cargo add libnotify-rs
```

### Nix flake
For nix users, there is also a flake inside. If you want to use it add this to your flake:

```nix
inputs = {
    libnotify-rs.url = "github:szymon-jozef/libnotify-rs";
};
```

And then to your derivation/devshell/wherever:

```Nix
packages = [
    inputs.libnotify-rs.packages.${stdenv.system}.default
];
```

## Dependencies
To use this crate you need to have installed:
- [libnotify](https://gitlab.gnome.org/GNOME/libnotify), duh (version 0.8.8 >= is tested. Older versions may work, but are unsupported)
- [clang](https://clang.llvm.org/) for bindgen
- [pkg-config](https://www.freedesktop.org/wiki/Software/pkg-config/?__goaway_challenge=meta-refresh&__goaway_id=e567f4f3ff21df0fb2b589fc86ed197e&__goaway_referer=https%3A%2F%2Fsearch.brave.com%2F), to find the above

## Examples
Refer to [examples dir](./examples)

Also have a look at [notify-send-rs](https://github.com/szymon-jozef/notify-send-rs). That's a small notify-send clone I made with this lib, as an example.

Sneak peek for making a notification:

### Without my super cool wrapper (:()

```rust
use libnotify_rs::api::{
    functions::{init, uninit},
    notification::{self, Notification},
};

fn main() -> Result<(), LibnotifyError> {
    init("I love coffee")?;
    let mut notify = Notification::new(
        "Remember to drink coffe!",
        "Coffee is very important for your mental health!",
        None,
    )?;

    notify.set_timeout(notification::Timeout::Custom(1000 * 10)); // 10 secs
    notify.set_urgency(notification::Urgency::Critical); // coffee is very important
    notify.set_category("Coffee")?;

    notify.show()?;

    uninit();

    Ok(())
}
```

### With my super cool wrapper (:))

```rust
use libnotify_rs::{builder::NotificationBuilder, context::LibnotifyContext};

/// Builder is recommended way to interact with libnotify notifications

fn main() -> Result<(), LibnotifyError> {
    let ctx = LibnotifyContext::new("Coffee addicted reminder")?; // builder requires context

    let notification_builder = NotificationBuilder::new(
        "Remember about your coffee!",
        "Only idiots forget about coffee!",
        None,
        &ctx,
    )?
    .set_urgency(libnotify_rs::api::notification::Urgency::Critical)
    .set_timeout(libnotify_rs::api::notification::Timeout::Never)
    .set_category("Coffee")?;

    notification_builder.show()?;

    // you don't need to uninitialize libnotify, cause context already does that!

    Ok(())
}

```


To test those examples use:
```bash
cargo r --example <example name>
```

## Why?
Why write this when there is already [rust-libnotify](https://github.com/hasufell/rust-libnotify)?

Two reasons:
- That crate is really, really old. Almost a decade old, as of now. This crate is not old.
- Rust-libnotify doesn't support setting callbacks.

I decided to write these bindings, when I thought the original doesn't support settings notification urgency level. Then I realised it did, but the project was fun, so I decided to finish it.

Why not [notify-rust](https://github.com/hoodie/notify-rust)? Well, I found it yesterday, that's why. But it looks great! You should definitely check it out. This crate is just a thin wrapper around libnotify c-calls.

## AI use
AI was used only for two things:
- Code review
- As a faster search engine

It didn't write any of the actual code, nor any documentation.

## Roadmap
I will be happy to fix any bugs that you may find! Just create a new issue, or better yet a pull request.
