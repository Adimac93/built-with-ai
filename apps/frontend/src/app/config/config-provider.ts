import { Injectable, signal, computed } from '@angular/core';

export interface AppConfig {
  apiUrl: string;
  fakeApiUrl: string;
}

@Injectable({ providedIn: 'root' })
export class ConfigProvider {
  private readonly _config = signal<AppConfig>({ apiUrl: '', fakeApiUrl: '' });

  readonly apiUrl = computed(() => this._config().apiUrl);
  readonly fakeApiUrl = computed(() => this._config().fakeApiUrl);

  setConfig(config: AppConfig): void {
    this._config.set(config);
  }
}
