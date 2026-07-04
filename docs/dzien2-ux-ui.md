# CONTEXT.md — Hackathon Dzień 2 (UI/UX, dostępność, AI-native design systemy)

> Ten plik jest źródłem prawdy o warstwie designu, dostępności i workflowów AI↔Figma z dnia 2 („UI/UX & AI-Native Design Systems").
> Agencie: nie wymyślaj narzędzi, nazw ani skrótów od nowa — trzymaj się tego, co poniżej. Jeśli czegoś brakuje, powiedz o tym, nie zgaduj.
> Uzupełnia: CONTEXT dnia 1 (proces produktowy) i CONTEXT dnia 4 (stack techniczny). Ten plik = warstwa UI/UX + design system + integracja AI z Figmą.

**Kontekst wydarzenia:** Build with AI Wrocław — 6-dniowy hackathon/warsztat. Dzień 2 (30.06) = „UI/UX & AI-Native Design Systems": dostępność cyfrowa → AI-native design systemy (Antigravity/Figma/MCP) → od design systemu do działającego produktu. Prowadzący: Kinga Witko, Witold Iglewski, Przemysław Nowak.

---

## 1. Główna filozofia dnia

1. **Dostępność od początku, nie na końcu.** Semantyczny HTML i dobra struktura to fundament — bez tego technologie wspomagające nie działają, niezależnie od tego, ile AI się doda na wierzch.
2. **Design system jako źródło prawdy (source of truth).** Agent kodujący ma czerpać z design systemu przez MCP, a nie zgadywać styl — automatyzuje to dziedziczenie decyzji projektowych w generowanym kodzie.
3. **Najpierw użyteczność i spójność, potem estetyka.** Na hackathonie liczy się szybka, spójna walidacja pomysłu (MVP), nie dopieszczony wizualnie interfejs.
4. **AI generuje „happy path".** Automatyzacja (Figma→kod, one-shot prompty) przyspiesza pracę, ale nie pokrywa edge case'ów, testów dostępności ani bezpieczeństwa — to zostaje po stronie człowieka.
5. **Bezpieczeństwo kluczy i uprawnień agentów jest nienegocjowalne.** Klucze API/Personal Access Tokeny to sekrety; nieostrożne agenty z pełnymi uprawnieniami potrafią realnie skasować pliki.

---

## 2. Dostępność cyfrowa (fundament)

### Definicja i skala

- Dostępność = możliwość korzystania z aplikacji **całkowicie, samodzielnie i bezpiecznie** przez osoby z różnymi ograniczeniami (wzrok, słuch, motoryka, dysleksja, seniorzy, ograniczenia krótkotrwałe).
- Skala w Polsce: **3–5 mln** osób z niepełnosprawnościami (dane ZUS); to realna grupa użytkowników, nie margines.
- To **wymóg prawny** (nie tylko dobra praktyka) — brak zgodności = ryzyko kar finansowych.

### Prawo

- **European Accessibility Act (EAA)** wymusza dostępność m.in. serwisów publicznych, e-commerce, aplikacji bankowych, automatów biletowych.
- Docelowy, praktyczny minimalny poziom: **WCAG 2.1 AA** (dla hackathonu jako cel realistyczny).
- WCAG — 4 filary: **dostrzegalność, funkcjonalność, zrozumiałość, solidność.**

### Zasada nr 1: semantyczny HTML przed ARIA

- Nagłówki H1–H6 w logicznej hierarchii — **nie używać nagłówków jako narzędzia stylizacji** (do powiększania tekstu); do tego służy CSS.
- Poprawne elementy: `table` dla tabel, `ul`/`ol` dla list, `label` powiązany z `input` w formularzach.
- **ARIA to dodatek, nie fundament**: „najpierw dobry kod, potem ARIA" — nie używać ARIA, jeśli da się to osiągnąć natywnym HTML-em.

### Nawigacja klawiaturowa

- **Tab / Shift+Tab** — podstawowa metoda nawigacji; każdy element aktywny musi mieć **widoczny fokus**.
- Kolejność tabulacji = logiczna kolejność treści (dla PL: lewa→prawa).
- **Enter/Space** zatwierdzają, **Escape** zamyka.
- **Skip to content** — pierwszy element w kodzie (niewidoczny wzrokowo, dostępny Tabem), przenosi od razu do głównej treści. Obowiązkowy element na hackathonie.

### Pułapki do unikania

- Modale niezamykalne klawiaturą i cookie bannery mogą tworzyć **pułapkę klawiaturową** (klawisz Tab nie wypuszcza z elementu).
- Klasyczne obrazkowe CAPTCHA są zwykle niedostępne — preferować alternatywy audio/proste zadania.
- **Nakładki „dostępności" (accessibility widgets/overlays) — unikać.** Często nadpisują ustawienia systemowe i w testach z realnymi użytkownikami czytników ekranu nie działają zgodnie z obietnicą.

### Multimedia

- **Closed captions (CC)** — zawierają też opisy dźwięków, nie tylko dialogi. Dodawać CC + udostępniać transkrypcję do pobrania; tłumacz języka migowego tam, gdzie to możliwe.

### Testowanie

- **Czytniki ekranu:** NVDA (darmowy, popularny w PL), JAWS (płatny, ~4000$), VoiceOver (macOS/iOS), TalkBack (Android), Narrator (Windows).
- **Narzędzia automatyczne** (np. Wave, wtyczki IDE/przeglądarki) wykrywają tylko **część błędów** — resztę trzeba zweryfikować testem z żywym użytkownikiem/czytnikiem.
- AI może pomóc raportować błędy w kodzie, ale **nie zastępuje** testów z prawdziwą technologią wspomagającą.

### Checklist minimalna na hackathon

Semantyczny HTML → dostępna nawigacja klawiaturą → widoczny fokus → skip link → podstawowe labelki formularzy → napisy/transkrypcje tam, gdzie jest multimedia.

---

## 3. Design systemy i tokenizacja

### Tokeny — trzy poziomy

1. **Prymitywy** — surowe wartości: kolory, spacingi, radiusy, typografia.
2. **Semantyczne** — mapowanie prymitywów do ról: `background`, `content`, `border`.
3. **Kontekstowe** — nazwy typu `BG.warning.access` dla powtarzalnego, jednoznacznego użycia.

**Zasada kierunku:** iść od ogółu do szczegółu — **UI element → priorytet → stan**.
**Zasada minimalizmu:** minimalizować liczbę tokenów — mniej tokenów = łatwiejsza implementacja. Zacząć od kilku wariantów, rozszerzać w razie potrzeby.
Skala metryczna np. 50–100–500, z możliwością wstawiania wartości pośrednich.

### Komponenty

- Struktura: **atomy → molekuły → organizmy** (button, input, badge, karta, modal).
- Każdy komponent ma: **warianty, rozmiary, stany** (hover, focus, active).
- Komponenty muszą być **opisane i labelkowane** tak, żeby przy implementacji nie było pytań.

### Figma jako repozytorium

- Struktura: strony z guidelines, paleta kolorów, spacingi, radiusy, typografia.
- **Dev/Inspect mode** — inspekcja komponentów, gotowy kod CSS/JSON dla developerów.

### Dostępność w designie (Stark / Wave)

- Testy kontrastu wtyczką (np. Stark — Contrast Meter): **4.5:1** dla zwykłego tekstu, mniej dla dużego tekstu; WCAG AA jako cel.
- Przy niskim kontraście: podnieść kontrast lub zmienić kolor tekstu — decyzja projektowa, nie do pominięcia.

### Tempo pracy na hackathonie (orientacyjne)

- Pierwsze tokeny: kilkanaście–kilkadziesiąt minut.
- One-shot prompt (jeśli wiadomo czego się chce): **5–15 min** na wygenerowanie sensownego zestawu.
- Priorytet: **użyteczność i spójność > dopracowanie wizualne.**

---

## 4. AI ↔ Figma ↔ Kod: workflow (Antigravity, MCP)

### Antigravity — czym jest

- Klient-agent podobny do VS Code, z dostępem do wielu modeli i możliwością podłączania zewnętrznych agentów.
- Modele dostępne w demo: **Gemini 3.5 Flash, Claude Sonnet 4.6, Claude Opus 4.6** (oraz Kodex/Kodexa jako uzupełnienie — dobre do zadań graficznych).
- Google Antigravity daje darmowe tokeny na konto Google — **nie przelogowywać kont**, by sztucznie zwiększać limity (uznane za nieetyczne).

### Integracja z Figma przez MCP — kroki

1. Zainstalować **Figma Desktop** (warunek konieczny).
2. Dodać serwer **MCP (Model Context Protocol)** w Connectors: oficjalny **Figma Developer MCP** albo nieoficjalny plugin.
3. Utworzyć **Personal Access Token** w Figma (Settings → Personal Access Tokens).
4. Nieoficjalny plugin (przykład komendy):
   ```
   npx -y figma-developer-mcp --figma-api-key=YOUR-KEY
   ```
   Kompresuje odpowiedzi API Figmy o **blisko 90%**, żeby nie zapychać kontekstu modelu.
5. Antigravity nasłuchuje MCP i łączy się z projektem Figma.

### Efekt końcowy (przykład: komponent „switch")

Wskazanie pliku w Figma + prompt do agenta → automatyczne wygenerowanie: **SVG, HTML, CSS, animacja, gotowy snippet komponentu** → push na GitHub Pages.

### Dwa oficjalne podejścia (wg prezentacji Przemka Nowaka) vs nieoficjalne

- Podejście oficjalne przez Figmę: zgodne ze sztuką, bezpieczne — ale **wymaga konta płatnego** (lub ma ograniczenia).
- Podejście „nie chcę płacić za Figmę": nieoficjalny plugin `figma-developer-mcp` z własnym kluczem API — działa, ale świadomie mniej „oficjalne" (autor sam zaznacza: „nie powinienem tego robić").
- Skrajna opcja: w ogóle bez Figmy / bez MCP — też częściowo działa (np. odręczny szkic → zdjęcie → Kodex generuje design).

### Oszczędzanie tokenów

- Używać wtyczek kompresujących odpowiedzi API.
- Ograniczać liczbę zapytań (GET) do Figmy — **6 darmowych GET-ów z kompresją** zwykle wystarcza na pobranie projektu.

### Alternatywne/uzupełniające narzędzia (wspomniane, nie wymyślać nowych)

- **Kodex/Kodexa** — dobre do zadań graficznych i analiz designu, można łączyć z Antigravity.
- **Google AI Studio / Build** — generuje proste aplikacje TS/CSS, dobre do prototypów.
- **Cloud Design / Cloud Decoder** — automatyzacja brandbooków, eksport do TS/CSS, generowanie modeli 3D/konfiguratorów (przykład B2B: konfigurator mebli, schody).

### Scenariusze pracy z design systemem

- **Scenariusz A (brak designera):** wygenerować minimalne tokeny + komponenty szybko od zera (typowy hackathon).
- **Scenariusz B (istniejący design system):** zmapować istniejące komponenty do tokenów, przeklikać w Figma/Dev Mode.
- Zmiana w design systemie **propaguje się** na wygenerowane widoki (source of truth).

---

## 5. Bezpieczeństwo i higiena pracy z agentami

- **Traktuj każdy klucz API/token jako sekret.** Nigdy nie publikować w publicznych repozytoriach. Wklejony do agenta kod/klucz uznawać za „spalony" (skompromitowany).
- Rozważyć **serwer pośredniczący (proxy/REST)** między agentem a zewnętrznym API — agent nie widzi wtedy bezpośrednio kluczy.
- **Uprawnienia agentów ograniczać do minimum.** Pełne/„danger" uprawnienia mogą prowadzić do realnych szkód (np. agent kasujący pliki, żeby zrobić miejsce na dysku).
- **Ryzyko npm / „slopsquatting":** nie instalować niezweryfikowanych konektorów/pakietów bez sprawdzenia — mogą być złośliwe.
- Testować agenty w izolowanym środowisku (VM/Docker/stary sprzęt), z ograniczonymi uprawnieniami i bez ujawniania kluczy.
- Przy wielu równoległych agentach możliwy jest **brak RAM** — ograniczać liczbę agentów działających jednocześnie.
- Zasada ogólna: **ufaj, ale sprawdzaj i weryfikuj źródła** generowane przez AI, zanim coś opublikujesz (spójnie z dniem 4).

---

## 6. Case study: Horse Match (MVP w praktyce)

- Cel demo: prosta webowa aplikacja „swipe" (styl Tinder) do dopasowania konia do jeźdźca.
- Stack demo: **Vite + React + TypeScript**, agenty: Claude, Kodex, Gemini, środowisko: Antigravity / AI Studio.
- **`agents.md`** — obowiązkowy plik, który każdy agent czyta przed działaniem: role agentów, reguły orkiestracji, kiedy odpalać subagentów, poziomy wysiłku. (Analogiczna koncepcja do „Agent MD" z dnia 4 — jeden plik z zasadami statycznymi.)
- **Plan developmentu** — osobny plik Markdown, prosty i szybki (kamienie milowe, podejście Scrum-like), zapisany w repo _przed_ rozpoczęciem właściwej implementacji.
- Wniosek z demo: agenty potrafią one-shotowo wygenerować front + prostą logikę + eksport danych (np. Excel) — ale **walidacja źródeł i testy pozostają zadaniem człowieka**.
- Rekomendowana architektura produkcyjna: frontend jw. + **backend low-code (np. Vercel + Supabase)** jako szybkie, bezpieczne rozwiązanie, albo własny serwer pośredniczący bez ujawniania kluczy agentom.
- Uwaga kosztowa: prompty **po polsku kosztują więcej tokenów** (tłumaczenie w obie strony) — agenty „myślą" domyślnie po angielsku.
