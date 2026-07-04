import { ChangeDetectionStrategy, Component } from '@angular/core';
import { UserContainer } from '../../containers/user-container/user-container';

@Component({
  imports: [UserContainer],
  selector: 'app-user-page',
  templateUrl: './user-page.html',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class UserPage {}
