import {
  ApplicationConfig,
  inject,
  provideAppInitializer,
  provideBrowserGlobalErrorListeners,
} from '@angular/core';
import { provideRouter } from '@angular/router';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { appRoutes } from './app.routes';
import { provideHttpClient, withInterceptors } from '@angular/common/http';
import { AppConfig, ConfigProvider } from './config/config-provider';
import { appInterceptor } from './config/app-interceptor';

export const appConfig: ApplicationConfig = {
  providers: [
    provideBrowserGlobalErrorListeners(),
    provideRouter(appRoutes),
    provideHttpClient(withInterceptors([appInterceptor])),
    provideAppInitializer(async () => {
      const configProvider = inject(ConfigProvider);
      const response = await fetch('/config.json');
      const config: AppConfig = await response.json();
      configProvider.setConfig(config);
    }),
    provideAnimationsAsync(),
  ],
};
