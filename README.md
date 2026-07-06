<div align="center">

# 🧭 Heureka

### From a raw problem to an invention — reasoning you can watch, step by step.

Heureka turns any technical problem into a ranked set of solution candidates using
**TRIZ** and **SCAMPER**, then shows *why* it chose the winner — the full reasoning
trail, not just an answer.

<br/>

![Rust](https://img.shields.io/badge/Rust-stable-CE422B?logo=rust&logoColor=white)
![Dioxus](https://img.shields.io/badge/Frontend-Dioxus%20%2B%20WASM-00A2FF)
![Axum](https://img.shields.io/badge/Backend-axum-6B4FBB)
![Gemini](https://img.shields.io/badge/LLM-Gemini-8E75B2?logo=googlegemini&logoColor=white)
![just](https://img.shields.io/badge/Tasks-just-5A4FCF)
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

A sequential pipeline of eight steps — LLM calls where judgement is needed,
deterministic Rust where correctness matters.

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

Hand-rolled in Rust ([axum](https://github.com/tokio-rs/axum)), served on Cloud Run, powered by **Gemini** via the Vertex AI REST API.

---

## 🖥️ The interface

An "engineering sheet" aesthetic — IBM Plex Mono, Big Shoulders Display, blueprint grid.

- **Live pipeline diagram** — an SVG map of the whole agent topology; every node exposes its
  internal prompt. During analysis it opens as a focused popup that **highlights and follows**
  the active step in real time.
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
│ Dioxus · WASM │───────▶│     axum      │────────▶│     axum · Rust        │
│  SVG diagrams │        │  /solve       │         │  TRIZ+SCAMPER pipeline │
│  (Cloud Run)  │◀───────│  /speech/...  │◀────────│  (Cloud Run · us-east1)│
└───────────────┘        └───────┬───────┘         └────────────┬───────────┘
                                 │                              │
                          Google Cloud STT                Gemini · Vertex AI
```

- **`apps/frontend`** — Dioxus 0.7 (Rust → WebAssembly), signals, router. Design-token CSS + scoped Heureka theme.
- **`apps/api`** — axum gateway. Proxies `/solve` to the agent, hosts `/speech/transcribe`.
- **`apps/agent`** — axum service running the TRIZ+SCAMPER pipeline: six Gemini calls + deterministic matrix lookup and argmax choice.

---

## 🚀 Getting started

**Prerequisites:** Rust stable (with the `wasm32-unknown-unknown` target),
[just](https://github.com/casey/just), [dioxus-cli](https://dioxuslabs.com/)
(`cargo binstall dioxus-cli`), and `gcloud` authenticated for Vertex AI.

```bash
# run agent (:8000) + api (:3000) + frontend (:8080) together
just dev
```

Or run apps individually:

```bash
just serve-agent            # http://localhost:8000  (Gemini via your gcloud token)
just serve-api              # http://localhost:3000/api
just serve-frontend         # http://localhost:8080
```

---

## 🧩 Tech stack

| Layer | Tech |
|-------|------|
| Frontend | Rust · Dioxus 0.7 (WASM) · SVG diagrams · CSS design tokens |
| API | Rust · axum · Google Cloud Speech-to-Text |
| Agent | Rust · axum · Gemini (Vertex AI REST) · TRIZ matrix in code |
| Tooling | cargo · just · clippy · dioxus-cli |
| Infra | Google Cloud Run · Terraform · Docker |

---

## 📁 Project structure

```
apps/
├── frontend/          Dioxus app — the Heureka UI
│   └── src/
│       ├── pages/         analysis · methods · architecture · team
│       ├── diagrams/      pipeline + architecture (SVG)
│       ├── trail.rs       /solve response → reasoning trail (unit-tested)
│       ├── state.rs       config · a11y · diagram-highlight signals
│       └── layout.rs      sheet layout (masthead + a11y controls)
├── api/               axum gateway (solve + speech proxy)
└── agent/             axum agent service
    └── src/pipeline/      the TRIZ+SCAMPER pipeline (prompts, schemas)
```

---

## ☁️ Deployment

Everything runs on **Google Cloud Run**.

```bash
just deploy             # agent → api → frontend, in dependency order
just deploy-api         # resolves the live agent URL, builds, deploys
just deploy-frontend
```

The agent's infrastructure (service, service account, public-invoker IAM) is managed by
Terraform under `apps/agent/deployment/terraform/` — so unauthenticated access survives
redeploys instead of drifting into 403s.

---

## 🛠️ Common commands

| Command | Does |
|---------|------|
| `just dev` | Run agent + api + frontend together |
| `just build` | Release build of everything |
| `just test` | Test every app |
| `just lint` | Clippy on every app (warnings are errors) |
| `just fmt` | Format all crates |
| `just ci` | Exactly what CI runs (lint + test + build) |

---

<div align="center">
<sub>TRIZ · SCAMPER · reasoning you can inspect — <b>Heureka</b>.</sub>
</div>
