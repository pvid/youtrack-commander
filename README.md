# Youtrack Commander

[![Build status](https://github.com/pvid/youtrack-commander/workflows/CI/badge.svg)](https://github.com/pvid/youtrack-commander/actions?query=workflow%3ACI+branch%3Amaster)
[![crates.io](https://img.shields.io/crates/v/youtrack-commander.svg)](https://crates.io/crates/youtrack-commander)

Execute commands on Youtrack issues from the command line

- [Youtrack Commander](#youtrack-commander)
  - [Motivation](#motivation)
  - [Example usage](#example-usage)
  - [Configuration](#configuration)
  - [Coming soon](#coming-soon)
  - [Installation](#installation)
    - [Download pre-built binaries](#download-pre-built-binaries)
    - [Using cargo](#using-cargo)
  - [Similar projects](#similar-projects)

## Motivation

> "I can change the state of my Youtrack issue from the command line!"
>
> "... weird flex but OK"

Lately, due to a our team workflow, I had to frequently make small
changes to Youtrack issues, to communicate the current state of the task, whether it was
ready for code review, etc...

I wanted to do these interactions from the command line. And so, Youtrack Commander was born.
Its philosophy is to be just a thin wrapper around [issue commands](https://www.jetbrains.com/help/youtrack/incloud/Commands.html) and [search queries](https://www.jetbrains.com/help/youtrack/incloud/Search-for-Issues.html).

The real reason why this exists is that I wanted to try my hand at writing a CLI app
in Rust.

## Example usage

Basic use - executing a comment on issue ABC-123

```bash
youtrack-commander issue ABC-123 "State In progress assignee pavol.vidlicka"
```

Changing issue state and leaving a comment

```bash
youtrack-commander issue ABC-123 "State CR comment" -k "Ready for CR: ..."
```

Run `youtrack-commander --help` for a comprehensive help

## Configuration

`youtrack-commander` needs two things to work:

1. The URL of your Youtrack instance
2. A perm token to interact with the REST API (see [youtrack documentation](https://www.jetbrains.com/help/youtrack/standalone/Manage-Permanent-Token.html) for how to get one)

`youtrack-commander` loads these from a file located at `$HOME/.youtrack/commander.yml`
that has the following format:

```yaml
youtrack_url: "https://your-youtrack.com"
auth_token: "perm:your-auth-token"
```

## Coming soon

- a console mode with suggestions (implemented using the `/commands/assist` endpoint)

## Installation

### Download pre-built binaries

You can download a pre-built binaries for Linux and MacOS from [releases](https://github.com/pvid/youtrack-commander/releases).

The Linux binary needs OpenSSL to be installed on the system.

### Using cargo

Make sure that you have the Rust toolchain installed. [You can consult the Rust Book.](https://doc.rust-lang.org/book/ch01-01-installation.html)

To install run

```bash
cargo install youtrack-commander
```

## Similar projects

- [youtrack-cli](https://github.com/shanehofstetter/youtrack-cli) (and [youtrack-rest-client](https://github.com/shanehofstetter/youtrack-rest-client) that it uses) written in TypeScript
- [goutrack](https://github.com/codegoalie/goutrack) - a go project that also allows you to execute commands on issues
