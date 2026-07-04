import { Route } from '@angular/router';

export const appRoutes: Route[] = [
  {
    path: '',
    loadComponent: () =>
      import('./layouts/heureka-layout/heureka-layout').then(
        (m) => m.HeurekaLayout,
      ),
    children: [
      {
        path: '',
        loadComponent: () =>
          import('./pages/analysis-page/analysis-page').then(
            (m) => m.AnalysisPage,
          ),
      },
      {
        path: 'metody',
        loadComponent: () =>
          import('./pages/methods-page/methods-page').then(
            (m) => m.MethodsPage,
          ),
      },
      {
        path: 'zespol',
        loadComponent: () =>
          import('./pages/team-page/team-page').then((m) => m.TeamPage),
      },
    ],
  },
];
