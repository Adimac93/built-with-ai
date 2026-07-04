import { Injectable, InternalServerErrorException, Logger } from '@nestjs/common';
import axios, { AxiosError } from 'axios';
import { SolveRequestDto } from './dto/solve-request.dto';
import { SolveResponseDto } from './dto/solve-response.dto';

@Injectable()
export class PipelineService {
  private readonly logger = new Logger(PipelineService.name);
  private readonly agentUrl = process.env['AGENT_URL'] ?? 'http://localhost:8000';

  async solve(dto: SolveRequestDto): Promise<SolveResponseDto> {
    try {
      const response = await axios.post<SolveResponseDto>(
        `${this.agentUrl}/solve`,
        { problem: dto.problem },
        { timeout: 120_000 },
      );
      return response.data;
    } catch (err) {
      const axiosErr = err as AxiosError;
      const detail = axiosErr.response?.data ?? axiosErr.message;
      this.logger.error('Agent call failed', detail);
      throw new InternalServerErrorException('Agent call failed');
    }
  }
}
