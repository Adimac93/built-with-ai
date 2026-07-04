import {
  ChangeDetectionStrategy,
  Component,
  computed,
  input,
  output,
} from '@angular/core';
import { CurrencyPipe } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatIconModule } from '@angular/material/icon';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatTableModule } from '@angular/material/table';
import { MatTooltipModule } from '@angular/material/tooltip';
import { Order as OrderModel, User as UserModel } from '@workspace/http';

interface OrderRow extends OrderModel {
  userLabel: string;
}

@Component({
  imports: [
    CurrencyPipe,
    MatButtonModule,
    MatCardModule,
    MatIconModule,
    MatProgressBarModule,
    MatTableModule,
    MatTooltipModule,
  ],
  selector: 'app-order',
  templateUrl: './order.html',
  styleUrl: './order.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class Order {
  readonly orders = input.required<OrderModel[]>();
  readonly users = input<UserModel[]>([]);
  readonly loading = input<boolean>(false);

  readonly add = output<void>();
  readonly edit = output<OrderModel>();
  readonly remove = output<number>();

  readonly displayedColumns = [
    'user',
    'productName',
    'quantity',
    'amount',
    'actions',
  ];

  readonly rows = computed<OrderRow[]>(() => {
    const usersById = new Map(this.users().map((u) => [u.id, u]));
    return this.orders().map((order) => {
      const user = usersById.get(order.userId);
      return {
        ...order,
        userLabel: user ? `${user.name} ${user.surname}` : `#${order.userId}`,
      };
    });
  });
}
