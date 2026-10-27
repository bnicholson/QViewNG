// Members-only Help Center articles, served by the backend to logged-in users (see
// backend/src/services/helpdoc.rs). Their screenshot links come back signed and expiring.

interface HelpDocResponse {
  html: string;
}

/** Articles already fetched this session. Kept well inside the server's minimum 1-hour image-link
 *  lifetime, so a cached article never shows screenshots whose links have expired. */
const CACHE_TTL_MS = 45 * 60 * 1000;
const cache = new Map<string, { html: string; at: number }>();

export const HelpDocAPI = {
  getArticle: async (categoryId: string, version: string, slug: string, accessToken: string): Promise<string> => {
    const key = `${categoryId}/${version}/${slug}`;
    const hit = cache.get(key);
    if (hit && Date.now() - hit.at < CACHE_TTL_MS) return hit.html;
    const response = await fetch(`/api/helpdocs/${key}`, {
      headers: { Authorization: `Bearer ${accessToken}` },
    });
    if (!response.ok) throw new Error(`Failed to load article (${response.status})`);
    const { html } = (await response.json()) as HelpDocResponse;
    cache.set(key, { html, at: Date.now() });
    return html;
  },
};
