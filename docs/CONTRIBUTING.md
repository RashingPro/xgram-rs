# Contributing Guidelines

## Repository Guidelines

### Branches

PRs should be targeted to merge into `dev` branch, not `main`. The `main` branch contains the latest
release, not the latest changes made.

### Commit Messages

You must follow [Conventional Commits Specification](https://conventionalcommits.org/en/v1.0.0/).

## Code Guidelines

### Linting and Formatting

We use Clippy for linting and Rustfmt for formatting. You can run them like this:

```
cargo clippy --tests
cargo fmt
```

## Testing

It is recommended to run tests with nextest. A command for most cases is
`cargo nextest run --config-file nextest.toml`
