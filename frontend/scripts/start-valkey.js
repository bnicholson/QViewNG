// Brings the Valkey cache container up before the dev servers start (wired via "predev").
//
// Why this has to happen *before* the backend, not alongside it: backend/src/cache.rs falls back
// to a disabled cache when Valkey is unreachable at startup, and that fallback is sticky. A
// connection that drops later reconnects on its own, but a cache that *started* disabled stays
// disabled for the life of the process. So Valkey must be healthy before the backend boots, not
// merely shortly after -- hence `up -d --wait`, which returns only once the healthcheck passes.
//
// Failure here is deliberately not fatal. The cache is an optimization, and cache.rs's one rule
// is that it must never be able to take the site down; blocking all local development because
// Docker is not running would break that same spirit. On any failure we warn and let `npm run
// dev` carry on uncached -- slower, but working.
//
// Set QVIEW_SKIP_VALKEY=1 to skip this entirely and develop with caching disabled.
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

if (process.env.QVIEW_SKIP_VALKEY) {
  console.log('[valkey] QVIEW_SKIP_VALKEY is set; skipping. The backend will run uncached.');
  process.exit(0);
}

// The compose files live at the repo root, two levels up from frontend/scripts/. Running compose
// from there also lets it auto-load the root .env, which the other services interpolate from.
const repoRoot = join(dirname(fileURLToPath(import.meta.url)), '..', '..');

console.log('[valkey] starting cache container (first run may pull the image)...');

const { status, error } = spawnSync(
  'docker',
  ['compose', '-f', 'compose.yaml', '-f', 'compose.dev.yaml', 'up', '-d', '--wait', 'valkey'],
  { cwd: repoRoot, stdio: 'inherit' },
);

if (error || status !== 0) {
  const why = error?.code === 'ENOENT' ? 'docker was not found on PATH' : `docker compose exited ${status}`;
  console.warn(`[valkey] could not start the cache (${why}).`);
  console.warn('[valkey] Continuing without it: the app runs fine uncached, just slower.');
  process.exit(0);
}

console.log('[valkey] ready at redis://127.0.0.1:6379');
