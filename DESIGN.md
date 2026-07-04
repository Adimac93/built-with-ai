# Heureka Design System

Documentation for the design tokens defined in `tokens.scss`.

## Token architecture

Tokens follow a layered "day-2" design-system model, where each layer references the one below it:

```
primitive  ->  semantic  ->  contextual / component
```

- **Primitive** — raw values (hex colors, rem sizes, pixel spacing). No meaning attached.
- **Semantic** — role-based tokens that reference primitives (e.g. background, content, border).
- **Contextual / component** — tokens scoped to specific components or surfaces (buttons, chips, diagrams).

There are two scopes:

- `:root` — global primitives + the **admin surface** semantic layer (built on Angular Material's `--mat-sys-*` system tokens).
- `.heureka` — the **Heureka product** semantic and component layer.

---

## Primitives

### Colors

| Token                                   | Value                       |
| --------------------------------------- | --------------------------- |
| `--ds-color-primitive-sage-50`          | `#fafbf6`                   |
| `--ds-color-primitive-sage-100`         | `#f1f4ec`                   |
| `--ds-color-primitive-sage-200`         | `#d6decb`                   |
| `--ds-color-primitive-sage-500`         | `#8f9c8c`                   |
| `--ds-color-primitive-sage-700`         | `#5a685a`                   |
| `--ds-color-primitive-sage-950`         | `#1b2b21`                   |
| `--ds-color-primitive-red-100`          | `#f6e8e4`                   |
| `--ds-color-primitive-red-200`          | `#f2b8ad`                   |
| `--ds-color-primitive-red-600`          | `#c93a26`                   |
| `--ds-color-primitive-amber-300`        | `#f5d97a`                   |
| `--ds-color-primitive-amber-700`        | `#b07d1a`                   |
| `--ds-color-primitive-white-alpha-180`  | `rgba(255, 255, 255, 0.18)` |
| `--ds-color-primitive-slate-shadow-060` | `rgba(15, 23, 42, 0.06)`    |
| `--ds-color-primitive-slate-shadow-040` | `rgba(15, 23, 42, 0.04)`    |

The palette is anchored by a **sage** green scale (neutral/brand), with **red** as the accent/danger family and **amber** reserved for active-state highlighting. Slate-shadow and white-alpha tokens exist only for elevation and overlay effects.

### Typography

| Token                      | Value                                  | Use                |
| -------------------------- | -------------------------------------- | ------------------ |
| `--ds-font-family-admin`   | `Roboto, 'Helvetica Neue', sans-serif` | Material/admin UI  |
| `--ds-font-family-body`    | `'IBM Plex Sans', sans-serif`          | Body text          |
| `--ds-font-family-display` | `'Big Shoulders Display', sans-serif`  | Headings / display |
| `--ds-font-family-code`    | `'IBM Plex Mono', monospace`           | Code               |

| Weight token                | Value |
| --------------------------- | ----- |
| `--ds-font-weight-regular`  | `400` |
| `--ds-font-weight-medium`   | `500` |
| `--ds-font-weight-semibold` | `600` |
| `--ds-font-weight-bold`     | `800` |

| Line-height token        | Value  |
| ------------------------ | ------ |
| `--ds-line-height-body`  | `1.55` |
| `--ds-line-height-tight` | `1.1`  |

### Type scale

A dense, numerically-stepped scale ranging from `0.58rem` to `2.2rem`.

| Token                | Value     |     | Token                 | Value     |
| -------------------- | --------- | --- | --------------------- | --------- |
| `--ds-font-size-50`  | `0.58rem` |     | `--ds-font-size-500`  | `0.9rem`  |
| `--ds-font-size-75`  | `0.6rem`  |     | `--ds-font-size-600`  | `0.95rem` |
| `--ds-font-size-100` | `0.62rem` |     | `--ds-font-size-700`  | `1rem`    |
| `--ds-font-size-150` | `0.68rem` |     | `--ds-font-size-800`  | `1.05rem` |
| `--ds-font-size-200` | `0.7rem`  |     | `--ds-font-size-900`  | `1.25rem` |
| `--ds-font-size-250` | `0.72rem` |     | `--ds-font-size-1000` | `1.5rem`  |
| `--ds-font-size-300` | `0.75rem` |     | `--ds-font-size-1100` | `1.9rem`  |
| `--ds-font-size-350` | `0.78rem` |     | `--ds-font-size-1200` | `2.2rem`  |
| `--ds-font-size-400` | `0.8rem`  |     |                       |           |

### Spacing scale

Pixel-based scale from `0` to `88px`. Fine-grained at the low end (1–12px) for dense UI, coarser above.

| Token            | Value  |     | Token             | Value  |
| ---------------- | ------ | --- | ----------------- | ------ |
| `--ds-space-0`   | `0`    |     | `--ds-space-600`  | `16px` |
| `--ds-space-25`  | `1px`  |     | `--ds-space-700`  | `18px` |
| `--ds-space-50`  | `2px`  |     | `--ds-space-800`  | `20px` |
| `--ds-space-75`  | `3px`  |     | `--ds-space-900`  | `24px` |
| `--ds-space-100` | `4px`  |     | `--ds-space-1000` | `28px` |
| `--ds-space-150` | `5px`  |     | `--ds-space-1100` | `32px` |
| `--ds-space-200` | `6px`  |     | `--ds-space-1200` | `40px` |
| `--ds-space-300` | `8px`  |     | `--ds-space-1300` | `48px` |
| `--ds-space-350` | `10px` |     | `--ds-space-1400` | `64px` |
| `--ds-space-400` | `12px` |     | `--ds-space-1500` | `72px` |
| `--ds-space-500` | `14px` |     | `--ds-space-1600` | `88px` |

### Geometry & motion

| Token                        | Value        |
| ---------------------------- | ------------ |
| `--ds-border-width-100`      | `1px`        |
| `--ds-border-width-150`      | `1.5px`      |
| `--ds-border-width-300`      | `3px`        |
| `--ds-radius-50`             | `2px`        |
| `--ds-radius-100`            | `10px`       |
| `--ds-radius-150`            | `12px`       |
| `--ds-radius-200`            | `14px`       |
| `--ds-radius-pill`           | `999px`      |
| `--ds-grid-size-engineering` | `24px`       |
| `--ds-size-control-sm`       | `40px`       |
| `--ds-size-step-marker`      | `60px`       |
| `--ds-motion-fast`           | `120ms ease` |
| `--ds-motion-medium`         | `150ms ease` |

---

## Admin surface (`:root`)

Semantic tokens for the Material/admin surface. These map onto Angular Material's system tokens (`--mat-sys-*`), so the admin UI inherits the active Material theme.

| Token                                | Reference                                         |
| ------------------------------------ | ------------------------------------------------- |
| `--ds-color-admin-bg-app`            | `mat-sys-primary` mixed 6% into `mat-sys-surface` |
| `--ds-color-admin-bg-surface`        | `mat-sys-surface`                                 |
| `--ds-color-admin-bg-sidebar`        | `mat-sys-surface-container-low`                   |
| `--ds-color-admin-content-primary`   | `mat-sys-on-surface`                              |
| `--ds-color-admin-content-secondary` | `mat-sys-on-surface-variant`                      |
| `--ds-color-admin-brand-primary`     | `mat-sys-primary`                                 |
| `--ds-color-admin-brand-secondary`   | `mat-sys-tertiary`                                |
| `--ds-color-admin-border`            | `mat-sys-outline-variant`                         |
| `--ds-color-admin-danger`            | `mat-sys-error`                                   |
| `--ds-color-admin-on-danger`         | `mat-sys-on-error`                                |
| `--ds-shadow-admin-sm`               | Two-layer shadow from slate-shadow primitives     |

### Admin compatibility aliases

Retained so existing Material components keep working without refactoring:

| Alias                | Points to                 |
| -------------------- | ------------------------- |
| `--app-surface-tint` | `--ds-color-admin-bg-app` |
| `--app-radius-lg`    | `--ds-radius-200`         |
| `--app-radius-md`    | `--ds-radius-100`         |
| `--app-shadow-sm`    | `--ds-shadow-admin-sm`    |

---

## Heureka product layer (`.heureka`)

### Semantic colors

| Token                               | Reference | Role                         |
| ----------------------------------- | --------- | ---------------------------- |
| `--ds-color-bg-canvas`              | sage-100  | Page/canvas background       |
| `--ds-color-bg-card`                | sage-50   | Card background              |
| `--ds-color-bg-subtle`              | sage-100  | Subtle/muted background      |
| `--ds-color-bg-selected`            | sage-950  | Selected state (dark)        |
| `--ds-color-bg-score-win`           | red-100   | Winning-score background     |
| `--ds-color-grid-line`              | sage-200  | Grid lines                   |
| `--ds-color-content-primary`        | sage-950  | Primary text                 |
| `--ds-color-content-secondary`      | sage-700  | Secondary text               |
| `--ds-color-content-inverse`        | sage-50   | Text on dark surfaces        |
| `--ds-color-content-inverse-accent` | red-200   | Accent text on dark surfaces |
| `--ds-color-accent-danger`          | red-600   | Danger/accent                |
| `--ds-color-border-strong`          | sage-950  | Strong borders               |
| `--ds-color-border-muted`           | sage-500  | Muted borders                |
| `--ds-color-focus-ring`             | red-600   | Focus ring                   |

### Border & shadow composites

| Token                      | Definition                                  |
| -------------------------- | ------------------------------------------- |
| `--ds-border-default`      | `1.5px solid` strong border                 |
| `--ds-border-muted`        | `1px solid` muted border                    |
| `--ds-border-muted-dashed` | `1px dashed` muted border                   |
| `--ds-focus-outline`       | `3px solid` focus ring                      |
| `--ds-sheet-shadow`        | Double ring: 5px card + 6.5px strong border |

### Component tokens

| Token                                  | Reference                 |
| -------------------------------------- | ------------------------- |
| `--ds-component-button-primary-bg`     | selected (sage-950)       |
| `--ds-component-button-primary-fg`     | inverse content (sage-50) |
| `--ds-component-button-hover-bg`       | danger (red-600)          |
| `--ds-component-chip-bg`               | card background           |
| `--ds-component-chip-selected-bg`      | selected (sage-950)       |
| `--ds-component-note-bg`               | subtle background         |
| `--ds-component-diagram-active-border` | amber-700                 |
| `--ds-component-diagram-active-bg`     | amber-300                 |
| `--ds-component-diagram-active-shadow` | amber-700                 |

### Heureka compatibility aliases

Short-named aliases retained for legacy Heureka SCSS and diagram nodes:

| Alias        | Points to                      |
| ------------ | ------------------------------ |
| `--paper`    | `--ds-color-bg-canvas`         |
| `--grid`     | `--ds-color-grid-line`         |
| `--ink`      | `--ds-color-content-primary`   |
| `--graphite` | `--ds-color-content-secondary` |
| `--card`     | `--ds-color-bg-card`           |
| `--red`      | `--ds-color-accent-danger`     |
| `--line`     | `--ds-color-border-muted`      |

---

## Usage guidelines

- **Never reference primitives directly in components.** Consume semantic or component tokens so theme changes propagate cleanly.
- **Admin vs. Heureka.** Use `--ds-color-admin-*` inside the Material/admin surface; use the `.heureka`-scoped tokens inside the Heureka product. The admin layer tracks the Material theme; the Heureka layer is fixed to the sage/red/amber palette.
- **Compatibility aliases** (`--app-*`, `--paper`, `--ink`, etc.) exist only to support legacy code. Prefer the full `--ds-*` tokens in new work.
- **Naming convention:** `--ds-{category}-{subcategory}-{scale/role}`. Numeric scales are unitless steps, not literal values.
