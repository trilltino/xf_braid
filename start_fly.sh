#!/bin/bash
set -e

export LEPTOS_SITE_ADDR="${LEPTOS_SITE_ADDR:-0.0.0.0:3000}"
export LEPTOS_SITE_ROOT="${LEPTOS_SITE_ROOT:-site}"

echo "=== XFBraid Starting ==="
echo "LEPTOS_SITE_ADDR: $LEPTOS_SITE_ADDR"
echo "LEPTOS_SITE_ROOT: $LEPTOS_SITE_ROOT"
echo "DATABASE_URL: ${DATABASE_URL:-sqlite:./data.db?mode=rwc}"

exec /app/backend
