// ── Documentation registry ───────────────────────────────────────────────────
//
// Adding a manual is a data-only change: create a `DocVersion` module and list it (newest
// first) under the right category below.

import type { DocArticle, DocCategory, DocCategoryId, DocNavNode, DocVersion } from './types'
import { qview0100 } from './qview-0.1.0.0'
import { qviewCurrent } from './qview'
import { quizmachine600 } from './quizmachine-6.0.0'
import { quizmachine500 } from './quizmachine-5.0.0'
import { qmserver600 } from './qmserver-6.0.0'
import { qmserver500 } from './qmserver-5.0.0'

export type { DocArticle, DocCategory, DocCategoryId, DocNavNode, DocVersion } from './types'

/** The category shown when none is specified (e.g. the bare /help landing redirect). */
export const DEFAULT_CATEGORY_ID: DocCategoryId = 'qview'

export const docCategories: DocCategory[] = [
  {
    id: 'qview',
    title: 'QView',
    path: 'qview-docs',
    description:
      'Running tournaments in the QView web app — accounts, tournaments, divisions and rooms, games, rosters and gear.',
    versions: [qview0100, qviewCurrent],
  },
  {
    id: 'quizmachine',
    title: 'QuizMachine',
    path: 'quizmachine-docs',
    description:
      'The desktop jump-and-scoring program: installing and registering it, the scoring interface, quizmastering a round, and working with quiz data.',
    versions: [quizmachine600, quizmachine500],
  },
  {
    id: 'qmserver',
    title: 'QMServer',
    path: 'qmserver-docs',
    description:
      'The QuizMachine companion server: running a quiz meet, moving names/schedule and quiz round data, and generating statistics and awards.',
    versions: [qmserver600, qmserver500],
  },
]

export const docCategoryById = (id: string | undefined): DocCategory | undefined =>
  docCategories.find((c) => c.id === id)

export const docCategoryByPath = (path: string | undefined): DocCategory | undefined =>
  docCategories.find((c) => c.path === path)

/** The default (newest) version of a category. */
export const latestVersion = (category: DocCategory): DocVersion => category.versions[0]

/** The requested version, or the newest one when the request doesn't match. */
export const resolveVersion = (category: DocCategory, version: string | undefined): DocVersion =>
  category.versions.find((v) => v.version === version) ?? latestVersion(category)

/** True only when `version` names a real published version of the category. */
export const isKnownVersion = (category: DocCategory, version: string | undefined): boolean =>
  category.versions.some((v) => v.version === version)

export const docArticle = (docVersion: DocVersion, slug: string | undefined): DocArticle | undefined =>
  slug ? docVersion.articles.find((a) => a.slug === slug) : undefined

/** Ordered flat list of article slugs in sidebar (tree) order. */
export const orderedSlugs = (docVersion: DocVersion): string[] => {
  const out: string[] = []
  const walk = (nodes: DocNavNode[]) =>
    nodes.forEach((n) => {
      if (n.slug) out.push(n.slug)
      if (n.children) walk(n.children)
    })
  walk(docVersion.nav)
  return out
}

/** True when `slug`'s nav node, or any folder above it, is marked `requiresLogin`. */
export const articleRequiresLogin = (docVersion: DocVersion, slug: string): boolean => {
  const walk = (nodes: DocNavNode[], inherited: boolean): boolean | undefined => {
    for (const n of nodes) {
      const required = inherited || !!n.requiresLogin
      if (n.slug === slug) return required
      const found = n.children && walk(n.children, required)
      if (found !== undefined) return found
    }
    return undefined
  }
  return walk(docVersion.nav, false) ?? false
}

/**
 * The previous/next articles relative to `slug`, treating the version's whole hierarchy as one
 * flat list. Either side is undefined at the first/last article.
 */
export const adjacentArticles = (
  docVersion: DocVersion,
  slug: string,
): { prev?: DocArticle; next?: DocArticle } => {
  const slugs = orderedSlugs(docVersion)
  const idx = slugs.indexOf(slug)
  if (idx === -1) return {}
  const prevSlug = idx > 0 ? slugs[idx - 1] : undefined
  const nextSlug = idx < slugs.length - 1 ? slugs[idx + 1] : undefined
  return {
    prev: prevSlug ? docArticle(docVersion, prevSlug) : undefined,
    next: nextSlug ? docArticle(docVersion, nextSlug) : undefined,
  }
}

/** Build a docs URL: `/{category}/{version}` or `/{category}/{version}/{slug}`. */
export const docPath = (category: DocCategory, version: string, slug?: string): string =>
  `/${category.path}/${version}${slug ? `/${slug}` : ''}`

/**
 * Resolves a `doc:` href used inside article HTML into a real app path. This is how the source
 * manuals' bookmark links ("see this section (link)") survive as working cross-references.
 *
 *   doc:compatibility                  → same category, same version
 *   doc:qmserver/getting-stats         → another category; keeps the same version when that
 *                                        category publishes one, else falls back to its newest
 *   doc:quizmachine@5.4/compatibility  → another category at an explicit version
 *   doc:                               → the current category/version index ("Back to Contents")
 *
 * Returns null for anything that isn't a `doc:` href, so ordinary links pass through untouched.
 */
export const resolveDocHref = (
  href: string,
  currentCategory: DocCategory,
  currentVersion: string,
): string | null => {
  if (!href.startsWith('doc:')) return null
  const body = href.slice('doc:'.length)
  if (!body) return docPath(currentCategory, currentVersion)

  const parts = body.split('/')
  // Bare slug → an article in the category/version being read.
  if (parts.length === 1) return docPath(currentCategory, currentVersion, parts[0])

  const [target, ...rest] = parts
  const slug = rest.join('/')
  const [categoryId, explicitVersion] = target.split('@')
  const category = docCategoryById(categoryId)
  if (!category) return null

  // Prefer the matching version so QuizMachine 6.0.0 links land on QMServer 6.0.0.
  const version = explicitVersion && isKnownVersion(category, explicitVersion)
    ? explicitVersion
    : isKnownVersion(category, currentVersion)
      ? currentVersion
      : latestVersion(category).version

  return docPath(category, version, slug || undefined)
}
