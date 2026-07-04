import { Injectable, inject, signal } from '@angular/core';
import { UserHttp, User, UserPayload } from '@workspace/http';

@Injectable()
export class UserContainerService {
  private readonly http = inject(UserHttp);

  readonly users = signal<User[]>([]);
  readonly loading = signal(false);

  load(): void {
    this.loading.set(true);
    this.http.getAll().subscribe({
      next: (users) => {
        this.users.set(users);
        this.loading.set(false);
      },
      error: () => this.loading.set(false),
    });
  }

  create(payload: UserPayload): void {
    this.http.create(payload).subscribe((user) => {
      this.users.update((list) => [...list, user]);
    });
  }

  update(id: number, payload: UserPayload): void {
    this.http.update(id, payload).subscribe((updated) => {
      this.users.update((list) => list.map((u) => (u.id === id ? updated : u)));
    });
  }

  delete(id: number): void {
    this.http.delete(id).subscribe(() => {
      this.users.update((list) => list.filter((u) => u.id !== id));
    });
  }
}
