import {
  ApplicationConfig,
  inject,
  provideAppInitializer,
  provideBrowserGlobalErrorListeners,
} from '@angular/core';
import { provideRouter } from '@angular/router';
import { appRoutes } from './app.routes';
import { provideHttpClient } from '@angular/common/http';
import { AppConfig, ConfigProvider } from './config/config-provider';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(appRoutes),
    provideHttpClient(),
    provideAppInitializer(async () => {
      const configProvider = inject(ConfigProvider);
      const response = await fetch('/config.json');
      const config: AppConfig = await response.json();
      configProvider.setConfig(config);
    }),
  ],
};
