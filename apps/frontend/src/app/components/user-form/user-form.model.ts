import { User, UserPayload } from '@workspace/http';

function emptyPayload(): UserPayload {
  return {
    name: '',
    surname: '',
    birthYear: new Date().getFullYear() - 20,
    email: '',
  };
}

export function toPayload(user: User | null): UserPayload {
  return user
    ? {
        name: user.name,
        surname: user.surname,
        birthYear: user.birthYear,
        email: user.email,
      }
    : emptyPayload();
}