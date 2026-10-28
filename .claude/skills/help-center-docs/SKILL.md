---
name: help-center-docs
description: Use whenever creating or modifying Help Center documentation and articles.
---

# Writing Help Center articles

The Help Center lives in `frontend/src/data/docs/`. There's one file per category and version (`qview.ts`, `quizmachine-6.0.0.ts`, `qmserver-6.0.0.ts`, …), and the data model is in `types.ts`. Each article is a `DocArticle` with raw HTML in `contentHtml`, and each version's `nav` tree controls the sidebar. Before writing anything, read a few existing articles in the QuizMachine and QMServer 6.0.0 files. They are the reference for voice and structure.

### Members-only articles live in the backend
A nav folder marked `requiresLogin: true` (in QView 0.1.0.0: **Setting Up a Tournament** and **Tournament Day**) holds members-only articles. Their content is not in the frontend bundle, so scrapers and logged-out visitors can't read it:
- The article's HTML is a file, `backend/help-docs/{category}/{version}/{slug}.html`. Its screenshots go in `backend/help-docs/{category}/{version}/images/` and are referenced by bare file name: `<img src="{slug}-{n}.webp" …>`. The backend serves the article to logged-in users only and turns those names into signed, expiring links (see `backend/src/services/helpdoc.rs`).
- Its entry in `articles` has a `slug` and `title` but **no** `contentHtml`. The nav node stays in the frontend as usual, so the title still shows in the sidebar.
- Write the HTML out in full: there's no `shot()` helper in these files, so write each `<figure>` by hand. `doc:` links work as usual.
- Moving an article into or out of a members-only folder means moving its HTML and screenshots between the frontend and `backend/help-docs/` too.
- After adding or changing one, run `cargo test --test it helpdoc_test::` in `backend/`. It loads every article there and every screenshot link in it, so a missing image fails the test.

## Step 1: Find the gaps first, and get approval

Before writing or changing any article, compare what the codebase can actually do with what the docs cover for that category:

- Walk the relevant pages, routes, and components (`frontend/src/pages`, `frontend/src/components`) and the permissions/roles that control who sees what.
- Note features, workflows, or roles with no article, articles describing things that no longer exist or work differently, and placeholder/lorem/stub articles.

### Where does new content go?
For each gap, decide whether it belongs in an existing article or needs a new one. Default to an existing article; only create a new one when one of the "new article" conditions applies.

**Put it in an existing article when:**
- The reader needs it to finish that article's workflow, e.g. a new required field or an extra step.
- It's a small variation of that workflow (a different option, an edge case, a "what if it doesn't work"). It can be a short section or a `doc-note`.
- It's something the reader would expect to find there, so leaving it out would leave a hole.

**Create a new article when:**
- It's a workflow of its own, with its own goal and "you're done when…", even if it starts from the same screen.
- It's for a different kind of user (role) than the existing article's reader.
- Adding it would distract from the existing workflow: the reader would have to skip past it to finish what they came to do.
- Several articles would need it. Write it once and link to it from each with `doc:` links, instead of repeating it.
- It would make the existing article too long to take in at once. In that case, split the article into focused pieces.

**When it's a close call**, keep the main path in the workflow article: one or two sentences and a link. Put the detail in its own article.

