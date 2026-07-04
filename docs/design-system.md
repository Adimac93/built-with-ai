# Heureka Design System

This design system captures the current frontend instead of replacing it. It has two surfaces:

- **Heureka product UI**: engineering-sheet visual language for the public analysis flow.
- **Admin UI**: Angular Material surface for users/orders.

## Token Model

Tokens use the day-2 structure:

- **Primitive**: raw values such as `sage-950`, `space-600`, `font-family-body`.
- **Semantic**: role names such as `background.canvas`, `content.primary`, `border.muted`.
- **Contextual**: component/task roles such as `button.primary.background`, `form.focusRing`, `sheet.grid`.

Runtime tokens live in [tokens.scss](../apps/frontend/src/design-system/tokens.scss). Figma/design handoff tokens live in [tokens.json](../apps/frontend/src/design-system/tokens.json).

## Heureka Identity

Heureka uses an engineering worksheet metaphor:

- pale paper canvas with a 24px grid
- dark ink borders and type
- red accent for focus, validation, deterministic/system emphasis
- squared cards and controls, no decorative rounded cards
- display headings in Big Shoulders Display, body in IBM Plex Sans, metadata in IBM Plex Mono

## Core Tokens

| Role | CSS token | Value/source |
| --- | --- | --- |
| Canvas | `--ds-color-bg-canvas` | `#f1f4ec` |
| Card | `--ds-color-bg-card` | `#fafbf6` |
| Primary content | `--ds-color-content-primary` | `#1b2b21` |
| Secondary content | `--ds-color-content-secondary` | `#5a685a` |
| Strong border | `--ds-color-border-strong` | `#1b2b21` |
| Muted border | `--ds-color-border-muted` | `#8f9c8c` |
| Focus/danger accent | `--ds-color-accent-danger` | `#c93a26` |
| Body font | `--ds-font-family-body` | IBM Plex Sans |
| Display font | `--ds-font-family-display` | Big Shoulders Display |
| Code/meta font | `--ds-font-family-code` | IBM Plex Mono |

## Components

### Button

- Use `.btn` for primary actions in the Heureka surface.
- Token roles: `--ds-component-button-primary-bg`, `--ds-component-button-primary-fg`, `--ds-component-button-hover-bg`.
- States: hover changes background to danger/accent red; disabled keeps ink background with reduced opacity.

### Form

- Use semantic `label`, native `textarea`, `fieldset`, and `legend`.
- Focus ring: `--ds-focus-outline`; must remain visible at 200% zoom.
- Error text uses `--ds-color-accent-danger` and `role="alert"` where errors are dynamic.

### Sheet/Card

- The main `.sheet` owns the page frame.
- Cards (`.doc-card`, `.cand`, `.person`, `.choice`) use strong or muted borders and the card background token.
- Avoid nested cards unless the inner element is a repeated item or a true framed tool.

### Navigation

- Primary public nav uses native links and `aria-current="page"`.
- Skip link is the first focusable control and targets `main#tresc`.
- Current location must be visible by color and contrast, not color alone.

## Accessibility Rules

- Target **WCAG 2.1 AA**.
- Normal text contrast must be at least 4.5:1; large text and UI boundaries at least 3:1.
- Prefer semantic HTML before ARIA.
- Every interactive element must be reachable by keyboard and have visible focus.
- Do not introduce accessibility overlays.
- Images need meaningful alt text unless decorative; decorative team portraits currently use empty `alt`.

## Figma/MCP Workflow

Use `tokens.json` as the importable design-token source for Figma/Dev Mode. If the Figma MCP workflow is added later, keep this repo file as the code-side source of truth and update it from intentional design-system changes, not from one-off generated screens.
