import { Body, Controller, HttpCode, Post } from '@nestjs/common';
import {
  ApiOperation,
  ApiResponse,
  ApiTags,
} from '@nestjs/swagger';
import { SolveRequestDto } from './dto/solve-request.dto';
import { SolveResponseDto } from './dto/solve-response.dto';
import { PipelineService } from './pipeline.service';

@ApiTags('pipeline')
@Controller('solve')
export class PipelineController {
  constructor(private readonly pipelineService: PipelineService) {}

  @Post()
  @HttpCode(200)
  @ApiOperation({
    summary: 'Solve an inventive problem',
    description:
      'Runs the TRIZ + SCAMPER reasoning pipeline and returns a 5-step trail JSON.',
  })
  @ApiResponse({ status: 200, description: 'Reasoning trail', type: SolveResponseDto })
  @ApiResponse({ status: 400, description: 'Invalid request body' })
  @ApiResponse({ status: 500, description: 'Agent pipeline failed' })
  solve(@Body() dto: SolveRequestDto): Promise<SolveResponseDto> {
    return this.pipelineService.solve(dto);
  }
}
