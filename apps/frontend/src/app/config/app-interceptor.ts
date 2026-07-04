import { HttpHandlerFn, HttpRequest } from '@angular/common/http';
import { inject } from '@angular/core';
import { ConfigProvider } from './config-provider';

export function appInterceptor(req: HttpRequest<unknown>, next: HttpHandlerFn) {
  const configProvider = inject(ConfigProvider);
  const fakeApi = configProvider.fakeApiUrl();

  const newReq = req.clone({
    url: `${fakeApi}/${req.url}`,
  });
  return next(newReq);
}
