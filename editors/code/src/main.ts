import * as vscode from 'vscode';
import { Context } from './context';

export function activate(extensionContext: vscode.ExtensionContext) {
  console.log('activate: starting extension');
  const context = new Context(extensionContext);
  void context.start();
}
