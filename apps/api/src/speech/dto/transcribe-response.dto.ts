import { ApiProperty } from '@nestjs/swagger';

export class TranscribeResponseDto {
  @ApiProperty({ description: 'Best transcript returned by Speech-to-Text' })
  text!: string;
}
