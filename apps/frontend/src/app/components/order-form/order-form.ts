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
import { form, FormField, min, required, submit } from '@angular/forms/signals';
import { Order, OrderPayload, User } from '@workspace/http';
import { OrderFormModel, toFormModel } from './order-form.model';

@Component({
  imports: [
    FormField,
    MatButtonModule,
    MatCardModule,
    MatFormFieldModule,
    MatIconModule,
    MatInputModule,
  ],
  selector: 'app-order-form',
  templateUrl: './order-form.html',
  styleUrl: './order-form.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class OrderForm {
  readonly order = input<Order | null>(null);
  readonly users = input<User[]>([]);

  readonly save = output<OrderPayload>();
  readonly cancelled = output<void>();

  protected readonly model = linkedSignal<OrderFormModel>(() =>
    toFormModel(this.order(), this.users()),
  );

  protected readonly orderForm = form<OrderFormModel>(this.model, (path) => {
    required(path.userId, { message: 'User is required' });
    required(path.productName, { message: 'Product name is required' });
    required(path.amount, { message: 'Amount is required' });
    min(path.amount, 0, { message: 'Amount must be positive' });
    required(path.quantity, { message: 'Quantity is required' });
    min(path.quantity, 1, { message: 'Quantity must be at least 1' });
  });

  protected readonly isEdit = computed(() => this.order() !== null);

  protected handleSubmit(event: Event): void {
    event.preventDefault();
    submit(this.orderForm, async (form) => {
      const value = form().value();
      this.save.emit({
        userId: Number(value.userId),
        productName: value.productName,
        amount: Number(value.amount),
        quantity: Number(value.quantity),
      });
    });
  }

  protected handleCancel(): void {
    this.cancelled.emit();
  }
}
