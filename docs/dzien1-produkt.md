# CONTEXT.md — Hackathon Dzień 1 (Dicsovery & MVP scoping)

> Ten plik jest źródłem prawdy o procesie, metodach i artefaktach z warsztatów przygotowujących do hackathonu.
> Agencie: nie wymyślaj metod ani nazw od nowa — trzymaj się tego, co poniżej. Jeśli czegoś brakuje, powiedz o tym, nie zgaduj.

---

## 1. Cel i sytuacja

- Przygotowanie do **hackathonu (sobota)**. Finalnym artefaktem projektu jest **backlog** opisujący zadania konieczne do osiągnięcia MVP/API.
- Praca zespołowa: **grupy ~3-osobowe + lider + facylitator**. Lider jest osobą pierwszego kontaktu (niekoniecznie pisze kod, opiekuje się zespołem, jego zgłoszenia mają priorytet).
- Narzędzia warsztatowe: **Miro** (boardy, canvasy, sticky notes), materiały/slajdy na **Discordzie**. Prace udostępniane jako screeny na kanale zespołu.
- Praca po sesji dokańczana **asynchronicznie**.

---

## 2. Główna filozofia (nie łamać tych zasad)

1. **Najpierw problem, potem technologia.** Zaczynaj od opisu problemu, nie od wyboru LLM/AI. Klient mówiący „chcę AI" (analogia: „chcę prąd") potrzebuje odkrycia celu — ważniejsze „po co" niż samo rozwiązanie.
2. **Najprostsze skuteczne rozwiązanie.** Często reguły / klasyczny ML / model statystyczny są lepsze niż od razu LLM (mniejsze koszty, mniej ryzyka). Eskaluj do zaawansowanych modeli dopiero gdy trzeba.
3. **Mierzalny efekt biznesowy.** Każde wdrożenie ma mieć benchmark, metrykę, ROI. Bez tego nie da się ocenić efektu.
4. **Jakość danych.** „Gówno na wejściu, gówno na wyjściu." Czyszczenie danych zajmuje najwięcej czasu; wymagana świeżość i aktualizacje w produkcji.
5. **Odpowiedzialność i nadzór.** Modele (zwłaszcza LLM) halucynują → potrzebny monitoring, ownership, procedury naprawcze. Decyzje krytyczne zostawić ludziom.
6. **Wartość dla użytkownika.** 9 na 10 startupów upada z powodu braku finansowania lub braku popytu → buduj rozwiązania z realną wartością.

---

## 3. Proces (kolejność pracy)

Discovery → Persona → MVP (user-story mapping) → Modelowanie procesu (BPMN / Event Storming) → KPI → **Backlog**.

Kolejność ćwiczeń w Miro: **Problem Statement Canvas → Persona → modelowanie procesu → backlog.**

### 3.1 Discovery — definicja problemu

- Etap 1: zebrać kontekst — kiedy i gdzie występuje problem.
- Etap 2: zidentyfikować dokładny problem i mierzalne skutki.
- Technika **5x „dlaczego"** — docieranie do rzeczywistej potrzeby.
- Soft skills: nie tłumaczyć od razu technologii, zadawać pytania otwarte, mówić językiem korzyści (oszczędność czasu/pieniędzy, poprawa procesów).
- Wynik: **jedno zdanie** definiujące problem i użytkownika.

**Broadband / Problem Statement Canvas — 6 kafelków:**

1. kontekst
2. sytuacje
3. dokładny problem
4. kogo dotyczy
5. alternatywy (obecne rozwiązania)
6. mierzalne skutki / emocje

### 3.2 Persona

- Persona = fikcyjna, oparta na danych postać reprezentująca archetyp klienta.
- Rozróżnić **B2C** (pojedynczy użytkownik) vs **B2B** (wiele osób, rozmyte płatności/decyzje). (Przykład: Instagram — konta prywatne vs firmowe.)
- Pola persony: **profil, cel/motywacja, bóle/frustracje, zachowania/nawyki, kontekst użycia.**
- AI może przyspieszyć wypełnienie persony i generowanie pytań do rozmów, ale **nie zastąpi** rozmów z prawdziwymi użytkownikami ani empatii.

### 3.3 MVP i user-story mapping

- **MVP = najmniejsza rzecz walidująca hipotezę.** Liczy się szybkie potwierdzenie wartości, nie pełna funkcjonalność.
- Iteracja typu **deskorolka → rower → auto** (nie budować od razu auta). Przykład: Google Form jako „deskorolka".
- Case'y: Dropbox (jeden feature — synchronizacja plików), Airbnb (prosty HTML z listą pokoi, iteracje do 500 userów).
- **User-story mapping:** aktywności użytkownika (epiki) → user stories → priorytety (góra–dół). Układ lewa→prawa = chronologia korzystania.
- Oddzielić kreską elementy MVP; **każda kreska = kolejny release**. Zasada: z każdego epika **po 1 minimalnej user story** do pierwszego release.

