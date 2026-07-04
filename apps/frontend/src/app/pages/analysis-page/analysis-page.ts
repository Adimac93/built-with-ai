import {
  ChangeDetectionStrategy,
  Component,
  ElementRef,
  inject,
  signal,
  viewChild,
} from '@angular/core';
import { PipelineDiagramComponent } from '../../components/pipeline-diagram/pipeline-diagram';
import { AnalysisEngine } from '../../services/analysis-engine';
import { DiagramHighlightService } from '../../services/diagram-highlight.service';
import { SpeechTranscriptionService } from '../../services/speech-transcription.service';
import { EVAL_LABELS, EvalMode, Trail } from '../../models/trail.model';

const PHASES = [
  'Krok 1/5 — normalizacja problemu…',
  'Krok 2/5 — formułowanie kontradykcji…',
  'Krok 3/5 — generowanie kandydatów…',
  'Krok 4/5 — ewaluacja…',
];
const PHASE_INTERVAL_MS = 8000;
const MAX_RECORDING_MS = 55_000;

const PHASE_NODES: string[][] = [
  ['problem_normalizer'],
  ['contradiction_extractor', 'triz_lookup', 'criteria_extractor'],
  ['candidate_generators', 'triz_generator', 'scamper_generator'],
  ['evaluator'],
];
const COMPLETION_NODES = ['choice_selector', 'trail_assembler'];

