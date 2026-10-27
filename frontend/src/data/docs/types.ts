// ── Documentation data model ─────────────────────────────────────────────────
//
// The Help Center is split into three documentation categories (QView, QuizMachine,
// QMServer). Each category holds one or more versions, and each version carries its own
// navigation tree and article set — so a future manual can restructure its sections
// without disturbing the versions already published.
//
// URLs are `/{category.path}/{version}/{slug}`, e.g. `/quizmachine-docs/6.0.0/compatibility`.

export type DocCategoryId = 'qview' | 'quizmachine' | 'qmserver'

export interface DocArticle {
  slug: string
  title: string
  /**
   * Raw HTML. Trusted authored content (rendered via dangerouslySetInnerHTML).
   * Internal links use the `doc:` scheme so they survive version/category changes —
   * see `resolveDocHref` in ./index.
   *
   * Omitted for a members-only article (`requiresLogin` in the nav) whose content is served by
   * the backend instead (`/api/helpdocs/...`), so it never ships in the bundle.
   */
  contentHtml?: string
}

/** A node in the sidebar tree: a link to an article (`slug`) or a collapsible folder (`children`). */
export interface DocNavNode {
  title: string
  slug?: string
  children?: DocNavNode[]
  /**
   * Articles at or under this node show their content only to logged-in users; visitors still
   * see the titles in the sidebar. Leave such an article's `contentHtml` out and put its HTML in
   * backend/help-docs/ so the backend serves it only to logged-in users; an article that keeps
   * its `contentHtml` is merely hidden on the page, since the text still ships in the bundle.
   */
  requiresLogin?: boolean
}

export interface DocVersion {
  /** Version string as it appears in the URL, e.g. "6.0.0". */
  version: string
  /** Release date of this manual, shown beside the version in the selector. */
  released?: string
  /** True while this version's articles are still being migrated from the source manual. */
  draft?: boolean
  nav: DocNavNode[]
  articles: DocArticle[]
}

export interface DocCategory {
  id: DocCategoryId
  /** Display name, e.g. "QuizMachine". */
  title: string
  /** URL segment, e.g. "quizmachine-docs". */
  path: string
  /** One-line description used on the Help landing cards. */
  description: string
  /** Newest version first — the first entry is treated as the default. */
  versions: DocVersion[]
}

/**
 * Builds a placeholder article body for a section whose prose hasn't been migrated yet.
 * `seeAlso` entries become real cross-references using the `doc:` link scheme, which is how
 * the source manual's bookmark links ("see this section (link)") are preserved.
 */
export function makeStub(sourceLabel: string) {
  return (title: string, opts?: { summary?: string; seeAlso?: { label: string; ref: string }[] }): string => {
    const summary = opts?.summary
      ? `<p>${opts.summary}</p>`
      : `<p>This section covers <em>${title}</em>.</p>`
    const seeAlso = opts?.seeAlso?.length
      ? `<h2>See also</h2>\n<ul>\n${opts.seeAlso
          .map((s) => `  <li><a href="${s.ref}">${s.label}</a></li>`)
          .join('\n')}\n</ul>`
      : ''
    return [
      `<p class="doc-draft-note">The full text of this section is being migrated from the ${sourceLabel}. The outline below is in place; the complete instructions are coming next.</p>`,
      summary,
      seeAlso,
    ]
      .filter(Boolean)
      .join('\n\n')
  }
}
