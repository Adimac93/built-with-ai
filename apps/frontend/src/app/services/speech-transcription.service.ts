import { HttpClient } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';
import { firstValueFrom } from 'rxjs';
import { ConfigProvider } from '../config/config-provider';

interface TranscribeResponse {
  text: string;
}

@Injectable({ providedIn: 'root' })
export class SpeechTranscriptionService {
  private readonly http = inject(HttpClient);
  private readonly config = inject(ConfigProvider);

  async transcribe(blob: Blob, languageCode = 'pl-PL'): Promise<string> {
    const audioContent = await this.blobToBase64(blob);
    const response = await firstValueFrom(
      this.http.post<TranscribeResponse>(
        `${this.config.apiUrl()}/speech/transcribe`,
        {
          audioContent,
          mimeType: blob.type,
          languageCode,
        },
      ),
    );
    return response.text.trim();
  }

  private blobToBase64(blob: Blob): Promise<string> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = () => {
        const result = String(reader.result);
        resolve(result.includes(',') ? result.split(',')[1] : result);
      };
      reader.onerror = () => reject(reader.error);
      reader.readAsDataURL(blob);
    });
  }
}
