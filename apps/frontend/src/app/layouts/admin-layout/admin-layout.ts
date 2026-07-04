import {
  ChangeDetectionStrategy,
  Component,
  DOCUMENT,
  computed,
  effect,
  inject,
  signal,
} from '@angular/core';
import { RouterLink, RouterLinkActive, RouterOutlet } from '@angular/router';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatListModule } from '@angular/material/list';
import { MatSidenavModule } from '@angular/material/sidenav';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatTooltipModule } from '@angular/material/tooltip';

interface NavItem {
  readonly label: string;
  readonly path: string;
  readonly icon: string;
}

const THEME_STORAGE_KEY = 'app.theme-mode';

@Component({
  imports: [
    RouterOutlet,
    RouterLink,
    RouterLinkActive,
    MatButtonModule,
    MatIconModule,
    MatListModule,
    MatSidenavModule,
    MatToolbarModule,
    MatTooltipModule,
  ],
  selector: 'app-admin-layout',
  templateUrl: './admin-layout.html',
  styleUrl: './admin-layout.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class AdminLayout {
  private readonly document = inject(DOCUMENT);

  protected readonly title = 'Workshop App';
  protected readonly navItems: readonly NavItem[] = [
    { label: 'Users', path: '/users', icon: 'group' },
    { label: 'Orders', path: '/orders', icon: 'shopping_cart' },
  ];

  protected readonly isDark = signal<boolean>(this.readInitialDark());
  protected readonly themeIcon = computed(() =>
    this.isDark() ? 'light_mode' : 'dark_mode',
  );
  protected readonly themeTooltip = computed(() =>
    this.isDark() ? 'Switch to light theme' : 'Switch to dark theme',
  );

  constructor() {
    effect(() => {
      const dark = this.isDark();
      const root = this.document.documentElement;
      root.classList.toggle('dark', dark);
      root.classList.toggle('light', !dark);
      try {
        this.document.defaultView?.localStorage?.setItem(
          THEME_STORAGE_KEY,
          dark ? 'dark' : 'light',
        );
      } catch {
        // ignore storage errors
      }
    });
  }

  protected toggleTheme(): void {
    this.isDark.update((v) => !v);
  }

  private readInitialDark(): boolean {
    try {
      const saved =
        this.document.defaultView?.localStorage?.getItem(THEME_STORAGE_KEY);
      if (saved === 'dark') return true;
      if (saved === 'light') return false;
    } catch {
      // ignore
    }
    return (
      this.document.defaultView?.matchMedia?.('(prefers-color-scheme: dark)')
        .matches ?? false
    );
  }
}
