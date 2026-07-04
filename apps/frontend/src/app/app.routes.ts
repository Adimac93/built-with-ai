import { Route } from '@angular/router';

export const appRoutes: Route[] = [
  {
    path: '',
    redirectTo: 'users',
    pathMatch: 'full',
  },
  {
    path: 'users',
    loadComponent: () =>
      import('./pages/user-page/user-page').then((m) => m.UserPage),
  },
  {
    path: 'orders',
    loadComponent: () =>
      import('./pages/order-page/order-page').then((m) => m.OrderPage),
  },
];
