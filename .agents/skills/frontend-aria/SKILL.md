---
name: frontend-aria
description: >
  Use when writing front-end components that require ARIA attributes, accessible custom widgets,
  screen reader support, or dynamic content announcements.
---

## ARIA Guidelines

- Use ARIA landmarks to identify page regions (main, navigation, search, etc.)
- Apply appropriate ARIA roles to custom elements lacking semantic HTML equivalents
- Set `aria-expanded` and `aria-controls` for expandable content (accordions, dropdowns)
- Use `aria-live` regions with appropriate politeness settings for dynamic content updates
- Implement `aria-hidden` to hide decorative or duplicative content from screen readers
- Apply `aria-label` or `aria-labelledby` for elements without visible text labels
- Use `aria-describedby` to associate descriptive text with form inputs or complex elements
- Implement `aria-current` for indicating the current item in navigation or a process
- Avoid redundant ARIA that duplicates semantics of native HTML elements
- Apply `aria-invalid` and appropriate error messaging for form validation
