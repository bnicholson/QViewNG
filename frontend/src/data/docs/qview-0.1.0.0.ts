// QView 0.1.0.0 — the first published QView manual, imported from the "QView Docs Drafting 20261026"
// drafts. Articles are grouped by who is doing the work: every member, coaches, tournament directors
// setting up a tournament, and whoever runs the rooms and stats on tournament day.

import type { DocVersion } from './types'

/** A screenshot from /docs/qview, where `caption` doubles as the alt text. */
function shot(file: string, caption: string): string {
  return `<figure>
  <img src="/docs/qview/${file}.webp" alt="${caption}" loading="lazy" />
  <figcaption>${caption}</figcaption>
</figure>`
}

export const qview0100: DocVersion = {
  version: '0.1.0.0',
  released: 'October 26, 2026',
  nav: [
    {
      title: 'Your Account',
      children: [
        { title: 'Creating a QView Account', slug: 'creating-an-account' },
        { title: 'Resetting or Recovering Your Password', slug: 'recovering-your-password' },
      ],
    },
    {
      title: 'For Coaches',
      children: [
        { title: 'Creating Rosters', slug: 'creating-rosters' },
        { title: 'Adding Your Gear', slug: 'adding-gear' },
        { title: 'Registering for a Tournament', slug: 'registering-for-a-tournament' },
      ],
    },
    {
      title: 'Setting Up a Tournament',
      requiresLogin: true,
      children: [
        { title: 'Becoming a Tournament Manager', slug: 'applying-to-be-a-tournament-manager' },
        { title: 'Creating a Tournament and Its Rooms', slug: 'creating-a-tournament' },
        { title: 'Divisions and Registration', slug: 'divisions-and-registration' },
        { title: 'Scheduling Rounds and Games', slug: 'rounds-and-games' },
        { title: 'Setting Question Types for Rounds', slug: 'setting-question-types' },
      ],
    },
    {
      title: 'Tournament Day',
      requiresLogin: true,
      children: [
        { title: 'Connecting QuizMachine to QView', slug: 'setting-up-rooms' },
        { title: 'Monitoring Rooms and Resending Game Data', slug: 'monitoring-rooms' },
        { title: 'Stats Groups, Importing and Exporting Results', slug: 'computing-stats' },
      ],
    },
  ],
  articles: [
    // ── Your Account ────────────────────────────────────────────────────────────
    {
      slug: 'creating-an-account',
      title: 'Creating a QView Account',
      contentHtml: `
<p>Anyone can use QView to view Tournaments and watch the action that unfolds, however, several features such as
registering Teams for Tournaments, creating/hosting Tournaments and more require having a QView account.</p>

<p>To create a QView account, first click this icon which, when hovered over, says “Login”:</p>
${shot('creating-an-account-1', 'The Login icon in the top-right corner of every page.')}

<p>This will take you to the login page. To create an account, click this link on the Login page:</p>
${shot('creating-an-account-2', 'The “Register here” link on the Login page.')}

<p>You will then be taken to the Registration page. Fill out the fields and then click “Register”.</p>
${shot('creating-an-account-3', 'The Registration form.')}
${shot('creating-an-account-4', 'The Register button at the bottom of the Registration form.')}

<p>An email will be sent to the email address you provided. To confirm your email address and activate your QView
account, copy the activation code, paste it in the current page and click “Activate”:</p>
${shot('creating-an-account-5', 'Pasting the activation token and clicking Activate.')}

<p>Upon receiving a success message, you can then navigate back to the login page, log in with your new account
credentials, and begin using user-based features of QView!</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:recovering-your-password">Resetting/Recovering Your Password</a></li>
  <li><a href="doc:creating-rosters">Creating Rosters (Preparing to Register Teams)</a></li>
  <li><a href="doc:applying-to-be-a-tournament-manager">Applying to Be a Tournament Manager (Creator/Owner)</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'recovering-your-password',
      title: 'Resetting/Recovering Your Password',
      contentHtml: `
<p>You can recover your password from the login page if you happen to have lost or forgotten your QView account
password. You can also reset your password if you know your current login credentials.</p>

<h2>Resetting Your Password from your User Profile</h2>
<p>First, log in as your user. This will direct you to your user profile upon logging in.</p>
<p>In the left-hand menu, click “Change Password”. You will see a page like this:</p>
${shot('recovering-your-password-1', 'Change Password in the user profile sidebar.')}
${shot('recovering-your-password-2', 'The Change Password form.')}
<p>Enter your “Current Password” followed by the “New Password” that you want your account to have. Then click
“Change Password”. The effects will take place immediately and from then onward the new password will take
effect.</p>

<h2>Recovering a Lost or Forgotten Password</h2>
<p>To recover your account when you lose or forget your password, first go to the login page and click the
recover password link:</p>
${shot('recovering-your-password-3', 'The “Recover password” link on the Login page.')}
${shot('recovering-your-password-4', 'The Account Recovery form.')}
<p>Enter the email that is saved on your account and click “Send Recovery Email”.</p>
<p>If a QView account exists with that username + email address combination, this will send an email to that email
address with a link for recovering your account’s password.</p>
<p>If, however, the username + email address combination is not found in QView, it will not notify you that no
account was found.</p>
<p>Once you have used the recovery link to reset your password, navigate back to the login page and log in using
your new password.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:creating-an-account">Creating a QView Account</a></li>
</ul>
`.trim(),
    },

    // ── For Coaches ─────────────────────────────────────────────────────────────
    {
      slug: 'creating-rosters',
      title: 'Creating Rosters (Preparing to Register Teams)',
      contentHtml: `
<p>To register a Team for a Tournament, QView requires that you must first create a <strong>Roster</strong>.
Rosters are made up of your Quizzers, and each Roster is intended to form a single Team when registering for a
Tournament. The aim here is to make registration simpler by allowing Teams to be more organized and reusable.</p>

<p>To create a Roster, first log in with your QView login credentials.</p>
<p>Then in your user profile, click on “My Rosters”:</p>
${shot('creating-rosters-1', 'My Rosters in the user profile, before any Rosters exist.')}

<p>The first tab in the top row labelled “All Quizzers” is the tab where you can view all your Quizzers across all
of your Rosters. It starts out empty since you have not created any Rosters or added Quizzers to them yet.</p>
<p>Click either the plus icon in the tab bar or the “Create Roster” button to begin creating a Roster:</p>
${shot('creating-rosters-2', 'The Create Roster form.')}

<p>Give a name to your Roster and a description (for remembering later what the Roster was created for). Then click
“Save” and confirm.</p>
<p>The new Roster you created will now be available. Click its tab to view its details:</p>
${shot('creating-rosters-3', 'A new Roster’s tab, with its details, coaches and quizzers.')}

<p>There is now a 2nd tab next to the original “All Quizzers” tab called “Roster One”. This is our first and only
Roster at the moment. (The “All Quizzers” tab is intended only to allow you to see all of your Quizzers easily from
one place.)</p>
<p>Below the tabs we see the Roster’s description and when it was created. To the right in the same row we have the
options to edit the Roster’s details as well as delete the Roster.</p>
<p>Next we see “Shared with (Coaches)”. Here you can click the “Add Coach” button on the right and search for users
that you want to also have access to this Roster. When you add them, this Roster will appear in their Roster
list.</p>
<p>This is an optional feature; you do not need to add other coaches if you do not want to. It is purely a
convenience factor in case you want someone else to edit or use a Roster for registering for a Tournament.</p>
<p>Clicking “Add Coach” will bring up this dialog. Type at least 3 letters to begin searching for a user to
add:</p>
${shot('creating-rosters-4', 'Searching for a coach to add to the Roster.')}

<p>Here we have 1 result for our search. If we know the username of the user we’re looking for, we can check to make
sure this is the right user. If we’re still not sure we can click the user’s name and it will take us to their
profile in a separate tab (*the name being in blue text tells us this user’s name is a navigable link; this is true
in nearly all places in QView).</p>
<p>Since this is the user we want to add, we can click “Add” and it will add this user as a coach:</p>
${shot('creating-rosters-5', 'The added coach under “Shared With (Coaches)”.')}

<p>If we ever want to remove a coach from the Roster, we just click the <strong>trash can icon</strong> next to
their name.</p>
<p>But what is a Roster without any Quizzers? Let’s add some. Since we don’t have any Quizzers at all, we’ll need to
create a new one. We click “Create New User” to open the user/quizzer creation form:</p>
${shot('creating-rosters-6', 'The Create New Quizzer form.')}

<p>Note that here what you are actually doing is creating a brand new user! If you want to give them login access
you can provide a username, email address and a default password. Then you can share these login credentials with
your Quizzer so that they can view their Tournament results in their own profile once the Tournament they are
registered for has concluded.</p>
<p>At this time, the only way to add Quizzers to a Roster is to create a new user.</p>
${shot('creating-rosters-7', 'The Roster’s quizzers after adding two.')}

<p>If you delete a Quizzer that you created from a Roster, even if they no longer exist in any of your Rosters, they
still exist as a user in QView and as one of your Quizzers.</p>
<p>You can view your Quizzers in the ‘My Quizzers’ tab even after deleting them:</p>
${shot('creating-rosters-8', 'Deleting a quizzer from the Roster.')}
${shot('creating-rosters-9', 'Confirming the deletion.')}
${shot('creating-rosters-10', 'The Roster after the quizzer is removed.')}
${shot('creating-rosters-11', 'The removed quizzer still listed in the My Quizzers tab.')}

<p>You can also add any of your Quizzers to a new Roster:</p>
${shot('creating-rosters-12', 'Adding existing quizzers to another Roster.')}
${shot('creating-rosters-13', 'The second Roster with its quizzers.')}

<p>With a fully-created Roster, you are ready to register your Team(s) for a Tournament!</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:adding-gear">Adding Gear to Your Account (for Tournament Registration)</a></li>
  <li><a href="doc:registering-for-a-tournament">Registering for a Tournament: Teams, Gear and Volunteers</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'adding-gear',
      title: 'Adding Gear to Your Account (for Tournament Registration)',
      contentHtml: `
<p>QView attempts to reduce repetitive work for you by allowing you to manage sets of Gear so that when you register
for a Tournament you can register your set(s) over and over again with each Tournament that you bring your Gear
to.</p>
<p>QView also requires this, so you will need to create Gear Sets before registering your Gear for a
Tournament.</p>

<p>First, log in and navigate in your user profile to “My Gear”. You will find something that looks like the
following upon first arrival:</p>
${shot('adding-gear-1', 'My Gear in the user profile, before any Gear Sets exist.')}

<p>In this part of your profile you will be able to view each Gear Set as its own tab at the top of the screen and
view all of your Gear using the first tab in the row.</p>
<p>To get started, click either the “Create Gear Set” button or the plus icon in the tab row.</p>
<p>This will open the following form:</p>
${shot('adding-gear-2', 'The Create New GearSet form.')}

<p>Give your Gear Set a name and description, click “Save” and confirm. Your Gear Set will look something like this
starting out:</p>
${shot('adding-gear-3', 'A new, empty Gear Set.')}

<p>Our Gear Set is called “Gear Set 1”, its description is “Primary Gear Set” and we start out with no Gear Items.
We can edit or delete the Gear Set with the buttons in the upper right corner.</p>
<p>Let’s add some items to the Gear Set. Click “Add Gear” to open the Gear creation form:</p>
${shot('adding-gear-4', 'The Add Gear Item form with the Computer type selected.')}

<p>The first dropdown determines which Gear Set to assign the piece of Gear to. Each piece of Gear can only be
assigned to a single Gear Set.</p>
<p>Below that we have the different types of Gear QView can record in your profile. Here we selected the “Computer”
Gear type which revealed the fields relevant to adding a computer. Selecting a different type of Gear would make the
fields available to us change.</p>
<p>Enter the information for each piece of Gear that you have for this Gear Set, click “Save” and confirm for
each.</p>
<p>Soon enough you’ll have a more or less complete Gear Set for a quiz room:</p>
${shot('adding-gear-5', 'A Gear Set with its Gear Items.')}

<p>Cool, we’ve got a full Gear Set. If we click “All Gear” we can view the Gear that we manage across all of our
Gear Sets from a single place as well as manage them there.</p>
<p>What if we didn’t want the computer to be a part of this Gear Set but instead part of a different one? Then we
can move it either by opening its edit form or we can click the “Move” icon in the “All Gear” tab which shows us
which Gear Sets we can move the item to.</p>
<p>Here we’ve created a second Gear Set called “Gear Set 2”. Let’s move it:</p>
${shot('adding-gear-6', 'Moving a Gear Item to another Gear Set from the All Gear tab.')}

<p>After doing this, we can see the Gear item changed Gear Sets, from the “All Gear” tab and in that Gear Set’s
profile:</p>
${shot('adding-gear-7', 'The All Gear tab showing the item’s new Gear Set.')}
${shot('adding-gear-8', 'The moved item in Gear Set 2.')}

<p>If you ever have a piece of Gear that is decommissioned (breaks or gets lost), you can always delete the item from
the Gear Set using the red trash can icon and then replace it with a different one (if it needs replacing).</p>
<p>Now you know how to manage your Gear in QView! Once entered, you’re ready to register your Gear for a
Tournament.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:registering-for-a-tournament">Registering for a Tournament: Teams, Gear and Volunteers</a></li>
  <li><a href="doc:creating-rosters">Creating Rosters (Preparing to Register Teams)</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'registering-for-a-tournament',
      title: 'Registering for a Tournament: Teams, Gear and Volunteers',
      contentHtml: `
<p>Your event director asked you to register your Team(s) and Gear on this new platform he/she is using called
“QView” and you have no idea what you need to do.</p>
<p>Do not fear: This article is here!</p>
<p>For real though, you’re in the right place if you need to know how to register your Team(s) and/or Gear for a
Tournament on QView.</p>
<p>To start, search for the Tournament you are looking for to navigate into the Tournament that your event director
has invited you to register for.</p>
<p>You will be looking for some Tournament and page that looks similar to this (except with different information).
Click it to view its profile:</p>
${shot('registering-for-a-tournament-1', 'Finding the Tournament on the Home page.')}
${shot('registering-for-a-tournament-2', 'The Tournament’s Overview, with the Register button and Registration sidebar item.')}

<p>If you can’t find the Tournament you are looking to register for, check with your event director to ensure that
the Tournament is <strong>publicly visible</strong> (*a setting that Tournaments have which can make a Tournament
‘visible’ or ‘not visible’ to the general public).</p>
<p>If you see the “Registration” item in the sidebar and the “Register” button under the Tournament, then today’s
date currently falls within the Registration Window and you can register your Teams and Gear. You can view the
Registration Window’s dates from the Tournament Overview information (*shown above).</p>
<p>If you don’t see these items here, then you should also see that the Registration Window is “CLOSED” and today’s
date does not fall within the date range. It is also possible that the Tournament director has chosen not to use
registration at all, in which case the “Registration” will show as “None”. For example:</p>
${shot('registering-for-a-tournament-3', 'A Tournament whose Registration shows as “None”.')}

<p>If you believe the Registration Window may be incorrect or if registration has closed and you need to register,
please reach out to your event director using the contact information on this same page.</p>
<p>Click “Registration” or “Register” to proceed.</p>

<h2>Registering Teams</h2>
${shot('registering-for-a-tournament-4', 'The Team registration tab.')}
<p>If you do not see the “Team” button/tab, this likely means your Tournament director has chosen to not use the Team
registration feature for this Tournament.</p>
<p>Above we see three different registration types: Team, Gear and Volunteer. If you don’t see all of these then it
is likely your Tournament director chose to not use one or more of these registration options for this
Tournament.</p>
<p>When on “Team”, if you see the above screen, this means you are ready to register your Team(s).</p>
<p>If at any time you see the message below for any of the registration types, this means you are not logged in as
your user, and you need to before registering anything:</p>
${shot('registering-for-a-tournament-5', 'The message shown when you aren’t signed in.')}

<p>If you see the message below, this means you are logged in, but you have not created either a single Roster or a
single Gear Set, and you need to. In this case, click the link in the message to be taken to the location where you
can add the needed Roster or Gear Set (also see these articles on
<a href="doc:creating-rosters">how to create a Quizzer Roster</a> and
<a href="doc:adding-gear">how to create a Gear Set</a>).</p>
${shot('registering-for-a-tournament-6', 'The message shown when you have no Rosters yet.')}
<p>The above is the Team version of this message, while the below is the Gear version:</p>
${shot('registering-for-a-tournament-7', 'The message shown when you have no Gear yet.')}

<p>From here, we are going to assume you have correctly entered what you need for registration.</p>
<p>For Teams, click the “Register New Team” button on the right to start. You will see the following appear:</p>
${shot('registering-for-a-tournament-8', 'The Register New Team form.')}

<p>In the first row we have the Team Name, selected Division for the Team and the Coach which defaults to the logged
in user.</p>
<p class="doc-note">Note: Teams can only be in one Division, though Quizzers may still be on multiple Teams and
therefore multiple Divisions (if the scheduled Round times don’t conflict).</p>
<p>In the second row, we have the 6 slots for Quizzers that can be on the Team. They start out empty. You are going
to fill them with at least 1 Quizzer by using the third row.</p>
<p>In the third row, we see something which should be familiar: Tabs for viewing all of your Quizzers, and then
separate tabs for each of your Quizzer Rosters.</p>
<p>Looking at the image below, the most efficient path here is to</p>
<ol>
  <li>Select a Roster</li>
  <li>Click the “Assign All” button, thereby assigning all of the Quizzers of the Roster to the empty slots of the
  second row</li>
  <li>Click “Register Team”</li>
</ol>
<p>This is shown here:</p>
${shot('registering-for-a-tournament-9', 'Assigning a whole Roster to the Team with Assign All.')}

<p>You can remove single Quizzers from the Team by clicking the “X” next to their name:</p>
${shot('registering-for-a-tournament-10', 'Removing a quizzer from the Team.')}
<p>You can also add any Quizzer you want to the Team - the Quizzer doesn’t have to be from the same Roster:</p>
${shot('registering-for-a-tournament-11', 'Adding a quizzer from another Roster.')}
<p>Note that the order of the Quizzers in the slots does not matter - they can still be rearranged on a per-Game
basis.</p>
<p>When you’ve confirmed the changes you want, you can click “Register Team” to register the Team.</p>
<p>The result will look like this:</p>
${shot('registering-for-a-tournament-12', 'The registered Team in My Registered Teams.')}
<p>In the Actions column, you can use the <strong>pencil icon</strong> to edit this registration all the way up
until registration closes. The same for the <strong>trash can icon</strong>: You can delete/remove this Team
registration until the registration window closes.</p>

<h2>Registering Gear</h2>
<p>Navigate to the Tournament you want to register Gear for and then click the “Gear” button/tab. You should see
something like this:</p>
${shot('registering-for-a-tournament-13', 'The Gear registration tab.')}
<p>If you do not see the “Gear” button/tab, this likely means your Tournament director has chosen to not use the
Gear registration feature for this Tournament.</p>
<p>If you see the following, it means you are currently not logged in as a user:</p>
${shot('registering-for-a-tournament-14', 'The Gear message shown when you aren’t signed in.')}
<p>If you see the following, it means you have not added any Gear (which requires that you create a Gear Set). You
will need to add Gear in your profile before continuing:</p>
${shot('registering-for-a-tournament-15', 'The message shown when you have no Gear yet.')}

<p>Let’s revisit the user who has Gear and Gear Sets entered in their profile. Here the logged-in user has 2
Gear Sets:</p>
${shot('registering-for-a-tournament-16', 'Gear registration with two Gear Sets, labelled A to H.')}
<p>In the image above, we see:</p>
<ol type="A">
  <li>We are currently looking at the “Gear” registration section/tab for the Tournament “Tournament Two”.</li>
  <li>We have 2 Gear Sets. “Set 1” is collapsed while “Set 2” is expanded. Both sets currently have 8 pieces of Gear
  that can be registered for this Tournament, making a total of 16 available for registration, and zero (0) of
  them are currently registered.</li>
  <li>Clicking the “Manage My Gear” button takes you to your user’s “My Gear” page so that you can
  organize/reorganize/add/remove Gear to/from your account.</li>
  <li>Each Gear Set has a “Register Set” button that allows you to register all pieces of Gear for that set with a
  single click. When the set is registered, its text changes to “Unregister Set” at which time if you clicked it
  then it would unregister all pieces of the Gear Set.</li>
  <li>These icons (chevrons) indicate whether the Gear Set is collapsed or expanded. You can click these to toggle
  between the two states.</li>
  <li>If you prefer more fine-grained Gear registration, each piece of Gear in the Gear Set can be registered on its
  own by clicking its “Register” button.</li>
  <li>The Description that the user gave to the Gear Set.</li>
  <li>The pieces of Gear for the Gear Set. They show in color their Gear type followed by their QView-issued ID plus
  all other information about the piece of Gear.</li>
</ol>
<p>Below we have registered a full Gear Set and a few pieces of Gear from Set 2:</p>
${shot('registering-for-a-tournament-17', 'A fully registered Gear Set and a partly registered one, labelled A to E.')}
<ol type="A">
  <li>We now see the updated state of Gear in each Gear Set.</li>
  <li>We can Unregister a full set by clicking the “Unregister Set” button.</li>
  <li>In Set 2 since we have some but not all pieces of Gear registered, we have the option to “Register Remaining”
  pieces of Gear.</li>
  <li>We see that these pieces of Gear in the set are currently Registered.</li>
  <li>If we want to unregister a piece of Gear then we simply click “Undo” for it.</li>
</ol>
<p>When you are happy with which Gear you’ve registered, there is nothing left to do for Gear registration - that’s
it!</p>

<h2>Registering As a Volunteer</h2>
<p>To register as a volunteer as a Tournament (meaning, you intend to show up to help with the operations of the
Tournament in some capacity), find the Tournament you are looking for from the Home screen and click it to enter its
profile. Then if registration is currently open, click either the “Register” button under the Tournament name or the
“Registration” sidebar item.</p>
<p>Then click the “As Volunteer” button/tab at the top. You should see the following:</p>
${shot('registering-for-a-tournament-18', 'The As Volunteer registration tab.')}
<p>If you don’t see a “As Volunteer” button/tab, this likely means that your Tournament director is not accepting
registrations for volunteers using QView and has disabled this feature.</p>
<p>If you are not logged in as a user, you will need to do so. If you’re not logged in you will see this:</p>
${shot('registering-for-a-tournament-19', 'The volunteer message shown when you aren’t signed in.')}
<p>If you are logged in, click the “Register as Volunteer” button. You will see the following:</p>
${shot('registering-for-a-tournament-20', 'Confirming your volunteer registration.')}
<p>Click “Confirm” to register. You should see the following if the registration was successful:</p>
${shot('registering-for-a-tournament-21', 'A successful volunteer registration, with the option to remove it.')}
<p>You can unregister from volunteering as long as registration is open for the Tournament.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:creating-rosters">Creating Rosters (Preparing to Register Teams)</a></li>
  <li><a href="doc:adding-gear">Adding Gear to Your Account (for Tournament Registration)</a></li>
</ul>
`.trim(),
    },

    // ── Setting Up a Tournament ─────────────────────────────────────────────────
    // The articles in this section and in Tournament Day are members-only (`requiresLogin` in the nav),
    // so their content isn't in this bundle: it's served to logged-in users from
    // backend/help-docs/qview/0.1.0.0/{slug}.html, screenshots included (see services/helpdoc.rs).
    {
      slug: 'applying-to-be-a-tournament-manager',
      title: 'Applying to Be a Tournament Manager (Creator/Owner)',
    },
    {
      slug: 'creating-a-tournament',
      title: 'Creating a Tournament (plus Rooms)',
    },
    {
      slug: 'divisions-and-registration',
      title: 'Setting Up Divisions and Registration',
    },
    {
      slug: 'rounds-and-games',
      title: 'Post-Registration Prep: Rounds and Games',
    },
    {
      slug: 'setting-question-types',
      title: 'Setting Question Types for Rounds',
    },

    // ── Tournament Day ──────────────────────────────────────────────────────────
    {
      slug: 'setting-up-rooms',
      title: 'Setting Up Rooms using QuizMachine + QView',
    },
    {
      slug: 'monitoring-rooms',
      title: 'Monitoring Rooms and Resending Game Data',
    },
    {
      slug: 'computing-stats',
      title: 'Computing Stats: Stats Groups, Sneakernet Importing and Exporting Results',
    },
  ],
}
