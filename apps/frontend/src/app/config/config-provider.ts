import { Injectable, signal, computed } from '@angular/core';

export interface AppConfig {
  apiUrl: string;
}

@Injectable({ providedIn: 'root' })
export class ConfigProvider {
  private readonly _config = signal<AppConfig>({ apiUrl: '' });

  readonly apiUrl = computed(() => this._config().apiUrl);

  setConfig(config: AppConfig): void {
    this._config.set(config);
  }
}
