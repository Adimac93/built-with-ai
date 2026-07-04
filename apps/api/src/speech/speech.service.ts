import {
  BadRequestException,
  Injectable,
  InternalServerErrorException,
  Logger,
} from '@nestjs/common';
import { TranscribeRequestDto } from './dto/transcribe-request.dto';
import { TranscribeResponseDto } from './dto/transcribe-response.dto';

interface SpeechRecognizeResponse {
  results?: Array<{
    alternatives?: Array<{
      transcript?: string;
      confidence?: number;
    }>;
  }>;
  error?: {
    message?: string;
  };
}

@Injectable()
export class SpeechService {
  private readonly logger = new Logger(SpeechService.name);
  private readonly apiUrl = 'https://speech.googleapis.com/v1/speech:recognize';

  async transcribe(dto: TranscribeRequestDto): Promise<TranscribeResponseDto> {
    const token = await this.getAccessToken();
    const encoding = dto.encoding ?? this.encodingFromMimeType(dto.mimeType);

    const response = await fetch(this.apiUrl, {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        config: {
          encoding,
          languageCode: dto.languageCode ?? 'pl-PL',
          alternativeLanguageCodes: ['en-US'],
          enableAutomaticPunctuation: true,
          model: 'latest_short',
        },
        audio: {
          content: dto.audioContent,
        },
      }),
    });

    const data = (await response.json()) as SpeechRecognizeResponse;

    if (!response.ok) {
      const detail = data.error?.message ?? response.statusText;
      this.logger.error('Speech-to-Text request failed', detail);
      throw new InternalServerErrorException('Speech-to-Text request failed');
    }

    const text =
      data.results
        ?.flatMap((result) => result.alternatives ?? [])
        .map((alternative) => alternative.transcript?.trim() ?? '')
        .filter(Boolean)
        .join(' ')
        .trim() ?? '';

    return { text };
  }

  private encodingFromMimeType(mimeType: string): TranscribeRequestDto['encoding'] {
    if (mimeType.includes('webm')) return 'WEBM_OPUS';
    if (mimeType.includes('ogg')) return 'OGG_OPUS';
    if (mimeType.includes('mpeg') || mimeType.includes('mp3')) return 'MP3';

    throw new BadRequestException(`Unsupported audio MIME type: ${mimeType}`);
  }

  private async getAccessToken(): Promise<string> {
    const explicitToken = process.env['GOOGLE_CLOUD_ACCESS_TOKEN'];
    if (explicitToken) return explicitToken;

    const response = await fetch(
      'http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token',
      { headers: { 'Metadata-Flavor': 'Google' } },
    );

    if (!response.ok) {
      this.logger.error(
        'Could not obtain Google Cloud access token from metadata server',
        response.statusText,
      );
      throw new InternalServerErrorException(
        'Google Cloud authentication is not configured',
      );
    }

    const data = (await response.json()) as { access_token?: string };
    if (!data.access_token) {
      throw new InternalServerErrorException(
        'Google Cloud metadata server returned no access token',
      );
    }
    return data.access_token;
  }
}
