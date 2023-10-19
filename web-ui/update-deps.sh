#! /usr/bin/env bash
set -euxo pipefail

script_dir="./script"
style_dir="./style"

curl -L -o "$script_dir/htmx-core.min.js" 'https://unpkg.com/htmx.org@1.9.6'
curl -L -o "$script_dir/htmx-ws.js" 'https://unpkg.com/htmx.org/dist/ext/ws.js'
curl -L -o "$style_dir/open-props.min.css" 'https://unpkg.com/open-props'
curl -L -o "$style_dir/open-props-normalize.min.css" 'https://unpkg.com/open-props/normalize.min.css'
