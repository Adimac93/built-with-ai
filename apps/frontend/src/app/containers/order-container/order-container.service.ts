import { Injectable, inject, signal } from '@angular/core';
import { OrderHttp, Order, OrderPayload } from '@workspace/http';

@Injectable()
export class OrderContainerService {
  private readonly http = inject(OrderHttp);

  readonly orders = signal<Order[]>([]);
  readonly loading = signal(false);

  load(): void {
    this.loading.set(true);
    this.http.getAll().subscribe({
      next: (orders) => {
        this.orders.set(orders);
        this.loading.set(false);
      },
      error: () => this.loading.set(false),
    });
  }

  create(payload: OrderPayload): void {
    this.http.create(payload).subscribe((order) => {
      this.orders.update((list) => [...list, order]);
    });
  }

  update(id: number, payload: OrderPayload): void {
    this.http.update(id, payload).subscribe((updated) => {
      this.orders.update((list) =>
        list.map((o) => (o.id === id ? updated : o)),
      );
    });
  }

  delete(id: number): void {
    this.http.delete(id).subscribe(() => {
      this.orders.update((list) => list.filter((o) => o.id !== id));
    });
  }
}
