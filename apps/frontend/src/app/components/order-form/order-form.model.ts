import { Order, User } from '@workspace/http';

export interface OrderFormModel {
  userId: string;
  productName: string;
  amount: number;
  quantity: number;
}

function emptyPayload(users: readonly User[]): OrderFormModel {
  return {
    userId: users[0] ? String(users[0].id) : '',
    productName: '',
    amount: 0,
    quantity: 1,
  };
}

export function toFormModel(
  order: Order | null,
  users: readonly User[],
): OrderFormModel {
  return order
    ? {
        userId: String(order.userId),
        productName: order.productName,
        amount: order.amount,
        quantity: order.quantity,
      }
    : emptyPayload(users);
}