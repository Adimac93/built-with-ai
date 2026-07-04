/* Heureka — frontend prototypu: mock silnika za jednym interfejsem.
   Docelowo runAnalysis() podmieniamy na wywołanie API NestJS. */

const form = document.getElementById('analysis-form');
const problemInput = document.getElementById('problem');
const errorBox = document.getElementById('form-error');
const statusBox = document.getElementById('status');
const runBtn = document.getElementById('run-btn');
const out = document.getElementById('trail-out');
const resultsHeading = document.getElementById('wyniki-h');

const EVAL_LABELS = {
  rubryka: 'Rubryka ważona',
  pugh: 'Macierz Pugha',
  pary: 'Porównanie parami',
};

const METHOD_LABELS = { triz: 'TRIZ', scamper: 'SCAMPER', morph: 'Analiza morfologiczna' };

form.addEventListener('submit', (e) => {
  e.preventDefault();
  const problem = problemInput.value.trim();
  const method = form.querySelector('input[name="method"]:checked').value;
  const evalMode = form.querySelector('input[name="eval"]:checked').value;

  if (!problem) {
    return showError('Opisz problem w polu 01, zanim uruchomisz analizę.', problemInput);
  }
  hideError();
  runAnalysis(problem, [method], evalMode);
});

function showError(msg, focusEl) {
  errorBox.textContent = msg;
  errorBox.hidden = false;
  if (focusEl) focusEl.focus();
}
function hideError() {
  errorBox.hidden = true;
  errorBox.textContent = '';
}

/* ── mock silnika ── */

function runAnalysis(problem, methods, evalMode) {
  runBtn.disabled = true;
  out.innerHTML = '';
  const phases = [
    'Krok 1/5 — normalizacja problemu…',
    'Krok 2/5 — formułowanie kontradykcji…',
    'Krok 3/5 — generowanie kandydatów…',
    'Krok 4/5 — ewaluacja…',
  ];
  phases.forEach((txt, i) => setTimeout(() => (statusBox.textContent = txt), i * 350));
  setTimeout(() => {
    const trail = mockTrail(problem, methods, evalMode);
    statusBox.textContent = 'Analiza zakończona · dane przykładowe (mock silnika)';
    out.innerHTML = renderTrail(trail);
    resultsHeading.focus();
    runBtn.disabled = false;
  }, phases.length * 350 + 400);
}

function mockTrail(problem, methods, evalMode) {
  const candidates = [];
  const add = (method, source, name, desc) => candidates.push({ method, source, name, desc });

  if (methods.includes('triz')) {
    [
      ['Zasada 1 · Segmentacja', 'Podziel kluczowy obiekt lub proces na niezależne części, tak aby zmiana lub awaria jednej nie przenosiła się na całość.'],
      ['Zasada 13 · Odwrócenie', 'Wykonaj działanie odwrotnie: zamień elementy ruchome z nieruchomymi albo odwróć kolejność operacji w procesie.'],
      ['Zasada 35 · Zmiana parametrów', 'Zmień stan, gęstość, elastyczność lub temperaturę kluczowego elementu, aby osłabić źródło sprzeczności.'],
    ].forEach(([source, desc], i) => add('triz', source, `Kandydat T${i + 1}`, desc));
  }
  if (methods.includes('scamper')) {
    [
      ['Operator S · Substitute', 'Zastąp najbardziej zawodny element procesu innym materiałem, mechanizmem lub etapem o tej samej funkcji.'],
      ['Operator C · Combine', 'Połącz dwie istniejące funkcje lub urządzenia w jedno rozwiązanie, które eliminuje słabe ogniwo.'],
      ['Operator A · Adapt', 'Zaadaptuj sprawdzone rozwiązanie z innej branży lub kontekstu do warunków zgłoszonego problemu.'],
    ].forEach(([source, desc], i) => add('scamper', source, `Kandydat S${i + 1}`, desc));
  }
  if (methods.includes('morph')) {
    [
      ['Kombinacja: bariera × adaptacyjny × system', 'Wariant działający na poziomie całego systemu: adaptacyjna bariera reagująca na warunki zamiast stałego zabezpieczenia.'],
      ['Kombinacja: źródło × pasywny × element', 'Wariant pasywny przy źródle problemu: konstrukcyjna zmiana pojedynczego elementu eliminująca przyczynę.'],
      ['Kombinacja: skutek × aktywny × otoczenie', 'Wariant aktywny w otoczeniu: system wykrywa skutek i uruchamia przeciwdziałanie, zanim się rozprzestrzeni.'],
    ].forEach(([source, desc], i) => add('morph', source, `Kandydat M${i + 1}`, desc));
  }

  const SCORES = [84, 71, 77, 63, 69, 58, 52, 66, 47];
  candidates.forEach((c, i) => (c.score = SCORES[i % SCORES.length]));
  const ranked = [...candidates].sort((a, b) => b.score - a.score);

  return {
    problem,
    methods,
    evalMode,
    contradiction: {
      better: { tag: 'Parametr 27', name: 'Niezawodność' },
      worse: { tag: 'Parametr 36', name: 'Złożoność systemu' },
      principles: '1 · 13 · 35',
    },
    candidates,
    ranked,
    winner: ranked[0],
  };
}

