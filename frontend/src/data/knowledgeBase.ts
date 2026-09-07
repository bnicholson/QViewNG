// ── QView Help Knowledge Base ────────────────────────────────────────────────
//
// Placeholder content. Article bodies are stored as raw HTML strings so they can be
// authored/edited as HTML later (rendered via dangerouslySetInnerHTML in the article page).
// The sidebar structure (`kbNav`) is a nested tree — folders may contain other folders
// and/or articles, enabling semantic categorization.

export interface KbArticle {
  slug: string
  title: string
  /** Raw HTML. Trusted placeholder content for now; sanitize here if this ever becomes user-authored. */
  contentHtml: string
}

/** A node in the sidebar tree: a link to an article (`slug`) or a collapsible folder (`children`). */
export interface KbNavNode {
  title: string
  slug?: string
  children?: KbNavNode[]
}

// Lorem-ipsum HTML body builder so each placeholder article has some structure (headings/lists).
function lorem(title: string): string {
  return `
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor
incididunt ut labore et dolore magna aliqua. This is placeholder content for
<em>${title}</em> and will be replaced with real documentation later.</p>

<h2>Overview</h2>
<p>Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip
ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit
esse cillum dolore eu fugiat nulla pariatur.</p>

<h2>Key points</h2>
<ul>
  <li>Excepteur sint occaecat cupidatat non proident.</li>
  <li>Sunt in culpa qui officia deserunt mollit anim id est laborum.</li>
  <li>Sed ut perspiciatis unde omnis iste natus error sit voluptatem.</li>
</ul>

<h2>Details</h2>
<p>Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia
consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro
quisquam est, qui dolorem ipsum quia dolor sit amet.</p>
<p>At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium
voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint.</p>
`.trim()
}

export const kbArticles: KbArticle[] = [
  { slug: 'welcome', title: 'Welcome to QView', contentHtml: lorem('Welcome to QView') },
  { slug: 'creating-an-account', title: 'Creating an Account', contentHtml: lorem('Creating an Account') },
  { slug: 'creating-a-tournament', title: 'Creating a Tournament', contentHtml: lorem('Creating a Tournament') },
  { slug: 'divisions-and-rooms', title: 'Adding Divisions & Rooms', contentHtml: lorem('Adding Divisions & Rooms') },
  { slug: 'running-games', title: 'Running Games', contentHtml: lorem('Running Games') },
  { slug: 'building-a-roster', title: 'Building a Roster', contentHtml: lorem('Building a Roster') },
  { slug: 'registering-teams', title: 'Registering Teams', contentHtml: lorem('Registering Teams') },
  { slug: 'managing-gear', title: 'Managing Your Gear', contentHtml: lorem('Managing Your Gear') },
]

// Sidebar tree — nested folders for semantic categorization.
export const kbNav: KbNavNode[] = [
  {
    title: 'Getting Started',
    children: [
      { title: 'Welcome to QView', slug: 'welcome' },
      { title: 'Creating an Account', slug: 'creating-an-account' },
    ],
  },
  {
    title: 'Tournaments',
    children: [
      {
        title: 'Managing Tournaments',
        children: [
          { title: 'Creating a Tournament', slug: 'creating-a-tournament' },
          { title: 'Adding Divisions & Rooms', slug: 'divisions-and-rooms' },
        ],
      },
      { title: 'Running Games', slug: 'running-games' },
    ],
  },
  {
    title: 'Teams & Rosters',
    children: [
      { title: 'Building a Roster', slug: 'building-a-roster' },
      { title: 'Registering Teams', slug: 'registering-teams' },
    ],
  },
  {
    title: 'Equipment',
    children: [
      { title: 'Managing Your Gear', slug: 'managing-gear' },
    ],
  },
]

export const kbArticleBySlug = (slug: string): KbArticle | undefined =>
  kbArticles.find((a) => a.slug === slug)

/** Ordered flat list of article slugs (tree order) — handy for prev/next and lookups. */
export const kbOrderedSlugs: string[] = (() => {
  const out: string[] = []
  const walk = (nodes: KbNavNode[]) => nodes.forEach((n) => {
    if (n.slug) out.push(n.slug)
    if (n.children) walk(n.children)
  })
  walk(kbNav)
  return out
})()
