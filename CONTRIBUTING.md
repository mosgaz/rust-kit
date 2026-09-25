# Contributing

Thanks for your interest in contributing to rust-kit. Please take a moment to review this before submitting your first pull request.

## About this repository

This is a monorepo for the rust-kit workspace, built on top of a vendored fork of [Rust/UI](https://github.com/rust-ui/leptos-ui) and powered by Leptos and Tailwind CSS.

```
crates/                     # Our crates
├── rust-kit-ui/            # package "ui" — components, templates, blocks
└── rust-kit-showcase/      # package "showcase" — demo app (SSR + WASM)

vendor/
└── rust-leptos-ui/         # Vendored fork of Rust/UI (branch: wasm-fix)
    ├── app/                # Leptos app (shell, routing)
    ├── server/             # Server binary
    ├── app_crates/
    │   └── registry/
    │       └── src/
    │           ├── ui/     # Component source (Button, Input, etc.)
    │           ├── demos/  # Demo/showcase components
    │           ├── blocks/ # Layout blocks
    │           └── hooks/  # Shared hooks
    └── crates/             # Supporting crates (tw_merge, icons, autoform, ...)
```

### How it fits together

- `vendor/rust-leptos-ui` is a fork of the upstream Rust/UI repository. It is **not modified** except on the `wasm-fix` branch, which carries minimal fixes for WASM/SSR compilation of `registry`.
- `crates/rust-kit-ui` (package `ui`) is our public facade. It re-exports components from `registry` and adds templates and blocks that are not part of upstream.
- `crates/rust-kit-showcase` (package `showcase`) is a demo application that consumes `ui` via `cargo-leptos`.

## Development

### Clone and install

```bash
git clone git@github.com:mosgaz/rust-kit.git
cd rust-kit

# Vendored fork must be on the wasm-fix branch
cd vendor/rust-leptos-ui
git checkout wasm-fix
cd ../..

cargo install --locked cargo-leptos
```

### Requirements

- Rust nightly (see `rust-toolchain.toml`)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Tailwind CSS in `PATH`

### Run the dev server

```bash
cd crates/rust-kit-showcase
cargo leptos watch
```

### Check the workspace

```bash
cargo check --workspace
```

Note: pure `cargo check` does not activate `erase_components`, which `registry` relies on. Prefer `cargo leptos watch` or pass `RUSTFLAGS="--cfg erase_components"` when checking `registry` directly.

### Format

Always run both formatters before committing:

```bash
cargo fmt && leptosfmt **/*.rs
```

## Scope of contributions

The main focus of contributions is:

- **Bug fixes** in our crates (`rust-kit-ui`, `rust-kit-showcase`)
- **New components** in `rust-kit-ui` that complement or override the vendored `registry`
- **Templates and blocks** not present upstream
- **Cross-platform compatibility** — ensuring components work correctly across OS and targets (Web, Desktop)

Changes to `vendor/rust-leptos-ui` should be avoided. If a fix is required, propose it upstream first via a pull request to [rust-ui/leptos-ui](https://github.com/rust-ui/leptos-ui). Only WASM/SSR compilation fixes live on the `wasm-fix` branch and are tracked separately.

If you'd like to propose a new component, please open a discussion first.

## Updating the vendored fork

To sync `vendor/rust-leptos-ui` with upstream while keeping the `wasm-fix` branch:

```bash
cd vendor/rust-leptos-ui
git fetch upstream
git rebase upstream/main wasm-fix
git push --force-with-lease origin wasm-fix
```

If the `wasm-fix` patch has already been merged upstream, the branch can be dropped and `main` used instead.

## Commit convention

Follow the `category(scope): message` format:

| Category | When to use |
|----------|-------------|
| `feat` | New component or feature |
| `fix` | Bug fix |
| `docs` | Documentation changes |
| `refactor` | Code change that isn't a fix or feature |
| `build` | Build system or dependency changes |
| `test` | Adding or updating tests |
| `ci` | CI configuration changes |
| `chore` | Everything else |

**Example:** `feat(ui): add primary button preset`

## Pull requests

1. Fork the repo and create a branch: `git checkout -b feat/my-component`
2. Make your changes following the steps above
3. Ensure `cargo fmt && leptosfmt **/*.rs` passes
4. Open a pull request with a clear description of what changed and why

## Bug reports

If you encounter a bug, please open an issue.

## Need help?

Feel free to open a discussion or reach out to the maintainers.