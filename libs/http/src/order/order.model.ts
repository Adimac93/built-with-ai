export interface Order {
  id: number;
  userId: number;
  productName: string;
  amount: number;
  quantity: number;
}

export type OrderPayload = Omit<Order, 'id'>;