**When you create a new article**, it needs both a node in `nav` and an entry in `articles`. If it goes in a members-only (`requiresLogin`) folder, it also needs its HTML file in `backend/help-docs/` (see [Members-only articles](#members-only-articles-live-in-the-backend)). Add the `doc:` links to it from the articles it was split from or that point to it.

Then **stop and give the user a short numbered overview** of the gaps. For each one, include:
- What it is, in one line.
- Who needs it (which kind of user/role).
- Whether it's a new article, an update, or a removal, and where it would go in the nav.
- A one-line reason for where it goes, e.g. "new article: separate goal from Creating a Tournament", so the user can approve or deny the placement as well as the item.

Let the user approve or deny each item. Only write the approved ones.

## Step 2: Write for the reader

### Voice: a friend teaching a friend
Write like you're sitting next to a friend and want them to really *get it*, without overwhelming them.
- Talk to them directly ("you"), in plain words. Short sentences. No jargon unless the UI uses it, and then explain it once.
- Ask the questions they'd be asking, and use them as headings (e.g. "How do you know if data is coming across correctly?").
- Give them the *why* in a sentence when it helps them avoid a mistake, but don't lecture.
- Use concrete examples ("say round 5 is complete but round 4 isn't…") rather than abstract rules.
- Keep it short. If a sentence doesn't help them succeed, cut it.

### Organize around workflows, by user
Organize articles around **what a specific kind of user is trying to get done**, not around screens or features. For each workflow:
- Say who it's for and what they'll have when they're done.
- List anything they need first (permissions, data already loaded, another step done).
- Walk through it as numbered steps, one action per step, with UI labels in **bold** exactly as they appear on screen.
- Finish with how they can tell it worked, then the common ways it goes wrong and how to fix each one.

### Show, don't tell: screenshots first
Lead with the screenshot and let it do most of the explaining. Text should point at what's in the picture, not describe the screen in words. Aim for a screenshot at each meaningful step: where to click, what appears, and what success looks like.

Existing screenshots are real application captures, cropped to the relevant area, with red callout boxes and red arrows pointing at what matters. How you get screenshots depends on the category.

#### QuizMachine and QMServer: always placeholders
These are Windows desktop apps Claude can't run or capture. Always use the [placeholder approach](#placeholder-approach) below.

#### QView: capture them yourself (tier 1), placeholders as a fallback (tier 2)
QView is the web app in this repo, so try to capture real screenshots first.

**Pre-flight check.** Before writing the article, confirm that you have everything you need:
- **The app is running.** The frontend is the Vite dev server, usually at `http://localhost:5173` (`npm run dev` in `frontend/`). The backend is at `http://localhost:3000` (`APP_PORT`), proxied through `/api`. Check that both respond, e.g. with `curl`.
- **A way to drive a browser.** Logged-in pages need browser automation, such as the Claude in Chrome or built-in browser tools if this session has them, or Playwright. Plain headless `chromium --screenshot` only works for pages that don't need a login.
- **Image tools.** `magick` (ImageMagick) for cropping, adding callouts, and converting to `.webp`.
- **A login with the right role.** Use an account whose permissions match the user the article is for (admin, tournament director, coach, and so on). Ask the user for credentials; never guess them or create accounts on your own.
- **Realistic demo data.** The screens need data that looks like a real tournament, so the screenshots make sense. Don't create or change data in the database without asking. Never capture real people's personal info.

**If anything is missing, stop and tell the user before writing.** List exactly what's missing (e.g. "the backend isn't responding on :3000", "no browser automation available", "I need a coach-level login", "there are no teams in the database to show"), and say how they could fix each one. Then wait.
- If the user fixes it, re-run the check and carry on with tier 1.
- If the user says they can't, use the [placeholder approach](#placeholder-approach) for QView too.

**Capturing.** Match the look of the existing screenshots:
- Use one consistent browser size for the whole article: a desktop width (around 1280px) by default, or a phone width when the workflow is a mobile one. Keep the app's default theme throughout.
- Always capture at **2× resolution** (device scale factor 2, e.g. `--force-device-scale-factor=2` for Chromium, `deviceScaleFactor: 2` for Playwright), so screenshots stay sharp on high-resolution screens. Pick the browser size in normal (1×) pixels; the image itself will be twice that size.
- Crop to the part of the screen that matters, with just enough around it that the reader can tell where they are.
- Add the callouts with ImageMagick: red rectangles (pure red, 6px on the 2× image so they look like 3px on screen) around what to click or look at, and red arrows when you need to connect two things.
- Save to `frontend/public/docs/qview/{slug}-{n}.webp`, and reference it as `/docs/qview/{slug}-{n}.webp`. For a members-only article, save it to `backend/help-docs/qview/{version}/images/{slug}-{n}.webp` and reference it as `{slug}-{n}.webp` instead (see [Members-only articles](#members-only-articles-live-in-the-backend)).
- Look at each finished image yourself and confirm it really shows what the caption says. If a capture doesn't work, use a placeholder for that one and say so.
- At the end, list every screenshot you captured and every placeholder left for the user.

#### Placeholder approach
**Write the article as if the screenshots are already there**:

1. Add the full `<figure>` in place, using the normal naming convention: `/docs/{category-version}/{slug}-{n}.webp` (QView uses `/docs/qview/{slug}-{n}.webp`), numbered in order within the article.
2. Make `alt` and `<figcaption>` the same short caption, matching existing articles.
3. Directly above each figure, add an HTML comment describing what the screenshot should show, including what to highlight with the red callouts:
   ```html
   <!-- SCREENSHOT TODO: Tournament page, Divisions tab. Red box around the "Add Division" button; red arrow from it to the empty division row that appears. -->
   <figure>
     <img src="/docs/qview/divisions-and-rooms-2.webp" alt="Adding a new division from the Divisions tab." loading="lazy" />
     <figcaption>Adding a new division from the Divisions tab.</figcaption>
   </figure>
   ```
4. When you're done, give the user a checklist of every screenshot needed: file path, and what it should show.

### Thorough, not long
Cover everything the user needs to succeed on their own, with no need to ask anyone. Include edge cases, gotchas, and "what if it doesn't work" for every workflow, but keep each one as short as possible:
- Use `<p class="doc-note">` for helpful tips and things that are easy to miss.
- Use `<p class="doc-warning">` for anything that can lose data or break something.
- Split a long topic into several focused articles rather than one wall of text (see [Where does new content go?](#where-does-new-content-go)).
- Link instead of repeating. End with a `<h2>See also</h2>` list of related articles.

## Markup conventions
- HTML only: `<p>`, `<h2>`/`<h3>`, `<ol>` for steps, `<ul>` for options/lists, `<strong>` for UI labels, and `<figure>` for screenshots.
- Internal links use the `doc:` scheme: `doc:slug` within the same version, `doc:category/slug` across categories (see `resolveDocHref` in `index.ts`). Never hard-code URLs.
- Escape `&gt;` / `&lt;` / `&amp;` in menu paths like `Setup &gt; General`.
- Every new article needs both an entry in `articles` and a node in `nav` (for a members-only article, the `articles` entry has no `contentHtml`; its HTML goes in `backend/help-docs/`). When an article stops being a placeholder, remove `draft` from the version once all its articles are real.

## Final check: spelling and casing
As the last step, after everything else is done, proofread all newly added or modified text: article titles, nav titles, headings, body text, captions and `alt` text. Correct every spelling mistake and casing error before handing the work back. This applies to content imported "verbatim" from the user's drafts too: verbatim means the wording, not the typos.
- **Spelling:** fix typos and misspellings (e.g. "GIve" → "Give", "wil" → "will"), doubled words ("the the"), and verb/noun mix-ups like "login" (noun) vs. "log in" (verb).
- **Casing:** capitalize QView entity names consistently when they mean the QView thing: Tournament, Division, Round, Game, Room, Team, Quizzer, Roster, Gear, Gear Set. Leave generic phrases ("tournament day", "quiz room") lowercase.
- **Product and UI names:** write QView, QuizMachine and QMServer exactly like that. Write UI labels exactly as they appear on screen, even when the app itself has a typo, and tell the user about the app's typo so it can be fixed at the source.
- List what you corrected in your report to the user.
