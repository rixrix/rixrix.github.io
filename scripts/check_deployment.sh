#!/usr/bin/env bash
set -euo pipefail

site_dir=$1
site_url=${2%/}
post_route=${3:-}
revision=$(cat "$site_dir/build-revision.txt")
response=$(mktemp)
trap 'rm -f "$response"' EXIT

routes=(build-revision.txt '' about/ feed.xml)
if [[ -n "$post_route" ]]; then
  routes+=("$post_route")
fi

for route in "${routes[@]}"; do
  case "$route" in
    ''|*/) target="${route}index.html" ;;
    *) target=$route ;;
  esac
  url="$site_url/$route?revision=$revision"
  verified=false
  for attempt in {1..12}; do
    if curl --fail --silent --show-error --location --max-time 10 --compressed "$url" --output "$response" &&
      cmp -s "$site_dir/$target" "$response"; then
      verified=true
      break
    fi
    if [[ "$attempt" != 12 ]]; then sleep 5; fi
  done
  if [[ "$verified" != true ]]; then
    printf 'Live page did not match revision %s: %s\n' "$revision" "$url" >&2
    exit 1
  fi
  printf 'Verified %s/%s\n' "$site_url" "$route"
done
