#!/usr/bin/env fish

set -gx PGPASSWORD garden

set -l command ".$argv[1]"

cat ./data/commands.yaml | yq -r "$command" | psql -h localhost -U postgres postgres
