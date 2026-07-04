## Wskazówki do oceny w poszczególnych kategoriach

### Day 1

### Day 2

### Day 3

1. **Monorepo (Nx)**  
   Organizacja projektu jako monorepo Nx, obejmującego zarówno frontend, jak i backend. Oceniana będzie poprawność struktury projektu oraz podział aplikacji na moduły i biblioteki.
2. **Clean code**
3. **Architektura warstwowa**  
   Zastosowanie architektury warstwowej zgodnej z materiałem z warsztatów (wyraźny podział odpowiedzialności, separacja logiki biznesowej od prezentacji i infrastruktury).

### Day 4

1. **Ewaluacja systemu**

Oceniamy, czy zespół zbudował coś, co da się odpalić komendą i sprawdzić — narzędzie (CLI/API), które na wrzucony problem zwraca ustrukturyzowaną odpowiedź z każdym krokiem; punktacja 50/50: połowa za inputy/scenariusze (od jednego problemu po gotową listę z trudnymi przypadkami), połowa za metryki ewaluacji (od ręcznej oceny po automatyczne, mierzące kodem to, co policzalne). Nie trzeba robić wszystkiego — mniej znaczy niżej na drabince, ale próg „czy w ogóle działa” jest po stronie inputów, więc bez działającego narzędzia nie ma czego oceniać.  
Kryteria oceny — część „Ewaluacja” (20 pkt / 50-50)

Pytanie główne: skąd wiemy, że system działa?  
\- Chcemy narzędzie, które odpalimy komendą (CLI albo API), wrzucimy problem i zobaczymy odpowiedź.  
\- Output ma być ustrukturyzowany — widać każdy krok (kontradykcja → pomysły → ocena → wybór), a nie jeden ładny akapit.  
\- Oceniamy silnik, który da się wywołać skryptem, nie ładny interfejs.

Część 1 — Inputy / scenariusze (50%)  
„Co wrzucili do systemu i czy to w ogóle działa?”  
\- Próg: odpalam 1 komendą, wrzucam 1 problem, dostaję pełną odpowiedź bez wywalenia.  
\- Więcej: zestaw kilku problemów, nie tylko ten z demo.  
\- Najwięcej: inputy jako gotowa lista/plik \+ przypadki trudne (wejście niejasne, śmieciowe, problem spoza siódemki).

Część 2 — Metryki ewaluacji (50%)  
„Jak sprawdzili, że odpowiedzi są dobre?”  
\- Próg: jest jakakolwiek ewaluacja i umieją nazwać metrykę (choćby „przeczytaliśmy i oceniliśmy ręcznie”).  
\- Więcej: metryki automatyczne (skrypt, nie oko), przypięte do kroków — kontradykcja sensowna? pomysły wynikają z matrycy? wybór \= najwyżej oceniony kandydat?  
\- Najwięcej: kod tam, gdzie odpowiedź jest znana (lookup w matrycy, wybór \= max oceny), a LLM/człowiek tylko do tego, co naprawdę subiektywne („czy pomysł jest dobry”). Metryki odpalają się po całej liście inputów z Części 1\.

Zasady dla oceniających

\- Nie trzeba robić wszystkiego — mniej zrobione \= niżej na drabince \= mniej punktów.  
\- Próg „czy działa” jest w Części 1, nie w Części 2:  
\- zrobili metryki, ale nic się nie odpala → mało punktów (nie ma czego sprawdzać),  
\- odpala się, ale nie ewaluowali → \~połowa.  
\- Złota zasada Części 2: mierz kodem to, co policzalne; człowieka/LLM zostaw na subiektywne. Kto wali „LLM oceń 1–10” na wszystko — zatrzymuje się w środku drabinki.

### Day 5

1. **Implementacja funkcjonalności**  
   Stopień realizacji założeń projektu, jakość implementacji, poprawność działania aplikacji oraz czytelność kodu. (full stack: frontend \+ backend \+ baza danych)
2. **Wdrożenie aplikacji**  
   Poprawne wdrożenie (deploy) aplikacji do chmury wraz z możliwością uruchomienia i przetestowania rozwiązania przez jury.

### Pitch

1. **Prezentacja projektu**  
   Sposób przedstawienia rozwiązania przed jury: zwięzłość, pokaz działania aplikacji, uzasadnienie decyzji projektowych oraz zmieszczenie się w wyznaczonym limicie czasu.