@Component({
  selector: 'app-analysis-page',
  imports: [PipelineDiagramComponent],
  templateUrl: './analysis-page.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AnalysisPage {
  private readonly engine = inject(AnalysisEngine);
  private readonly highlight = inject(DiagramHighlightService);
  private readonly speech = inject(SpeechTranscriptionService);
  private readonly resultsHeading =
    viewChild.required<ElementRef<HTMLHeadingElement>>('wynikiH');
  private readonly problemField =
    viewChild.required<ElementRef<HTMLTextAreaElement>>('problemField');
  private recorder: MediaRecorder | null = null;
  private audioStream: MediaStream | null = null;
  private audioChunks: Blob[] = [];
  private recordingStopTimer: ReturnType<typeof setTimeout> | null = null;

  protected readonly evalLabels = EVAL_LABELS;

  protected readonly problem = signal('');
  protected readonly evalMode = signal<EvalMode>('rubryka');
  protected readonly running = signal(false);
  protected readonly recording = signal(false);
  protected readonly transcribing = signal(false);
  protected readonly status = signal('');
  protected readonly speechStatus = signal('');
  protected readonly error = signal('');
  protected readonly trail = signal<Trail | null>(null);
  protected readonly processing = this.highlight.processing;

  protected speechInputAvailable(): boolean {
    return (
      typeof navigator !== 'undefined' &&
      Boolean(navigator.mediaDevices?.getUserMedia) &&
      typeof MediaRecorder !== 'undefined'
    );
  }

  protected async toggleRecording(): Promise<void> {
    if (this.recording()) {
      this.recorder?.stop();
      return;
    }

    if (this.running() || this.transcribing()) return;

    if (!this.speechInputAvailable()) {
      this.error.set('Ta przeglądarka nie obsługuje nagrywania z mikrofonu.');
      return;
    }

    this.error.set('');
    this.speechStatus.set('');

    try {
      this.audioStream = await navigator.mediaDevices.getUserMedia({
        audio: {
          echoCancellation: true,
          noiseSuppression: true,
        },
      });
      this.audioChunks = [];

      const mimeType = this.preferredAudioMimeType();
      const recorder = new MediaRecorder(
        this.audioStream,
        mimeType ? { mimeType } : undefined,
      );

      recorder.ondataavailable = (event) => {
        if (event.data.size > 0) this.audioChunks.push(event.data);
      };
      recorder.onerror = () => {
        this.error.set('Nagrywanie zostało przerwane przez przeglądarkę.');
        this.recording.set(false);
        this.clearRecordingStopTimer();
        this.stopAudioStream();
      };
      recorder.onstop = () => {
        void this.transcribeRecording(recorder.mimeType || mimeType || 'audio/webm');
      };

      this.recorder = recorder;
      recorder.start();
      this.recordingStopTimer = setTimeout(() => {
        if (recorder.state === 'recording') recorder.stop();
      }, MAX_RECORDING_MS);
      this.recording.set(true);
      this.speechStatus.set('Nagrywanie z mikrofonu...');
    } catch {
      this.recording.set(false);
      this.clearRecordingStopTimer();
      this.stopAudioStream();
      this.error.set(
        'Nie udało się uruchomić mikrofonu. Sprawdź uprawnienia przeglądarki.',
      );
    }
  }

  protected async run(event: Event): Promise<void> {
    event.preventDefault();
    const problem = this.problem().trim();
    if (!problem) {
      this.error.set('Opisz problem w polu 01, zanim uruchomisz analizę.');
      this.problemField().nativeElement.focus();
      return;
    }

    this.error.set('');
    this.running.set(true);
    this.trail.set(null);

    this.highlight.reset();
    this.highlight.processing.set(true);
    this.highlight.setActive(PHASE_NODES[0]);
    this.highlight.focusOn(PHASE_NODES[0][0]);
    this.status.set(PHASES[0]);

    let phaseIdx = 1;
    const timer = setInterval(() => {
      if (phaseIdx < PHASES.length) {
        this.status.set(PHASES[phaseIdx]);
        const next = PHASE_NODES[phaseIdx];
        this.highlight.completePrevious(next);
        this.highlight.setActive(next);
        this.highlight.focusOn(next[0]);
        phaseIdx++;
      }
    }, PHASE_INTERVAL_MS);

    let success = false;
    try {
      const trail = await this.engine.solve(problem, this.evalMode());
      success = true;
      this.trail.set(trail);
      this.status.set('Analiza zakończona');
      this.highlight.completePrevious(COMPLETION_NODES);
      this.highlight.setActive(COMPLETION_NODES);
      this.highlight.focusOn(COMPLETION_NODES[0]);
      setTimeout(() => {
        this.highlight.markDone(COMPLETION_NODES);
        this.highlight.processing.set(false);
        this.resultsHeading().nativeElement.focus();
      }, 1500);
    } finally {
      clearInterval(timer);
      this.running.set(false);
      if (!success) this.highlight.reset();
    }
  }

  private async transcribeRecording(mimeType: string): Promise<void> {
    this.recording.set(false);
    this.clearRecordingStopTimer();
    this.stopAudioStream();
    this.recorder = null;

    const audio = new Blob(this.audioChunks, { type: mimeType });
    this.audioChunks = [];

    if (audio.size === 0) {
      this.speechStatus.set('');
      this.error.set('Nie nagrano dźwięku. Spróbuj jeszcze raz.');
      return;
    }

    this.transcribing.set(true);
    this.speechStatus.set('Transkrypcja nagrania...');

    try {
      const transcript = await this.speech.transcribe(audio);
      if (!transcript) {
        this.error.set('Nie rozpoznano tekstu w nagraniu.');
        return;
      }

      const current = this.problem().trim();
      this.problem.set(current ? `${current}\n${transcript}` : transcript);
      this.speechStatus.set('Tekst z mikrofonu dodany do zgłoszenia.');
      this.problemField().nativeElement.focus();
    } catch {
      this.error.set(
        'Nie udało się rozpoznać mowy. Sprawdź konfigurację Google Cloud Speech-to-Text.',
      );
      this.speechStatus.set('');
    } finally {
      this.transcribing.set(false);
    }
  }

  private preferredAudioMimeType(): string {
    const types = [
      'audio/webm;codecs=opus',
      'audio/webm',
      'audio/ogg;codecs=opus',
    ];
    return types.find((type) => MediaRecorder.isTypeSupported(type)) ?? '';
  }

  private stopAudioStream(): void {
    this.audioStream?.getTracks().forEach((track) => track.stop());
    this.audioStream = null;
  }

  private clearRecordingStopTimer(): void {
    if (!this.recordingStopTimer) return;
    clearTimeout(this.recordingStopTimer);
    this.recordingStopTimer = null;
  }
}
