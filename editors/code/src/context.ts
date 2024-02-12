import * as path from 'path';
import * as vscode from 'vscode';
import { Executable, LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';
import { CommandFactory } from './commands';
import { StarlarkTextEditor, isStarlarkTextEditor } from './util';

/**
 * The `Context` class wraps a `vscode.ExtensionContext and keeps track of the current state
 * of the editor.
 */
export class Context {
  /**
   * The active client connection to the language server.
   */
  private _client!: LanguageClient;

  private _disposables: vscode.Disposable[];

  constructor(readonly extensionContext: vscode.ExtensionContext, private commandFactories: Record<string, CommandFactory>) {
    this._disposables = [];
  }

  get client(): LanguageClient {
    return this._client;
  }

  get disposables(): vscode.Disposable[] {
    return this._disposables;
  }

  /**
   * Initializes the context and establishes 
   */
  async start() {
    // Establish connection to the language server.
    console.log('context: connecting to the language server');
    const client = await this.getOrCreateClient();
    await client.start();

    // Register commands with the VSCode API.
    console.log('context: registering commands');
    this.registerCommands();
  }

  async stop() {
    this.disposables.forEach((disposable) => disposable.dispose());
    return this._client.stop();
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
      'starpls',
      'Starlark Language Server',
      serverOptions,
      clientOptions,
    );
  }

  private async ensureServerInstalled(): Promise<string> {
    const defaultServerPath = path.join(this.extensionContext.extensionPath, '/bin/starpls');
    const serverPath = process.env.__STARPLS_SERVER_DEBUG ? process.env.__STARPLS_SERVER_DEBUG : defaultServerPath;
    console.log('context: using server executable at %s', serverPath);
    return serverPath;
  }

  private registerCommands() {
    // Dispose of any currently active commands.
    this.disposables.forEach((disposable) => disposable.dispose());
    this._disposables = [];

    // Register the commands.
    for (const [name, factory] of Object.entries(this.commandFactories)) {
      const disposable = vscode.commands.registerCommand(name, factory(this));
      this.disposables.push(disposable);
    }
  }

  get activeStarlarkTextEditor(): StarlarkTextEditor | undefined {
    const activeTextEditor = vscode.window.activeTextEditor;
    return activeTextEditor && isStarlarkTextEditor(activeTextEditor) ? activeTextEditor : undefined;
  }
}
