import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  inject,
  signal,
} from '@angular/core';
import { User as UserModel, UserPayload } from '@workspace/http';
import { User } from '../../components/user/user';
import { UserForm } from '../../components/user-form/user-form';
import { UserContainerService } from './user-container.service';

@Component({
  imports: [User, UserForm],
  selector: 'app-user-container',
  templateUrl: './user-container.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  providers: [UserContainerService],
})
export class UserContainer implements OnInit {
  private readonly service = inject(UserContainerService);

  protected readonly users = this.service.users;
  protected readonly loading = this.service.loading;

  protected readonly editing = signal<UserModel | null>(null);
  protected readonly creating = signal(false);

  ngOnInit(): void {
    this.service.load();
  }

  protected onAdd(): void {
    this.editing.set(null);
    this.creating.set(true);
  }

  protected onEdit(user: UserModel): void {
    this.creating.set(false);
    this.editing.set(user);
  }

  protected onDelete(id: number): void {
    if (this.editing()?.id === id) {
      this.editing.set(null);
    }
    this.service.delete(id);
  }

  protected onSave(payload: UserPayload): void {
    const current = this.editing();
    if (current) {
      this.service.update(current.id, payload);
    } else {
      this.service.create(payload);
    }
    this.closeForm();
  }

  protected closeForm(): void {
    this.editing.set(null);
    this.creating.set(false);
  }
}
