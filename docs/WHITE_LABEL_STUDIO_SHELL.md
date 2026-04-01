# White-label Studio Shell

## Role

`pauli-vibe_cockpit` becomes the frontend shell for the platform:
- public ArchonX landing page
- Jarvis-style studio cockpit
- white-label client frontend

## Design principles
- conversation first
- voice first
- minimal manual controls
- visible execution
- approvals only when needed
- no secrets in the frontend bundle

## Required views
- landing page
- studio cockpit
- project/repo context
- run monitor
- approval queue
- artifact pane
- agent theater
- workspace settings

## White-label requirements
- logo swap
- color/theme tokens
- avatar pack
- copy pack
- backend base URL binding
- workspace slug binding

## Backend contract assumptions
The shell talks to ArchonX for:
- run state
- workspace/provider config
- approvals
- artifacts
- live events

## Success condition
A new client frontend can be deployed from the same shell with branding changes only, while all runtime control stays in ArchonX.
