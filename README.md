<div align="center">

# 🧭 Heureka

### From a raw problem to an invention — reasoning you can watch, step by step.

Heureka turns any technical problem into a ranked set of solution candidates using
**TRIZ** and **SCAMPER**, then shows *why* it chose the winner — the full reasoning
trail, not just an answer.

<br/>

![Angular](https://img.shields.io/badge/Angular-21-DD0031?logo=angular&logoColor=white)
![NestJS](https://img.shields.io/badge/NestJS-11-E0234E?logo=nestjs&logoColor=white)
![Google ADK](https://img.shields.io/badge/Agent-Google%20ADK-4285F4?logo=google&logoColor=white)
![Gemini](https://img.shields.io/badge/LLM-Gemini-8E75B2?logo=googlegemini&logoColor=white)
![Nx](https://img.shields.io/badge/Monorepo-Nx-143055?logo=nx&logoColor=white)
![Cloud Run](https://img.shields.io/badge/Deploy-Cloud%20Run-4285F4?logo=googlecloud&logoColor=white)

<sub>Built with AI · GDG Wrocław</sub>

</div>

---

## ✨ What it does

You describe a technical problem in plain language. Heureka:

1. **Normalises** the problem into a crisp statement.
2. **Extracts the technical contradiction** — the classic TRIZ "improving X worsens Y".
3. **Looks up inventive principles** deterministically from the 39×39 TRIZ matrix (code, not the model).
4. **Generates candidates in parallel** — one branch per method (TRIZ principles, SCAMPER operators).
5. **Scores every candidate** against generic + problem-specific criteria.
6. **Picks the winner** by a deterministic `argmax` — the code decides, not the LLM.
7. **Assembles a reasoning trail** the UI renders as five inspectable steps.

Where a plain LLM gives you *an answer*, Heureka gives you a **defensible process**:
LLMs propose, deterministic code validates and decides.

---

## 🧠 The agent pipeline

A `SequentialAgent` (`inventive_problem_solver`) orchestrating eight nodes — LLM steps
where judgement is needed, deterministic Python where correctness matters.

```
 problem_normalizer ─▶ contradiction_extractor ─▶ triz_lookup* ─▶ criteria_extractor
                                                                        │
                                          ┌─────────────────────────────┘
                                          ▼
                              ┌── candidate_generators (parallel) ──┐
                              │   triz_generator                    │
                              │   scamper_generator                 │
                              └──────────────┬──────────────────────┘
                                             ▼
                              evaluator ─▶ choice_selector* ─▶ trail_assembler*
                                            (* = deterministic, zero LLM)
```

| Node | Type | Role |
|------|------|------|
| `problem_normalizer` | LLM | Normalise the raw problem statement |
| `contradiction_extractor` | LLM | Find the technical contradiction (params 1–39) |
| `triz_lookup` | **deterministic** | Resolve inventive principles from the TRIZ matrix |
| `criteria_extractor` | LLM | Derive evaluation criteria |
| `triz_generator` / `scamper_generator` | LLM (parallel) | Generate solution candidates per method |
| `evaluator` | LLM | Score every candidate |
| `choice_selector` | **deterministic** | `argmax` over totals — pick the winner |
| `trail_assembler` | **deterministic** | Assemble the final reasoning trail |

Built on the [Google Agent Development Kit](https://adk.dev/), served on Cloud Run with the A2A protocol, powered by **Gemini** via Vertex AI.

---

## 🖥️ The interface

An "engineering sheet" aesthetic — IBM Plex Mono, Big Shoulders Display, blueprint grid.

- **Live pipeline diagram** — an interactive [ng-diagram](https://www.ngdiagram.dev/) of the
  whole agent topology; every node exposes its internal prompt. During analysis it opens as a
  focused popup that **highlights and follows** the active step in real time.
- **Reasoning trail** — the five steps rendered as an inspectable document: problem →
  contradiction → candidates → evaluation → choice.
- **Voice input** — dictate the problem; audio is transcribed server-side via Google Cloud
  Speech-to-Text (credentials never touch the browser).
- **Accessibility built in** — dark mode, high-contrast mode, and text enlargement, right in
  the top bar, persisted across sessions.

> The UI is in **Polish**; the codebase and docs are in English.

---

## 🏗️ Architecture

```
┌───────────────┐        ┌───────────────┐         ┌────────────────────────┐
│   Frontend    │  HTTP  │      API      │  HTTP   │         Agent          │
│  Angular 21   │───────▶│   NestJS 11   │────────▶│  Python · Google ADK   │
│  ng-diagram   │        │  /solve       │         │  SequentialAgent       │
│  (Cloud Run)  │◀───────│  /speech/...  │◀────────│  (Cloud Run · us-east1)│
└───────────────┘        └───────┬───────┘         └────────────┬───────────┘
                                 │                              │
                          Google Cloud STT                Gemini · Vertex AI
```

- **`apps/frontend`** — Angular 21, standalone components, signals, `OnPush`. Design-token system + scoped Heureka theme.
- **`apps/api`** — NestJS gateway. Proxies `/solve` to the agent, hosts `/speech/transcribe`, serves Swagger at `/api/docs`.
- **`apps/agent`** — Google ADK agent (Python), TRIZ matrix + SCAMPER, deployed to Cloud Run.

---

## 🚀 Getting started

**Prerequisites:** Node 20+, [pnpm](https://pnpm.io/), and (for the agent) [uv](https://docs.astral.sh/uv/) + the `google-agents-cli`.

```bash
# install workspace deps
pnpm install

# run api (:3000) + frontend (:4200) together
pnpm start
```

Or run projects individually with Nx:

```bash
pnpm nx serve frontend      # http://localhost:4200
pnpm nx serve api           # http://localhost:3000/api  (Swagger: /api/docs)
```

Work on the agent from its own directory:

```bash
cd apps/agent
agents-cli run "How do we cool a high-power chip without adding a fan?"
agents-cli playground        # interactive web playground
```

---

## 🧩 Tech stack

| Layer | Tech |
|-------|------|
| Frontend | Angular 21 · signals · [ng-diagram](https://www.ngdiagram.dev/) · SCSS design tokens |
| API | NestJS 11 · Swagger · Google Cloud Speech-to-Text |
| Agent | Python · [Google ADK](https://adk.dev/) · Gemini (Vertex AI) · A2A |
| Tooling | Nx monorepo · pnpm · ESLint · Vitest |
| Infra | Google Cloud Run · Terraform · Docker |

---

## 📁 Project structure

```
apps/
├── frontend/          Angular app — the Heureka UI
│   └── src/app/
│       ├── pages/         analysis · methods · team
│       ├── components/    pipeline-diagram (agent-node, parallel-group)
│       ├── services/      analysis-engine · diagram-highlight · accessibility · speech
│       └── layouts/       heureka-layout (masthead + a11y controls)
├── api/               NestJS gateway (solve + speech proxy)
└── agent/             Google ADK agent
    └── app/agent.py       the SequentialAgent pipeline
```

---

## ☁️ Deployment

Everything runs on **Google Cloud Run**.

```bash
# API — resolves the live agent URL, builds, deploys (see tools/deploy-api.sh)
bash tools/deploy-api.sh

# Frontend
pnpm nx deploy frontend
```

The agent's infrastructure (service, service account, public-invoker IAM) is managed by
Terraform under `apps/agent/deployment/terraform/` — so unauthenticated access survives
redeploys instead of drifting into 403s.

---

## 🛠️ Common commands

| Command | Does |
|---------|------|
| `pnpm start` | Run api + frontend together |
| `pnpm nx build frontend` | Production build |
| `pnpm nx lint frontend` | Lint |
| `pnpm lint:all` | Lint every project |
| `pnpm format` | Format the workspace |
| `pnpm nx graph` | Explore the project graph |

---

<div align="center">
<sub>TRIZ · SCAMPER · reasoning you can inspect — <b>Heureka</b>.</sub>
</div>
