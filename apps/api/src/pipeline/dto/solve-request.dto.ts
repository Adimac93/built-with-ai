import { ApiProperty } from '@nestjs/swagger';
import { IsNotEmpty, IsString, MinLength } from 'class-validator';

export class SolveRequestDto {
  @ApiProperty({
    description: 'The inventive problem to solve',
    example: 'Moving crude oil across oceans relies on very large tankers. When something goes wrong a spill can be catastrophic. How can we reduce oil spill risk without changing the transport economics?',
    minLength: 10,
  })
  @IsString()
  @IsNotEmpty()
  @MinLength(10)
  problem!: string;
}
