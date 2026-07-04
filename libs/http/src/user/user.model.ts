export interface User {
  id: number;
  name: string;
  surname: string;
  birthYear: number;
  email: string;
}

export type UserPayload = Omit<User, 'id'>;
