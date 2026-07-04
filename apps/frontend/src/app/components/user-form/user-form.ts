import {
  ChangeDetectionStrategy,
  Component,
  computed,
  input,
  linkedSignal,
  output,
} from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatIconModule } from '@angular/material/icon';
import { MatInputModule } from '@angular/material/input';
import {
  email,
  form,
  FormField,
  max,
  min,
  required,
  submit,
} from '@angular/forms/signals';
import { User, UserPayload } from '@workspace/http';
import { toPayload } from './user-form.model';

@Component({
  imports: [
    FormField,
    MatButtonModule,
    MatCardModule,
    MatFormFieldModule,
    MatIconModule,
    MatInputModule,
  ],
  selector: 'app-user-form',
  templateUrl: './user-form.html',
  styleUrl: './user-form.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class UserForm {
  readonly user = input<User | null>(null);

  readonly save = output<UserPayload>();
  readonly cancelled = output<void>();

  protected readonly model = linkedSignal<UserPayload>(() =>
    toPayload(this.user()),
  );

  protected readonly userForm = form<UserPayload>(this.model, (path) => {
    required(path.name, { message: 'Name is required' });
    required(path.surname, { message: 'Surname is required' });
    required(path.email, { message: 'Email is required' });
    email(path.email, { message: 'Enter a valid email' });
    required(path.birthYear, { message: 'Birth year is required' });
    min(path.birthYear, 1900, { message: 'Birth year must be after 1900' });
    max(path.birthYear, new Date().getFullYear(), {
      message: 'Birth year cannot be in the future',
    });
  });

  protected readonly isEdit = computed(() => this.user() !== null);

  protected handleSubmit(event: Event): void {
    event.preventDefault();
    submit(this.userForm, async (form) => {
      this.save.emit(form().value());
    });
  }

  protected handleCancel(): void {
    this.cancelled.emit();
  }
}
