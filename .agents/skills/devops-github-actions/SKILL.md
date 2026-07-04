---
name: devops-github-actions
description: >
  Use when configuring GitHub Actions workflows, writing CI/CD pipelines,
  setting up composite actions, or managing workflow secrets and environment variables.
---

## GitHub Actions Guidelines

- Check if `package.json` exists in project root and summarize key scripts
- Check if `.nvmrc` exists in project root
- Check if `.env.example` exists in project root to identify key `env:` variables
- Run `git branch -a | cat` to verify whether the repo uses `main` or `master`
- Always use `env:` variables and secrets attached to jobs instead of global workflows
- Always use `npm ci` for Node-based dependency setup
- Extract common steps into composite actions in separate files
- For each public action, verify the most up-to-date version via:
  ```bash
  curl -s https://api.github.com/repos/{owner}/{repo}/releases/latest
  ```
  Use only the major version tag.
