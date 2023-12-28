import * as vscode from 'vscode';
import { Context } from './context';
import createCommandFactories from './commands';

export function activate(extensionContext: vscode.ExtensionContext) {
  console.log('activate: starting extension');
  const context = new Context(extensionContext, createCommandFactories());
  void context.start();
}
