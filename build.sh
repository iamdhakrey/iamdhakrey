#!/bin/bash
 
VERSION=$(git describe --tags --always)
COMMIT=$(git rev-parse HEAD)
BRANCH=$(git rev-parse --abbrev-ref HEAD)
BUILDTIME=$(date -u '+%Y-%m-%dT%H:%M:%SZ')


go build -ldflags "-X 'spendliteapi/cmd.Version=$VERSION' \
                   -X 'spendliteapi/cmd.Commit=$COMMIT' \
                   -X 'spendliteapi/cmd.Branch=$BRANCH' \
                   -X 'spendliteapi/cmd.BuildTime=$BUILDTIME'" \
