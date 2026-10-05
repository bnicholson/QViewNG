// QView documentation — the web app's own help. QView is continuously deployed, so its docs
// track the running app version (from version.json) rather than a separately published manual.

import versionInfo from '../../version.json'
import type { DocVersion } from './types'

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
`.trim()
}

export const qviewCurrent: DocVersion = {
  version: versionInfo.version,
  draft: true,
  nav: [
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
      children: [{ title: 'Managing Your Gear', slug: 'managing-gear' }],
    },
  ],
  articles: [
    { slug: 'welcome', title: 'Welcome to QView', contentHtml: lorem('Welcome to QView') },
    { slug: 'creating-an-account', title: 'Creating an Account', contentHtml: lorem('Creating an Account') },
    { slug: 'creating-a-tournament', title: 'Creating a Tournament', contentHtml: lorem('Creating a Tournament') },
    { slug: 'divisions-and-rooms', title: 'Adding Divisions & Rooms', contentHtml: lorem('Adding Divisions & Rooms') },
    { slug: 'running-games', title: 'Running Games', contentHtml: lorem('Running Games') },
    { slug: 'building-a-roster', title: 'Building a Roster', contentHtml: lorem('Building a Roster') },
    { slug: 'registering-teams', title: 'Registering Teams', contentHtml: lorem('Registering Teams') },
    { slug: 'managing-gear', title: 'Managing Your Gear', contentHtml: lorem('Managing Your Gear') },
  ],
}
