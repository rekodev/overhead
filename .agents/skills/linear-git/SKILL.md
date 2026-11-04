---
name: linear-git
description: "Use for any git or Linear action in the Overhead repo: creating a branch, committing, pushing, opening or describing a PR, or creating, commenting on or editing an Overhead Linear issue or document (REK-123 IDs). Holds the branch, commit and PR naming, the PR description template, and how to reference Vibe Island and other GitHub issues in Linear without broken auto-links."
---

# Git & Linear Workflow (Overhead)

These are the repo's conventions. If you also have a personal
branch/commit/PR skill, use it for the mechanics, but these rules win here.

## Issues

Work is tracked in Linear, team **Rekodev** (key **`REK`**), project
**Overhead**. IDs look like `REK-123`. Milestones are feature areas
(1. Foundation … 9. Post-MVP); each has epics, and each epic has small
sub-issues. Pick up sub-issues, not epics.

The product spec is the Linear document "Overhead — Product spec". Every
ticket links to a section of it. If a ticket and the spec disagree, the spec
is updated first, then the code.

Set an issue to In Progress when you start it. Merging a PR that references
an issue should close it; if it doesn't move, set it to Done yourself.

## Naming

| Thing    | Format                                                          | Example                                          |
| -------- | --------------------------------------------------------------- | ------------------------------------------------ |
| Branch   | `<type>/rek-<n>-<slug>` with `feat/`, `fix/`, `chore/`, `docs/` | `feat/rek-194-nspanel`                           |
| Commit   | conventional with the ID as scope                               | `feat(REK-194): show a non-activating panel`     |
| PR title | `[REK-<n>] Imperative summary`                                  | `[REK-194] Show a non-activating floating panel` |

Build branch names yourself from this table. Don't use Linear's suggested
`gitBranchName` (e.g. `rekojsx/rek-123-…`): never prefix a branch with a
username, always start with the change type, and keep the slug short.

Several issues: `[REK-123, REK-124] …`. No issue given and none is obvious
from the branch or diff: don't invent one; use a clean title and commit
message without an ID.

## Commits and pull requests

- Branch from `main`; don't push to the branch of an already merged PR.
- Stage only the files that belong to the change.
- Before committing, run the checks for what changed (see `AGENTS.md`).
  There is no pre-commit hook in this repo, so nothing else will.
- Open PRs against `main` on `rekodev/overhead` with the authenticated `gh`
  CLI.

PR description shape (this overrides any personal PR skill's template):

```markdown
### Summary

One or two sentences on what changed and why, with the REK ID(s).

### Changes

- High-level change, one line each
- Another area that changed (panel, hook server, UI, plugin, tooling)

### Visual demonstration

Only for user-visible changes: a focused screenshot or short recording of
the panel, pill or card.
```

Keep it short. Reviewers read the code for specifics, so don't list file
names or every function touched. Aim for 3–6 `Changes` bullets that each
name an area. Don't add validation notes (test plans, lint/clippy results,
skipped checks) or a separate `Refs` section.

## Writing in Linear

Linear turns a bare `#123` into a link to a GitHub PR, and it picks the
wrong repo (InvoiceTrackr). So in issues, comments and documents:

- Refer to Vibe Island issues as `VI 78` with the full GitHub URL as the
  link, e.g. `[VI 78](https://github.com/vibeislandapp/vibe-island/issues/78)`.
- Refer to other repos' issues the same way (`tao 414`, linked).
- Never write a bare `#N`.

Issue descriptions use the sections **Context / Behaviour / Acceptance
criteria / Out of scope**, with technical detail in a collapsible
"Agent notes" block at the end. The workspace is at its free issue limit:
put new follow-ups in the "Overhead — Post-MVP backlog" document or as a
checklist on an existing issue instead of creating issues, unless the owner
says otherwise.