/* ── render ── */

const esc = (s) =>
  s.replace(/[&<>"']/g, (ch) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[ch]));

function renderTrail(t) {
  const methodCols = t.methods
    .map((m) => {
      const cards = t.candidates
        .filter((c) => c.method === m)
        .map(
          (c) => `<article class="cand"><span class="cand__src">${esc(c.source)}</span>
            <p class="cand__name">${esc(c.name)}</p><p>${esc(c.desc)}</p></article>`
        )
        .join('');
      return `<div><h4 class="method__title">Metoda — ${METHOD_LABELS[m]}</h4>${cards}</div>`;
    })
    .join('');

  const rows = t.ranked
    .map(
      (c, i) => `<tr${i === 0 ? ' class="win"' : ''}><td>${esc(c.name)}</td><td>${esc(c.source)}</td>
        <td><span class="bar" style="width:${c.score}px"></span><span class="score">${c.score}</span></td></tr>`
    )
    .join('');

  const trizOn = t.methods.includes('triz');

  return `
  <div class="trail">
    <section class="step" aria-labelledby="s1">
      <div class="step__no" aria-hidden="true">1</div>
      <p class="step__kicker">Krok 1 · wejście znormalizowane</p>
      <h3 id="s1">Problem</h3>
      <p class="note">${esc(t.problem)}</p>
    </section>

    <section class="step" aria-labelledby="s2">
      <div class="step__no" aria-hidden="true">2</div>
      <p class="step__kicker">Krok 2 · LLM proponuje, kod waliduje zakres 1–39</p>
      <h3 id="s2">Kontradykcja techniczna</h3>
      <div class="contradiction">
        <div class="param param--better"><span class="param__tag">${t.contradiction.better.tag}</span>
          <p class="param__name">${t.contradiction.better.name}</p></div>
        <span class="vs" aria-hidden="true">×</span>
        <div class="param param--worse"><span class="param__tag">${t.contradiction.worse.tag}</span>
          <p class="param__name">${t.contradiction.worse.name}</p></div>
      </div>
      ${trizOn ? `<p class="matrix">MATRYCA[27, 36] → zasady <b>${t.contradiction.principles}</b> · lookup deterministyczny</p>` : ''}
    </section>

    <section class="step" aria-labelledby="s3">
      <div class="step__no" aria-hidden="true">3</div>
      <p class="step__kicker">Krok 3 · po jednym kandydacie na zasadę / krok metody</p>
      <h3 id="s3">Kandydaci (${t.candidates.length})</h3>
      <div class="methods">${methodCols}</div>
    </section>

    <section class="step" aria-labelledby="s4">
      <div class="step__no" aria-hidden="true">4</div>
      <p class="step__kicker">Krok 4 · metoda: ${EVAL_LABELS[t.evalMode]}, oceny z uzasadnieniami</p>
      <h3 id="s4">Ewaluacja</h3>
      <table class="scores">
        <thead><tr><th scope="col">Kandydat</th><th scope="col">Źródło</th><th scope="col">Wynik łączny</th></tr></thead>
        <tbody>${rows}</tbody>
      </table>
    </section>

    <section class="step" aria-labelledby="s5">
      <div class="step__no" aria-hidden="true">5</div>
      <p class="step__kicker">Krok 5 · deterministyczny argmax — wybiera kod, nie model</p>
      <h3 id="s5">Wybór</h3>
      <div class="choice">
        <p class="stamp" aria-hidden="true">Wybrano</p>
        <h4>${esc(t.winner.name)}</h4>
        <p>Najwyższy wynik łączny (${t.winner.score}/100) w ocenie metodą „${EVAL_LABELS[t.evalMode]}".
        Źródło pomysłu: ${esc(t.winner.source)}. Pełne uzasadnienie każdej oceny — w reasoning trailu.</p>
      </div>
    </section>
  </div>`;
}
