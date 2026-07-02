#!/usr/bin/env bash
echo 'import %workspace%/bazel/remote-cache.bazelrc' >>.bazelrc
if [[ -n "$BUILDBUDDY_API_KEY" ]]; then
  echo "build --remote_header=x-buildbuddy-api-key=$BUILDBUDDY_API_KEY" >>.bazelrc
fi
