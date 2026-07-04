import {
  ChangeDetectionStrategy,
  Component,
  input,
  output,
} from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatTableModule } from '@angular/material/table';
import { MatTooltipModule } from '@angular/material/tooltip';
import { User as UserModel } from '@workspace/http';

@Component({
  imports: [
    MatButtonModule,
    MatCardModule,
    MatIconModule,
    MatProgressBarModule,
    MatTableModule,
    MatTooltipModule,
  ],
  selector: 'app-user',
  templateUrl: './user.html',
  styleUrl: './user.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class User {
  readonly users = input.required<UserModel[]>();
  readonly loading = input<boolean>(false);

  readonly add = output<void>();
  readonly edit = output<UserModel>();
  readonly remove = output<number>();

  readonly displayedColumns = [
    'name',
    'surname',
    'birthYear',
    'email',
    'actions',
  ];
}
