#!/bin/bash
cd "$(dirname "$0")"
npm run build 2>/dev/null
npx vitest run "$@" 2>&1
