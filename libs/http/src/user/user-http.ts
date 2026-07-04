import { HttpClient } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';
import { Observable } from 'rxjs';
import { User, UserPayload } from './user.model';

@Injectable({
  providedIn: 'root',
})
export class UserHttp {
  private readonly apiUrl = 'user';
  private readonly userHttp = inject(HttpClient);

  getAll(): Observable<User[]> {
    return this.userHttp.get<User[]>(this.apiUrl);
  }

  create(payload: UserPayload): Observable<User> {
    return this.userHttp.post<User>(this.apiUrl, payload);
  }

  update(id: number, payload: UserPayload): Observable<User> {
    return this.userHttp.put<User>(`${this.apiUrl}/${id}`, payload);
  }

  delete(id: number): Observable<void> {
    return this.userHttp.delete<void>(`${this.apiUrl}/${id}`);
  }
}
