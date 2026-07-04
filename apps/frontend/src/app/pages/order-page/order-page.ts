import { ChangeDetectionStrategy, Component } from '@angular/core';
import { OrderContainer } from '../../containers/order-container/order-container';

@Component({
  imports: [OrderContainer],
  selector: 'app-order-page',
  templateUrl: './order-page.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class OrderPage {}
