# Architecture, security and delivery plan

## Architecture

The proof of concept uses a PowerShell launcher only to test the integration hypothesis before building the requested interface. Each randomly generated profile ID maps to a directory under a per-user application directory. Names are display metadata, never path components. The launch passes the selected path through a child-process environment variable and starts the existing native Claude Code executable directly without constructing a shell command. Claude handles login. JSON contains labels and IDs only.

The intended production design is Tauri 2 with a minimal Vite/TypeScript interface and a Rust backend: profile repository, process launcher, path security, and optional session tracking. The frontend calls narrowly scoped backend commands; no generic shell command is exposed. Native Desktop isolation remains disabled unless an Anthropic-supported or explicitly experimental method is independently tested and documented. The normal Desktop application remains independently usable.

## Security model and unresolved risks

No credential extraction, token storage, protocol-handler mutation, binary patching, or automatic account switching is implemented. Removal only removes metadata and asks for confirmation; data remains. The prototype validates IDs and rejects reparse points at its managed root, profile directory and JSON file, and never recursively deletes data. It uses inherited Windows ACLs and is not hardened against malicious same-user processes, ancestor replacement, time-of-check/time-of-use races, or concurrent manager instances. A production implementation needs handle-based reparse-safe access, explicit user-only ACL verification, atomic and locked metadata updates, secure process environment policy, and careful error handling. Do not use the prototype in a hostile local environment.

Project, managed, and system configuration can still apply across profiles. Authentication isolation, credential storage, and browser callbacks require testing; removing a few known inherited variables cannot establish full isolation. The app should report unknown rather than guess login or running state. Account data is retained on removal; a future deletion feature needs explicit, separate confirmation and reparse-safe deletion.

## Milestones

1. Verify the actual CLI authentication isolation matrix with two authorized accounts, concurrent processes, reboot, logout, and global-config checks; block a release if it fails.
2. Build Tauri foundation and narrowly scoped Rust commands after the proof succeeds. Add profile edits, persist metadata safely, directory selection, clear errors, and accessibility.
3. Validate process behavior, environment handling, per-profile identity, and non-destructive removal. Track only processes whose identity can be established reliably, including manager restart behavior.
4. Research and independently test Desktop launch and OAuth behavior on each supported installation. Keep unsupported isolation unavailable; provide normal Desktop instructions.
5. Security review and tests, Windows 10/11 x64 integration, accessibility, uninstall, upgrades, failure recovery, and dependency audit. Measure startup, idle memory, and installer size rather than estimating.
6. Resolve naming and terms, package an unsigned test MSI/EXE, validate installation/uninstall and WebView2 prerequisites, then produce a release candidate. Signing and distribution decisions remain open.

## Test strategy

Automate unit tests for IDs, names, malformed JSON, paths and reparse points; property-test traversal and argument handling; simulate failed and concurrent writes; test restricted ACLs and process environment isolation with a harmless recording executable that never collects secrets. Run GUI and Windows integration tests using disposable profiles, followed by the manual real-account matrix in COMPATIBILITY.md. Test installers in clean Windows VMs, including offline/missing prerequisites, upgrade and uninstall. Never record tokens, cookies, or private account data in test artifacts.
