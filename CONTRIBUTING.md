# Contributing

Thank you for your interest in contributing to our project. To ensure code quality and consistency, it's essential that you follow the guidelines below.

When contributing to this repository, please first discuss the change you wish to make via issue, email, or any other method with the owners of this repository before making a change.

## Coding Standards

### Style Guide

Our project uses the official Rust style guide, also known as **Rustfmt**. It's important that all contributions adhere to this guide to maintain code consistency and readability.

- **Style Guide Reference**: [Rustfmt](https://github.com/rust-lang/rustfmt)

### Static Analysis Tools

To ensure the code meets our coding standards, the project uses static analysis tools. Specifically, we use `rustfmt` to automatically format the code according to Rust's standard style. Before submitting a pull request, make sure to run `rustfmt` on your code to ensure it's properly formatted.

### Style Exceptions

If, for any reason, you need to deviate from the style guide, you must clearly document it in the code at the specific location of the exception. Such exceptions should be rare and justified. For example:

```rust
// rustfmt::skip
// This section is skipped from rustfmt due to [specific reason]
...
```

## Developer Certificate of Origin (DCO)

In order to contribute, you must certify that you have the right to submit your contribution to this project and agree to the Developer Certificate of Origin (DCO) terms. This is certified by adding a "Signed-off-by" statement to your commits. The DCO is available as DCO.txt in this repository.

To certify your commits, simply include the statement:

 `Signed-off-by: Random J Developer <random@developer.example.org>`

when committing with Git. The name and email must match the user and email being used for Git.

All commits contributed to this project must be signed-off. By signing off with your commit, you agree to the following terms for your contribution:

- You certify that you have the right to submit the contribution in accordance to the project's license.
- You certify that the contribution was created by you or you have the rights to pass it on as an open source contribution.
- You agree to the Developer Certificate of Origin terms.

 Note that the "Signed-off-by" line is important, and Git has a '-s' command line option to add it automatically.

## How to Contribute

1. First discuss the change you wish to make via issue, email, or any other method with the owners of this repository
2. Fork the Repository: Fork the repository on your GitHub account.
3. Clone and Create a Branch: Clone the repository on your local machine and create a branch for your new feature or fix.
4. Make Your Changes: Make your changes and be sure to run `rustfmt` before committing.
5. Submit a Pull Request: Once you've completed your changes and ensured they meet the coding standards, submit a pull request to the original repository.
