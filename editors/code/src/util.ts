import * as vscode from 'vscode';

export type StarlarkDocument = vscode.TextDocument & { languageId: 'starlark ' };
export type StarlarkTextEditor = vscode.TextEditor & { document: StarlarkDocument };

export function isStarlarkDocument(document: vscode.TextDocument): document is StarlarkDocument {
  return document.languageId === 'starlark' && document.uri.scheme === 'file';
}

export function isStarlarkTextEditor(textEditor: vscode.TextEditor): textEditor is StarlarkTextEditor {
  return isStarlarkDocument(textEditor.document);
}
