import { ApiProperty } from '@nestjs/swagger';
import { IsIn, IsNotEmpty, IsOptional, IsString } from 'class-validator';

export class TranscribeRequestDto {
  @ApiProperty({
    description: 'Base64-encoded audio captured in the browser',
  })
  @IsString()
  @IsNotEmpty()
  audioContent!: string;

  @ApiProperty({
    description: 'Browser MediaRecorder MIME type',
    example: 'audio/webm;codecs=opus',
  })
  @IsString()
  @IsNotEmpty()
  mimeType!: string;

  @ApiProperty({
    description: 'BCP-47 language code used by Google Cloud Speech-to-Text',
    example: 'pl-PL',
    required: false,
  })
  @IsString()
  @IsOptional()
  languageCode?: string;

  @ApiProperty({
    description: 'Speech-to-Text audio encoding',
    enum: ['WEBM_OPUS', 'OGG_OPUS', 'LINEAR16', 'MP3'],
    required: false,
  })
  @IsIn(['WEBM_OPUS', 'OGG_OPUS', 'LINEAR16', 'MP3'])
  @IsOptional()
  encoding?: 'WEBM_OPUS' | 'OGG_OPUS' | 'LINEAR16' | 'MP3';
}
