import { Injectable, signal } from '@angular/core';

export type NodeHighlight = 'idle' | 'active' | 'done';

@Injectable({ providedIn: 'root' })
export class DiagramHighlightService {
  private readonly _state = signal<Map<string, NodeHighlight>>(new Map());

  readonly state = this._state.asReadonly();

  /** True while the pipeline runs — drives the fullscreen popup. */
  readonly processing = signal(false);

  /**
   * Node the viewport should keep centered. Bumped via a monotonic token so
   * re-focusing the same node id still triggers the viewport effect.
   */
  readonly focus = signal<{ nodeId: string; token: number } | null>(null);
  private focusToken = 0;

  focusOn(nodeId: string): void {
    this.focus.set({ nodeId, token: ++this.focusToken });
  }

  setActive(nodeIds: string[]): void {
    const next = new Map(this._state());
    for (const id of nodeIds) next.set(id, 'active');
    this._state.set(next);
  }

  completePrevious(nodeIds: string[]): void {
    const next = new Map(this._state());
    for (const [id, status] of next) {
      if (status === 'active' && !nodeIds.includes(id)) next.set(id, 'done');
    }
    this._state.set(next);
  }

  markDone(nodeIds: string[]): void {
    const next = new Map(this._state());
    for (const id of nodeIds) next.set(id, 'done');
    this._state.set(next);
  }

  reset(): void {
    this._state.set(new Map());
    this.focus.set(null);
    this.processing.set(false);
  }

  highlightOf(nodeId: string): NodeHighlight {
    return this._state().get(nodeId) ?? 'idle';
  }
}
