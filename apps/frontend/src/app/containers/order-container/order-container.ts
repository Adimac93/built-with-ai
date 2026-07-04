import {
  ChangeDetectionStrategy,
  Component,
  OnInit,
  inject,
  signal,
} from '@angular/core';
import {
  Order as OrderModel,
  OrderPayload,
  UserHttp,
  User,
} from '@workspace/http';
import { Order } from '../../components/order/order';
import { OrderForm } from '../../components/order-form/order-form';
import { OrderContainerService } from './order-container.service';

@Component({
  imports: [Order, OrderForm],
  selector: 'app-order-container',
  templateUrl: './order-container.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
  providers: [OrderContainerService],
})
export class OrderContainer implements OnInit {
  private readonly service = inject(OrderContainerService);
  private readonly userHttp = inject(UserHttp);

  protected readonly orders = this.service.orders;
  protected readonly loading = this.service.loading;
  protected readonly users = signal<User[]>([]);

  protected readonly editing = signal<OrderModel | null>(null);
  protected readonly creating = signal(false);

  ngOnInit(): void {
    this.service.load();
    this.userHttp.getAll().subscribe((users) => this.users.set(users));
  }

  protected onAdd(): void {
    this.editing.set(null);
    this.creating.set(true);
  }

  protected onEdit(order: OrderModel): void {
    this.creating.set(false);
    this.editing.set(order);
  }

  protected onDelete(id: number): void {
    if (this.editing()?.id === id) {
      this.editing.set(null);
    }
    this.service.delete(id);
  }

  protected onSave(payload: OrderPayload): void {
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
