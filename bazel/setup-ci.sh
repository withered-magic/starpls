#!/usr/bin/env bash
if [[ -n "$BUILDBUDDY_API_KEY" ]]; then
    echo 'import %workspace%/bazel/remote-cache.bazelrc' >>.bazelrc
    echo "build --remote_header=x-buildbuddy-api-key=$BUILDBUDDY_API_KEY" >>.bazelrc
fi
