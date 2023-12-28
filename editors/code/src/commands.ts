import * as vscode from 'vscode';
import { type Context } from './context';
import { isStarlarkDocument, isStarlarkTextEditor } from './util';

/**
 * A factory for creating commands with access to the `Context` object.
 */
export type CommandFactory = (ctx: Context) => (...args: any[]) => unknown;

function showVersion() {
  return () => {
    vscode.window.showInformationMessage('starpls, v0.1.0');
  };
}

function showSyntaxTree(ctx: Context) {
  // Define and register a content provider for the syntax tree viewer.
  const syntaxTreeScheme = 'starpls-syntax-tree';
  const syntaxTreeUri = vscode.Uri.parse(`${syntaxTreeScheme}://syntaxtree/tree.rast`);

  const syntaxTreeProvider = new class implements vscode.TextDocumentContentProvider {
    private readonly emitter = new vscode.EventEmitter<vscode.Uri>();
    onDidChange = this.emitter.event;

    constructor() {
      vscode.window.onDidChangeActiveTextEditor((textEditor) => this.onDidChangeActiveTextEditor(textEditor), this, ctx.disposables);
      vscode.workspace.onDidChangeTextDocument((textDocumentChangeEvent) => this.onDidChangeTextDocument(textDocumentChangeEvent), this, ctx.disposables);
    }

    provideTextDocumentContent(_uri: vscode.Uri, _token: vscode.CancellationToken): vscode.ProviderResult<string> {
      if (!ctx.activeStarlarkTextEditor) {
        return;
      }
      return ctx.client.sendRequest('starpls/showSyntaxTree', {
        textDocument: {
          uri: ctx.activeStarlarkTextEditor.document.uri.toString(),
        },
      });
    }

    onDidChangeActiveTextEditor(textEditor: vscode.TextEditor | undefined) {
      // Check if the new editor is an editor for Starlark source files. If so, update our content.
      if (textEditor && isStarlarkTextEditor(textEditor)) {
        this.emitter.fire(syntaxTreeUri);
      }
    }

    onDidChangeTextDocument(textDocumentChangeEvent: vscode.TextDocumentChangeEvent) {
      // The sleep here is a best-effort attempt at making sure the `onDidChangeTextDocument` notification was processed by the language
      // server before we request a content update.
      if (isStarlarkDocument(textDocumentChangeEvent.document)) {
        setTimeout(() => this.emitter.fire(syntaxTreeUri), 10);
      }
    }
  };

  ctx.disposables.push(vscode.workspace.registerTextDocumentContentProvider(syntaxTreeScheme, syntaxTreeProvider));

  return async () => {
    const document = await vscode.workspace.openTextDocument(syntaxTreeUri);
    await vscode.window.showTextDocument(document, {
      preserveFocus: true,
      viewColumn: vscode.ViewColumn.Two,
    });
  };
}

export default function createCommandFactories(): Record<string, CommandFactory> {
  return {
    'starpls.showSyntaxTree': showSyntaxTree,
    'starpls.showVersion': showVersion,
  };
}
