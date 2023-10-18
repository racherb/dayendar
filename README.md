# Dayendar

![Version](https://img.shields.io/badge/version-0.1.1-blue)
[![coverage](https://shields.io/endpoint?url=https://racherb.github.io/dayendar/coverage.json)](https://racherb.github.io/dayendar/index.html)

Welcome to the "Dayendar" crate. This is version 0.1.1 and it's currently in active development.

Dayendar implements advanced operations for working with calendars in Rust. The most important parts I highlight are:

- It allows creating DaysCalendar calendars from different data sources such as date vectors, individual months, binary patterns, etc.

- It provides functions to query and extract information from calendars such as years, months and present days.

- It has ```or```, ```and```, ```not```, ```match```, ```nomatch``` and others operations to modify calendars or combine calendars, as well as filtering by days of the week or ISO weeks, adding days, replicating patterns, etc.

- Conversion functions between different calendar and date representations (DaysCalendar, Date vectors, BiDay and Day days).

In general, it allows to create and manipulate calendars in a flexible and powerful way, performing operations both at bit level (BiDay) and with more abstract representations (Date dates).

## ⚠️ Warning ⚠️

The API of "Dayendar" is unstable at this stage. It is under active development, which means there might be significant changes at any time.

### Use at Your Own Risk

By using this crate, you acknowledge that there are associated risks due to its instability. Should you choose to use it, it is at your own discretion and responsibility. It's recommended not to use it in production environments until the API becomes stable.

## Installation

To add "Dayendar" to your project, add the following line to your `Cargo.toml` file:

```toml
[dependencies]
dayendar = "0.1.0"
```

## Contributions

If you wish to contribute to the development of "Dayendar", we encourage you to do so! Please read the contribution guidelines before submitting your changes.
Please note that contributions must be signed. Sign commits by adding ```-S``` option when you execute ```git commit```.

## Governance

The project is governed under a meritocracy model documented in GOVERNANCE.md.

Maintainers are responsible for reviewing, approving and managing changes, seeking consensus with the community.

Technical decisions follow the process detailed in GOVERNANCE.md.

Changes to governance itself must be approved by at least 2/3 of maintainers.

Participation of all community members is encouraged through issues, PRs, code reviews and patches.

Active members demonstrating valuable contributions and good technical judgement may be invited to become maintainers. See criteria in GOVERNANCE.md.

This section links to detailed information but summarizes the governance structure to make it visible and understandable to the community. It can be adapted and improved based on project member feedback.

## Generating a GPG key and configuring Git for Signed Commit

Install GPG. On Linux and Mac it generally comes preinstalled. On Windows you can install it through Gpg4win.
Generate your GPG key:

```bash
gpg --full-generate-key
```

Follow the prompts. It's recommended to use an email associated with your GitHub/GitLab account and a secure password.

Verify that the key has been generated:

```bash
gpg --list-secret-keys --keyid-format LONG
```

This will list the private keys generated. Copy the ID of the key.

Export the public key:

```bash
gpg --armor --export YOUR_KEY_ID
```

This will generate your public key in ASCII format.

Add the public key to GitHub/GitLab by pasting the result in the GPG keys section.

Configure Git to use this key:

```bash
git config --global user.signingkey YOUR_KEY_ID
git config --global commit.gpgsign true
```

Sign commits by adding -S when you git commit.
With this Git will be configured to sign all commits with your generated GPG key.
