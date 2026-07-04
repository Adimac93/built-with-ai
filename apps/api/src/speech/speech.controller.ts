import { Body, Controller, HttpCode, Post } from '@nestjs/common';
import { ApiOperation, ApiResponse, ApiTags } from '@nestjs/swagger';
import { TranscribeRequestDto } from './dto/transcribe-request.dto';
import { TranscribeResponseDto } from './dto/transcribe-response.dto';
import { SpeechService } from './speech.service';

@ApiTags('speech')
@Controller('speech')
export class SpeechController {
  constructor(private readonly speechService: SpeechService) {}

  @Post('transcribe')
  @HttpCode(200)
  @ApiOperation({
    summary: 'Transcribe browser-recorded audio',
    description:
      'Uses Google Cloud Speech-to-Text from the backend. Credentials never go to the browser.',
  })
  @ApiResponse({ status: 200, type: TranscribeResponseDto })
  transcribe(@Body() dto: TranscribeRequestDto): Promise<TranscribeResponseDto> {
    return this.speechService.transcribe(dto);
  }
}
