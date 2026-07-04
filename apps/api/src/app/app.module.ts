import { Module } from '@nestjs/common';
import { PipelineModule } from '../pipeline/pipeline.module';
import { SpeechModule } from '../speech/speech.module';
import { AppController } from './app.controller';
import { AppService } from './app.service';

@Module({
  imports: [PipelineModule, SpeechModule],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
