import { DOCUMENT } from '@angular/common';
import { Injectable, computed, effect, inject, signal } from '@angular/core';

/** Discrete text-scale steps, expressed as a percentage of the browser default. */
const FONT_STEPS = [100, 115, 130, 150] as const;
const STORAGE_CONTRAST = 'heureka.a11y.contrast';
const STORAGE_DARK = 'heureka.a11y.dark';
const STORAGE_FONT_STEP = 'heureka.a11y.fontStep';

/**
 * Holds user accessibility preferences (high contrast, text enlargement),
 * persists them, and reflects them onto the document root so global SCSS can
 * react. Signal-based so any component can bind to the current state.
 */
@Injectable({ providedIn: 'root' })
export class AccessibilityService {
  private readonly doc = inject(DOCUMENT);
  private readonly root = this.doc.documentElement;

  readonly highContrast = signal(this.readBool(STORAGE_CONTRAST));
  readonly darkMode = signal(this.readBool(STORAGE_DARK));
  private readonly fontStepIndex = signal(this.readFontStepIndex());

  readonly fontStep = computed(() => this.fontStepIndex() + 1);
  readonly fontStepCount = FONT_STEPS.length;
  readonly canEnlarge = computed(() => this.fontStepIndex() < FONT_STEPS.length - 1);
  readonly canShrink = computed(() => this.fontStepIndex() > 0);

  constructor() {
    effect(() => {
      const on = this.highContrast();
      this.root.classList.toggle('a11y-contrast', on);
      this.persist(STORAGE_CONTRAST, String(on));
    });

    effect(() => {
      const on = this.darkMode();
      this.root.classList.toggle('a11y-dark', on);
      this.persist(STORAGE_DARK, String(on));
    });

    effect(() => {
      const index = this.fontStepIndex();
      this.root.style.fontSize = `${FONT_STEPS[index]}%`;
      this.persist(STORAGE_FONT_STEP, String(index));
    });
  }

  toggleContrast(): void {
    this.highContrast.update((v) => !v);
  }

  toggleDark(): void {
    this.darkMode.update((v) => !v);
  }

  enlargeText(): void {
    this.fontStepIndex.update((i) => Math.min(i + 1, FONT_STEPS.length - 1));
  }

  shrinkText(): void {
    this.fontStepIndex.update((i) => Math.max(i - 1, 0));
  }

  private readFontStepIndex(): number {
    const raw = Number(this.read(STORAGE_FONT_STEP));
    return Number.isInteger(raw) && raw >= 0 && raw < FONT_STEPS.length ? raw : 0;
  }

  private readBool(key: string): boolean {
    return this.read(key) === 'true';
  }

  private read(key: string): string | null {
    try {
      return this.doc.defaultView?.localStorage.getItem(key) ?? null;
    } catch {
      return null;
    }
  }

  private persist(key: string, value: string): void {
    try {
      this.doc.defaultView?.localStorage.setItem(key, value);
    } catch {
      // Storage unavailable (private mode / SSR) — preference stays in-memory.
    }
  }
}
