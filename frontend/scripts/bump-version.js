// Bumps the project's build number on every `npm run dev` (wired via the "predev" script).
//
// Version pattern: {major}.{minor}.{build}.{revision}
//   - {build} auto-increments by 1 here, on each dev-server start.
//   - {major}/{minor}/{revision} are set intentionally by a human, so {revision} is
//     normally 0 (it exists for occasional hand-bumped revisions of a given build).
//
// The version lives in src/version.json and is overwritten in place on each run.
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const versionPath = join(dirname(fileURLToPath(import.meta.url)), '..', 'src', 'version.json');

let v = { major: 0, minor: 0, build: 0, revision: 0 };
try {
  v = { ...v, ...JSON.parse(readFileSync(versionPath, 'utf8')) };
} catch {
  // First run (or the file is missing/unreadable): start from the defaults above.
}

v.build = (Number(v.build) || 0) + 1;
v.revision = Number(v.revision) || 0;
v.version = `${v.major}.${v.minor}.${v.build}.${v.revision}`;

writeFileSync(versionPath, JSON.stringify(v, null, 2) + '\n');
console.log(`[version] bumped to ${v.version}`);
