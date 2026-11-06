# ClaudeDock — source preview

Made by Mactan Digital. Free to use. Independent of and not affiliated with Anthropic. See `LICENSE.txt` for the free-use notice; third-party names, logos, and software remain subject to their owners' terms.

This is a Tauri 2 preview, not a verified multi-account solution. The Windows installer compiled and the app passed a basic startup check, but it must not be relied upon for account separation until the two-account test matrix has passed. The profile launcher starts the installed unmodified Claude Code executable and passes a distinct `CLAUDE_CONFIG_DIR`; Anthropic's documentation does not establish that all authentication artifacts are isolated on Windows. Confirm the active account in Claude Code before every sensitive session. Claude Desktop profile isolation is intentionally unavailable.

## Build on Windows

Install Node.js, Rust's MSVC toolchain, Microsoft C++ Build Tools with the Desktop development with C++ workload, and WebView2. See [Tauri's official prerequisites](https://v2.tauri.app/start/prerequisites/). From this directory, with access to npm and crates.io:

```powershell
npm install
npm run tauri dev
npm run tauri build
```

The project requests an NSIS installer; inspect the generated `src-tauri\target\release\bundle` directory after a successful build. Installation and uninstallation have not been tested. Dependency lockfiles were generated but dependencies have not been audited. This preview is not ready for distribution or use with sensitive accounts.

## Implemented in source

Add, edit and remove profile labels; persist nonsecret metadata; select a working directory by typing an absolute path; launch Claude Code in a new console with a profile-specific directory; detect standard per-user native/npm installs; and switch between light and dark themes. Removal retains all profile files. Login and process status are unknown, and Windows Terminal, folder picker, telemetry controls, and Desktop launching are not implemented.

## Risks and validation

The application has not been security-reviewed or tested end to end. It inherits Windows directory permissions, is not resistant to a hostile same-user process or reparse-point races, does not serialize separate manager instances, and writes metadata without crash-safe replacement. The environment filtering is incomplete; project or managed settings may be shared. Review the companion proof of concept's compatibility report and plan before using this source. Before release, test two authorized accounts concurrently, check their identities after both logins and after Windows restart, verify global authentication has not changed, test logout and revocation, and repeat after updates. Do not infer credential isolation from directory separation alone.

This preview uses a neutral product name because Anthropic's [Claude Code legal documentation](https://code.claude.com/docs/en/legal-and-compliance) contains naming and distribution restrictions; review the applicable terms before distribution.
