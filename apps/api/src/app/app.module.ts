import { Module } from '@nestjs/common';
import { PipelineModule } from '../pipeline/pipeline.module';
import { AppController } from './app.controller';
import { AppService } from './app.service';

@Module({
  imports: [PipelineModule],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
