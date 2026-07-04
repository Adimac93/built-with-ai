import * as net from 'net';

/**
 * Polls a TCP port until something accepts a connection.
 * Replaces `waitForPortOpen` from `@nx/node/utils` (the API is no longer a
 * Node app, so the plugin was removed).
 *
 * @param port - TCP port to probe.
 * @param options - Optional host (default `localhost`) and timeout in ms (default 120s).
 * @throws When the port does not open before the timeout elapses.
 */
export async function waitForPortOpen(
  port: number,
  options: { host?: string; timeoutMs?: number } = {},
): Promise<void> {
  const host = options.host ?? 'localhost';
  const timeoutMs = options.timeoutMs ?? 120_000;
  const deadline = Date.now() + timeoutMs;

  while (Date.now() < deadline) {
    const open = await new Promise<boolean>((resolve) => {
      const socket = net.connect({ host, port });
      socket.once('connect', () => {
        socket.destroy();
        resolve(true);
      });
      socket.once('error', () => {
        socket.destroy();
        resolve(false);
      });
    });

    if (open) return;
    await new Promise((resolve) => setTimeout(resolve, 500));
  }

  throw new Error(`Timed out waiting for port ${port} on ${host} to open`);
}
