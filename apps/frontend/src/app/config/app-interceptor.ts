import { HttpHandlerFn, HttpRequest } from '@angular/common/http';
import { inject } from '@angular/core';
import { ConfigProvider } from './config-provider';

export function appInterceptor(req: HttpRequest<unknown>, next: HttpHandlerFn) {
  if (req.url.startsWith('http')) {
    return next(req);
  }
  const configProvider = inject(ConfigProvider);
  const fakeApi = configProvider.fakeApiUrl();
  return next(req.clone({ url: `${fakeApi}/${req.url}` }));
}
