#!/usr/bin/env starlark

# Test file for exec extension
load("command/execution.star", "exec")

# Test exec object methods
def test_exec():
    # These should provide completion and proper hover info
    result = exec.sh("ls -la")
    exec.echo("Hello world")
    exec.checkout("main")

    # Test dot completion here - cursor after exec.
    exec.

test_exec()