### 3.4 Modelowanie procesu (As-Is / To-Be)

- **BPMN** — standardowa notacja: swimlane, activity (kwadraty), sequence flow, gateway. Najpierw „brudny" **As-Is**, znajdź problemy, zaprojektuj **To-Be**.
- Szukaj: wąskich gardeł, shadow IT, ręcznych obliczeń, miejsc wymagających danych.
- **Event Storming** (warsztat): najpierw **pomarańczowe zdarzenia** (co już się wydarzyło) — najważniejsze karteczki. Potem **reverse narrative** (opowieść od tyłu) → luki i niespójności. Dodatkowe: aktorzy, reguły, hotspoty.
- Case OZE Solutions: wąskim gardłem był CFO (Marek) niedostarczający stanu konta → To-Be: system cyklicznie pobiera stan konta, operation manager (Ania) widzi aktualne środki. Efekt: ~4 godz./tydz. oszczędności, mniej stresu.

### 3.5 KPI i uzasadnienie biznesowe

- Na podstawie luk wyznacz KPI: czas oszczędzony, redukcja wąskich gardeł, eliminacja shadow IT.
- Pokazać oszczędność czasu + kosztów oraz wpływ emocjonalny (mniej stresu decydentów).

### 3.6 Backlog (artefakt końcowy)

- Backlog = „rameczka" zawierająca całą wiedzę potrzebną do pracy nad zadaniem.
- Powstaje z user-story mappingu + zamodelowanego procesu → lista konkretnych zadań potrzebnych do osiągnięcia **API/MVP**.
- To finalny, kluczowy artefakt. Ma być czytelnym planem pracy na hackathon.

---

## 4. Monetyzacja i model biznesowy

- Najpopularniejszy model: **subskrypcja** (miesięcznie/rocznie). Alternatywy: pay-per-use, płatne funkcje premium.
- **Uwaga na koszty AI (tokeny):** subskrypcja może nie wystarczyć, jeśli user zużywa dużo tokenów. Przykład: chat bankowy dostający niezwiązane pytania → ryzyko kosztów. Rozwiązanie: ograniczać zakres pytań (ściśle sparametryzowany LLM).
- Koszty wdrożenia/utrzymania często przewyższają oczekiwania klienta — przemyśleć opłacalność na produkcji.

## 5. Segmentacja klientów (strategia sprzedaży)

- **Innowatorzy:** kupują szybko, niskie budżety → dobre do testów i feedbacku.
- **Early adopters:** potrzebują dowodów, customowe wdrożenia → da się zarabiać.
- **Early majority (pragmatycy):** oczekują mierzalnego ROI, trudniej przekonać.
- **Late majority:** bardzo trudni.
- Rola: innowatorzy/early adopters = testy i referencje; early majority = skalowanie i ROI. Zdobyć referencje i ROI **przed** skalowaniem.

---

## 6. Rekomendacje techniczne (checklista dla rozwiązania)

- Uporządkować procesy i dane **przed** automatyzacją.
- Zmiany punktowo tam, gdzie dają największy wpływ.
- Preferować ograniczone, celowane użycie LLM (ściśle sparametryzowane), nie „dajcie LLM-owi wszystko".
- Ustalić benchmark i metrykę **przed** wdrożeniem; mierzyć efekt po.
- Ownership + monitoring + procedury naprawcze przed produkcją.
- Nie zostawiać modelu „samopas" przy decyzjach krytycznych.

---

## 7. Słownik artefaktów (czego agent ma się trzymać)

| Artefakt                             | Do czego                   | Kluczowe elementy                                             |
| ------------------------------------ | -------------------------- | ------------------------------------------------------------- |
| Problem Statement / Broadband Canvas | Definicja problemu         | 6 kafelków; wynik = 1 zdanie problemu                         |
| Persona                              | Dla kogo budujemy          | profil, cel/motywacja, bóle, zachowania, kontekst; B2C vs B2B |
| User-story mapping                   | Wyznaczenie MVP            | epiki → stories → priorytety; kreska = release                |
| BPMN (As-Is / To-Be)                 | Analiza procesu z biznesem | swimlane, activity, flow, gateway                             |
| Event Storming                       | Warsztat dev+biznes        | pomarańczowe zdarzenia, reverse narrative, hotspoty           |
| KPI                                  | Uzasadnienie biznesowe     | czas, koszt, wąskie gardła, shadow IT                         |
| **Backlog**                          | **Artefakt końcowy**       | lista zadań do MVP/API z mappingu + procesu                   |
