import { HttpClient } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';
import { Observable } from 'rxjs';
import { Order, OrderPayload } from './order.model';

@Injectable({
  providedIn: 'root',
})
export class OrderHttp {
  private readonly http = inject(HttpClient);
  private readonly url = 'order';

  getAll(): Observable<Order[]> {
    return this.http.get<Order[]>(this.url);
  }

  create(payload: OrderPayload): Observable<Order> {
    return this.http.post<Order>(this.url, payload);
  }

  update(id: number, payload: OrderPayload): Observable<Order> {
    return this.http.put<Order>(`${this.url}/${id}`, payload);
  }

  delete(id: number): Observable<void> {
    return this.http.delete<void>(`${this.url}/${id}`);
  }
}
