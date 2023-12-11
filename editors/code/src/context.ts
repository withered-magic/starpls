import * as vscode from 'vscode';
import { Executable, LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';

/**
 * The `Context` class wraps a `vscode.ExtensionContext and keeps track of the current state
 * of the editor.
 */
export class Context {
  /**
   * The active client connection to the language server.
   */
  private _client!: LanguageClient;

  constructor(readonly extensionContext: vscode.ExtensionContext) { }

  get client(): LanguageClient {
    return this._client;
  }

  async start() {
    // Establish connection to the language server.
    console.log('context: connecting to the language server');
    const client = await this.getOrCreateClient();
    await client.start();
  }

  private async getOrCreateClient(): Promise<LanguageClient> {
    if (this._client) {
      return this._client;
    }

    // Determine the path to the server executable, installing it if necessary.
    const serverPath = await this.ensureServerInstalled();

    // Set up language client/server options.
    const executable: Executable = { command: serverPath };
    const serverOptions: ServerOptions = { debug: executable, run: executable };
    const clientOptions: LanguageClientOptions = {
      documentSelector: [{ scheme: 'file', language: 'starlark' }],
    };

    return this._client = new LanguageClient(
      'star-ls',
      'Starlark Language Server',
      serverOptions,
      clientOptions,
    );
  }

  private async ensureServerInstalled(): Promise<string> {
    const serverPath = process.env.__STAR_LS_SERVER_DEBUG;
    if (!serverPath) {
      throw new Error('failed to find server executable: __STAR_LS_SERVER_DEBUG is not set');
    }
    console.log('context: using server executable at %s', serverPath);
    return serverPath;
  }
}
