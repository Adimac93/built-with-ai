import { ChangeDetectionStrategy, Component, inject } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import {
  NavigationEnd,
  Router,
  RouterLink,
  RouterLinkActive,
  RouterOutlet,
} from '@angular/router';
import { filter, map } from 'rxjs';

interface SheetMeta {
  readonly ark: string;
  readonly label: string;
  readonly arkusz: string;
}

const SHEET_META: Record<string, SheetMeta> = {
  '/metody': {
    ark: 'ARK. 2/3',
    label: 'KARTA METODOLOGII',
    arkusz: 'Karta metodologii',
  },
  '/zespol': {
    ark: 'ARK. 3/3',
    label: 'KARTA PERSONELU',
    arkusz: 'Karta personelu',
  },
};

const DEFAULT_META: SheetMeta = {
  ark: 'ARK. 1/3',
  label: 'TRIZ · SCAMPER',
  arkusz: 'Analiza',
};

@Component({
  imports: [RouterOutlet, RouterLink, RouterLinkActive],
  selector: 'app-heureka-layout',
  templateUrl: './heureka-layout.html',
  host: { class: 'heureka' },
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class HeurekaLayout {
  private readonly router = inject(Router);

  protected readonly today = new Date().toISOString().slice(0, 10);
  protected readonly meta = toSignal(
    this.router.events.pipe(
      filter((e) => e instanceof NavigationEnd),
      map((e) => SHEET_META[e.urlAfterRedirects.split('?')[0]] ?? DEFAULT_META),
    ),
    { initialValue: SHEET_META[this.router.url] ?? DEFAULT_META },
  );
}
