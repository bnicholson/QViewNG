// ── Page titles ──────────────────────────────────────────────────────────────
//
// `document.title` for every route, as "QView - {section name}". The three groups below mirror
// the three shapes in the routes table (see `routes.tsx`), so a new route only needs an entry
// here when it introduces a path or a section segment that isn't already listed:
//
//   1. Standalone pages            /login, /files, …
//   2. Profiles with sections      /{entity}/{id}, /{entity}/{id}/{section}
//   3. Documentation               /{category}-docs/{version}/{slug}
//
// A profile's title names the entity and then the section within it — /division/:did/games reads
// "QView - Division Games" and /tournament/:tid/games reads "QView - Tournament Games". Section
// labels are keyed by their URL segment and shared across every profile, so a section common to
// several entities is listed once.

import { docArticle, docCategoryByPath, resolveVersion } from './data/docs'

export const APP_NAME = 'QView'

/** "QView - {section}", or bare "QView" for the landing page. */
const format = (section?: string): string => (section ? `${APP_NAME} - ${section}` : APP_NAME)

/** Pages that own their whole path. A value of `undefined` yields just "QView". */
const STANDALONE: Record<string, string | undefined> = {
  '/': undefined,
  '/tournaments': 'Tournaments',
  '/tournaments-page': 'Tournaments',
  '/help': 'Help Center',
  '/markdown-cheatsheet': 'Markdown Cheatsheet',
  '/login': 'Log In',
  '/recovery': 'Password Recovery',
  '/reset': 'Reset Password',
  '/activate': 'Activate Account',
  '/register': 'Register',
  '/division/old': 'Divisions',
  '/tdeditor': 'Tournament Director Editor',
  '/rounds-in-progress': 'Rounds in Progress',
  '/files': 'Files',
  '/swagger': 'API Reference',
  '/dev': 'Developer',
  '/404': 'Page Not Found',
}

/** What a `/{entity}/{id}` profile route is a profile *of*; prefixes the section within it. */
const PROFILE_ROOTS: Record<string, string> = {
  user: 'User',
  tournament: 'Tournament',
  'tournament-group': 'Tournament Group',
  team: 'Team',
  building: 'Building',
  round: 'Round',
  division: 'Division',
  'division-session': 'Session',
  'pool-bracket': 'Pool / Bracket',
  room: 'Room',
  game: 'Game',
}

/** Section segments, shared by every profile page and by /crm. */
const SECTIONS: Record<string, string> = {
  overview: 'Overview',
  schedule: 'Schedule',
  register: 'Registration',
  divisions: 'Divisions',
  'division-sessions': 'Sessions',
  sessions: 'Sessions',
  pools: 'Pools',
  brackets: 'Brackets',
  buildings: 'Buildings',
  rooms: 'Rooms',
  teams: 'Teams',
  quizzers: 'Quizzers',
  rounds: 'Rounds',
  games: 'Games',
  gear: 'Gear',
  admins: 'Admins',
  tournaments: 'Tournaments',
  'tournament-groups': 'Groups',
  'stats-groups': 'Stats Groups',
  'room-monitor': 'Room Monitor',
  permissions: 'Permissions',
  'change-password': 'Change Password',
  'my-rosters': 'Rosters',
  'my-gear': 'Gear',
  'as-admin': 'As Admin',
  'as-quizmaster': 'As Quizmaster',
  'as-content-judge': 'As Content Judge',
  'managed-tournaments': 'Managed Tournaments',
  'managed-tournament-groups': 'Managed Tournament Groups',
  users: 'Users',
  'create-tournament-applicants': 'Tournament Applicants',
  gameevents: 'Game Events',
}

export function titleForPath(pathname: string): string {
  const path = pathname.replace(/\/+$/, '') || '/'
  if (path in STANDALONE) return format(STANDALONE[path])

  const [first, second, third] = path.split('/').filter(Boolean)

  // Documentation: an article's own title when the slug resolves, else the manual.
  const category = docCategoryByPath(first)
  if (category) {
    const article = third ? docArticle(resolveVersion(category, second), third) : undefined
    return format(article?.title ?? `${category.title} Docs`)
  }

  // /crm/{section} — sections sit directly under /crm, with no id in between.
  if (first === 'crm') {
    const section = second ? SECTIONS[second] : undefined
    return format(section ? `CRM ${section}` : 'CRM')
  }

  // /help/{slug} redirects into the documentation, so it only shows while bouncing.
  if (first === 'help') return format('Help Center')

  // /{entity}/{id}[/{section}] — the entity names the profile, the section names the part of it
  // on screen: "Division Games".
  const root = PROFILE_ROOTS[first]
  if (root) {
    const section = third ? SECTIONS[third] : undefined
    return format(section ? `${root} ${section}` : root)
  }

  return format('Page Not Found')
}
