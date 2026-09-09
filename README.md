# ward

Run a language server under a capability profile.

> **Status: early.** There is no product code yet — this repository currently
> holds a workspace skeleton and the merge gate. The capability profile format
> is the design decision everything else hangs off, and it is not settled.
> Nothing here is installable.

## The problem

Your editor starts language servers, formatters and linters as ordinary
processes. They inherit your home directory, your SSH keys, your cloud
credentials and your network. Nothing distinguishes "index this workspace" from
"read `~/.aws/credentials` and open a socket", and no editor asks you before
running the second one.

Opening an unfamiliar repository is enough to start that code.

## The approach

Ward is a shim. Your editor launches `ward` instead of the language server;
`ward` resolves a capability profile for that server and runs it confined to
what the profile grants. Because it speaks the Language Server Protocol on both
sides, it works in editors people already use — there is no plugin API to
adopt and no IDE to switch to.

The profiles are the hard part, and they are the product. rust-analyzer
genuinely needs the workspace *plus* `~/.cargo` *plus* the toolchain; a
workspace-only sandbox simply breaks it, and a sandbox that breaks the tool is
not a security control, it is an uninstall. Profiles derived from what a server
observably needs are the thing worth building.

First target is **rust-analyzer**, chosen because when confinement is wrong the
breakage is immediate and obvious rather than subtle.

## Building

```sh
cargo build --workspace
cargo test --workspace
```

The toolchain is pinned in [`rust-toolchain.toml`](rust-toolchain.toml); rustup
installs it on first use.

## Contributing

Every commit needs a sign-off trailer, `main` takes pull requests only, and the
`gate` check must be green. The details are in the
[organization CONTRIBUTING.md](https://github.com/dragon-sec/.github/blob/main/CONTRIBUTING.md).

Security issues do not go in public issues — see the
[security policy](https://github.com/dragon-sec/.github/blob/main/SECURITY.md).
Sandbox escapes and profile bypasses are the failure classes this project exists
to prevent, and they are the reports we most want.

## License

[Apache-2.0](LICENSE).
