import * as vscode from 'vscode';
import { Context } from './context';
import createCommandFactories from './commands';

let context: Context;

export function activate(extensionContext: vscode.ExtensionContext) {
  console.log('activate: starting extension');
  context = new Context(extensionContext, createCommandFactories());
  void context.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!context) {
    return;
  }
  return context.stop();
}
