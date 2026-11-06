# Feasibility and compatibility report

Research date: 2026-11-06. This is a documentation and source review, not an independent certification of either reference project or Anthropic's terms. Versions change; recheck before release.

## Findings

| Capability | Evidence and status |
| --- | --- |
| Claude Code configuration directories | [Anthropic's settings documentation](https://code.claude.com/docs/en/settings) documents `CLAUDE_CONFIG_DIR` for settings, session history, and plugins. It also describes a separate `~/.claude.json` holding account details. The documentation reviewed does not establish complete independent Windows credential isolation. **Configuration mechanism documented; independent authentication unverified.** |
| Two simultaneous CLI subscriptions | Separate processes and environment variables are technically possible, but actual sign-in, cross-account state, restart persistence, and effect on global authentication were **not tested**. The prototype must not be presented as proof of these outcomes. |
| Native Desktop parallel instances and separate sessions | No Anthropic-supported profile argument or multiple-instance guarantee was found in the reviewed documentation. The two community projects claim `--user-data-dir` works, but those claims are not independent verification. **Experimental; not implemented.** |
| Chat, Code and Cowork in isolated Desktop | **Not tested**. Community documentation describes shared Code settings, special Cowork VM path requirements, and login-protocol routing, indicating that a separate Chromium directory alone is insufficient evidence of complete isolation. |
| Desktop updates and restart persistence | **Not tested**. Executable locations are versioned in some installations; protocol handlers and VM paths may change. No compatibility guarantee is made. |
| Standard Desktop | [Anthropic's installation guide](https://support.claude.com/en/articles/10065433-install-claude-desktop) documents the official application and feature availability. Users may open it normally with its existing account; it is not isolated by this prototype. |
| Terms | [Claude Code legal documentation](https://code.claude.com/docs/en/legal-and-compliance) requires an unmodified binary and users' own authentication, and includes product, credential, and naming restrictions. It allows end users to sign in to the unmodified binary with their own subscription in the stated circumstances. This is not a legal conclusion that this product or multi-account use is approved. The proposed product name uses Anthropic's name; resolve naming and product-distribution requirements before release. |

## Reference projects inspected

- [fredless/claude-windows-multiprofile](https://github.com/fredless/claude-windows-multiprofile): its README describes MSIX executable discovery, `--user-data-dir`, optional `CLAUDE_CONFIG_DIR`, login routing, and Cowork VM placement. It claims profile independence but also identifies shared Code configuration and VM path dependencies. These are third-party claims, not verified behavior or supported Anthropic APIs.
- [PriyanshuGeTRekT/Claude-Code-Desktop-Switcher](https://github.com/PriyanshuGeTRekT/Claude-Code-Desktop-Switcher): its [script](https://github.com/PriyanshuGeTRekT/Claude-Code-Desktop-Switcher/blob/main/ClaudeSwitcher.ps1) and README describe separate Electron directories and optional `claude://` handler changes. Its own reported tests include a specific Windows 11 Store version and stand-in tests. Its login routing and account-detection heuristics are not adopted here. Different descriptions of Cowork paths across the projects reinforce the need for version-specific testing.

## Evidence gathered locally

On this machine, profile creation and listing were exercised against a disposable data root, and the installed npm Claude Code executable was detected. No actual login, production Claude session, Desktop app, reboot, installer, performance, or update test was performed. Rust and Cargo were not found on PATH; the Tauri app was not built.

## Required manual validation before claiming account isolation

1. Use a disposable Windows account and two independently authorized Claude accounts. Record Windows, Claude Code and Desktop versions and installation types, without recording credentials.
2. Create two profiles. Launch each and complete Anthropic's own login. In each session use Claude's account/status display to verify the intended identity; recheck the first after signing in to the second. Examine global configuration changes without copying or exposing secrets.
3. Keep both sessions open, use each concurrently within its own entitlement, close and reopen the manager, then restart Windows and repeat identity checks. Test logout/revocation independently.
4. Test inherited API key/provider variables, managed and project settings, and browser callbacks. Verify no cross-account sign-in or unintended global modifications. Stop if identity cannot be reliably established.
5. For any proposed Desktop integration, separately test Chat, Code, Cowork, OAuth callbacks, updates, process concurrency, and restart persistence across supported installation types. Do not enable experimental behavior in a release until its risks and rollback are documented.
6. Repeat after Claude updates. Record failures and version boundaries; never claim untested versions work.
