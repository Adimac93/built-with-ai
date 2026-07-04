import { ChangeDetectionStrategy, Component } from '@angular/core';

interface TeamMember {
  readonly photo: string;
  readonly photoAlt: string;
  readonly role: string;
  readonly name: string;
  readonly bio: string;
  readonly tags: readonly string[];
}

@Component({
  selector: 'app-team-page',
  templateUrl: './team-page.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class TeamPage {
  protected readonly members: readonly TeamMember[] = [
    {
      photo: 'img/AdmaM.jpeg',
      photoAlt: 'Adam Maciejczuk w czapce z daszkiem, selfie na górskim szlaku',
      role: 'Full-stack',
      name: 'Adam Maciejczuk',
      bio: 'Skleja silnik z interfejsem i pilnuje, żeby demo działało jedną komendą.',
      tags: ['pipeline', 'NestJS', 'integracja'],
    },
    {
      photo: 'img/AdamK.png',
      photoAlt: 'Adam Korwin przy palmie, w tle panorama miasta z wieżą kościoła',
      role: 'Frontend',
      name: 'Adam Korwin',
      bio: 'Odpowiada za arkusz, który właśnie oglądasz — od tokenów po dostępność z klawiatury.',
      tags: ['Angular', 'ng-diagram', 'a11y'],
    },
    {
      photo: 'img/AntekP.jpg',
      photoAlt: 'Antoni Pszenica w okularach, selfie na tle gotyckiej katedry',
      role: 'LLM · evals',
      name: 'Antoni Pszenica',
      bio: 'Trzyma model na krótkiej smyczy schematów, a jakość odpowiedzi mierzy metrykami, nie okiem.',
      tags: ['prompty', 'TRIZ', 'metryki'],
    },
  ];
}
