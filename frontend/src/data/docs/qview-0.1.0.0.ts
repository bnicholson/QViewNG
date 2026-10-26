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
      children: [
        { title: 'Becoming a Tournament Manager', slug: 'applying-to-be-a-tournament-manager' },
        { title: 'Creating a Tournament and Its Rooms', slug: 'creating-a-tournament' },
        { title: 'Divisions and Registration', slug: 'divisions-and-registration' },
        { title: 'Scheduling Rounds and Games', slug: 'rounds-and-games' },
      ],
    },
    {
      title: 'Tournament Day',
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
    {
      slug: 'applying-to-be-a-tournament-manager',
      title: 'Applying to Be a Tournament Manager (Creator/Owner)',
      contentHtml: `
<p><strong>Tournament Managers</strong> are QView users that most notably have the ability to create new Tournaments
and who also can manage and act as full owners of the Tournaments they create.</p>
<p>To apply to become a Tournament Manager, you first need to have a QView account. If you don’t have a QView
account yet, you can <a href="doc:creating-an-account">create an account here</a>.</p>
<p>If you have a QView account, log in on the login page.</p>
<p>Once logged in, you will be taken to your user’s profile.</p>
<p>In your profile, in the sidebar menu click “Managed Tournaments”. If you are not currently a Tournament Manager,
you will see the following:</p>
${shot('applying-to-be-a-tournament-manager-1', 'The Tournament Manager application in Managed Tournaments.')}
<p>In this case, to apply to be a Tournament Manager, share a little bit about yourself, if you are a district or
field director, what Tournaments you direct or want to direct and where (geographically) - all of these are good
information to share with the QView website administrators.</p>
<p>With this information added, click the button to submit it for review.</p>
<p>Once approved, when you navigate to this section of your profile you will see a table of Tournaments (or lack
thereof initially) that you are the owner/creator/manager of.</p>
${shot('applying-to-be-a-tournament-manager-2', 'Managed Tournaments once you are approved.')}

<h2>See also</h2>
<ul>
  <li><a href="doc:creating-a-tournament">Creating a Tournament (plus Rooms)</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'creating-a-tournament',
      title: 'Creating a Tournament (plus Rooms)',
      contentHtml: `
<p>The most fundamental data in QView is the Tournament. Nearly everything in QView has to do with some Tournament in
some way, shape or form.</p>
<p>To create Tournaments your user must first be approved for Tournament creation capabilities. See more on
<a href="doc:applying-to-be-a-tournament-manager">becoming approved to be a Tournament Manager here</a>.</p>

<h2>Creating the Tournament</h2>
<p>To start, Tournament creation can be initiated from two locations:</p>
<ol>
  <li>From your user profile, under <strong>Managed Tournaments</strong>, click <strong>Create Tournament</strong>:
${shot('creating-a-tournament-1', 'Create Tournament in your profile’s Managed Tournaments.')}
  </li>
  <li>From the Home page, click <strong>Create a New Tournament</strong> (if you don’t see this option, you’re either
  not logged in or you don’t have Tournament Manager capabilities):
${shot('creating-a-tournament-2', 'Create a New Tournament on the Home page.')}
  </li>
</ol>
<p>Both locations will open the <strong>Create / Edit Tournament</strong> form. Fill out the form, then click
<strong>Save</strong>.</p>
${shot('creating-a-tournament-3', 'The Create / Edit Tournament form.')}
<p>Most of the fields are self-explanatory. The ones worth a second look:</p>
<ul>
  <li><strong>Name</strong> must be unique across QView. QuizMachine also matches the Tournament by name, so avoid
  renaming it once Rooms are connected.</li>
  <li><strong>Start Date</strong> and <strong>End Date</strong> are required, and the start can’t be after the
  end.</li>
  <li><strong>Default Round Duration (minutes)</strong> (30 by default) is used when you create a schedule or edit
  Round start times in bulk.</li>
  <li><strong>Scheduling Mode</strong> chooses <strong>Normal</strong> or <strong>Advanced</strong> scheduling. You
  can switch it later from the Schedule page (see <a href="doc:rounds-and-games">Post-Registration Prep: Rounds and
  Games</a>).</li>
  <li><strong>Registration Types</strong> turns the Team, Gear and Volunteer registration tabs on or off. Untick all
  three to turn registration off completely. <strong>Registration Open Date</strong> and <strong>Registration Close
  Date</strong> set the Registration Window (see <a href="doc:divisions-and-registration">Setting Up Divisions and
  Registration</a>).</li>
  <li><strong>Short Information</strong> is the one line people see when searching for Tournaments, and in
  QuizMachine when a Room picks its Tournament.</li>
</ul>
<p>The <strong>Visibility</strong> dropdown is used to control whether the Tournament can be seen publicly - by
visitors and everyone else. New Tournaments start as <strong>Private</strong>. If it is set to “Private” only you (and
the Tournament’s admins) can view the Tournament, from your profile’s “Managed Tournaments” section. For private
Tournaments that you manage, you can click this button on the Home page to navigate there when logged in as a
Tournament Manager:</p>
${shot('creating-a-tournament-4', 'View Tournaments that I Manage on the Home page.')}
<p class="doc-warning">QuizMachine can only see <strong>Public</strong> Tournaments. Make the Tournament public before
tournament day, or the Rooms won’t be able to find it.</p>

<p>At the bottom of the form is what is called a Markdown editor. It is initialized with template text in the
Markdown format. You can edit its contents on the left and view the changes on the right side of the editor.</p>
<p>For help with writing Markdown, QView includes a <strong>Markdown syntax cheatsheet</strong>. To access it, just
click the word “Markdown” and the cheatsheet will open in a new tab.</p>
${shot('creating-a-tournament-5', 'The Markdown editor, with the cheatsheet link.')}
<p>When you click <strong>Save</strong>, you’ll be asked to confirm the information. Click
<strong>Confirm</strong> to create the Tournament.</p>
<p>You will then be directed to the Tournament’s Overview page, which looks something like this:</p>
${shot('creating-a-tournament-6', 'A new Tournament’s Overview page.')}
<p>The title is shown on its own line with an <strong>Edit</strong> button, which only you and the Tournament Admins
you add can see. Use it whenever you need to change any of the details above.</p>
<p>It then shows a few data fields that QView tracks uniquely and for different purposes. Finally, the Markdown
portion which shares all other relevant information is included in this Overview page.</p>

<h3>What is the Pairing Code?</h3>
<p>QView gives every new Tournament a 6-digit <strong>Pairing Code</strong>. Each Room’s QuizMachine needs it to
connect to the Tournament (see <a href="doc:setting-up-rooms">Setting Up Rooms using QuizMachine + QView</a>). It’s
hidden as <code>########</code> on the Overview page; click the eye icon to show it. Only Tournament Managers and
admins can see it.</p>
<p>You can change it in the Edit form. Changing it doesn’t disconnect Rooms that are already paired, but every Room
that pairs afterwards needs the new code. Only change it if the code has leaked.</p>

<h2>Adding Buildings (optional)</h2>
<p>If your Tournament is spread across more than one building, Buildings let you group your Rooms so people can find
them. Once your Rooms are assigned to Buildings, anyone viewing the Tournament’s <strong>Schedule</strong> can filter
it by Building: choose <strong>Room</strong> under <strong>Filter By</strong>, then pick a <strong>Building</strong> to
narrow the Room list to just that Building’s Rooms. You can skip this for a single-building Tournament.</p>
<ol>
  <li>In the Tournament sidebar, click <strong>Buildings</strong>.</li>
  <li>Click <strong>Create Building</strong>.</li>
  <li>Enter the <strong>Building Name</strong> and any <strong>Notes</strong> (for example, where to park or which
  entrance to use).</li>
  <li>Click <strong>Save</strong> and confirm.</li>
</ol>
<figure>
  <img src="/docs/qview/creating-a-tournament-7.webp" alt="Creating a Building from the Buildings page." loading="lazy" />
  <figcaption>Creating a Building from the Buildings page.</figcaption>
</figure>
<p>Click a Building’s name to open its page. Its <strong>Rooms</strong> tab lists the Rooms in that Building, and
Rooms you create from there are put in that Building automatically.</p>

<h2>Adding Rooms for the Tournament</h2>
<p>Rooms don’t have to be added to your Tournament until just before creating Games (commonly known as 20-question
quiz rounds), however, Rooms are associated directly with Tournaments and can be used across all Divisions of the
Tournament.</p>
<p>For this reason, it’s worth creating your Rooms now, to see how large your Tournament can scale to with addition
of more and more Teams when they register.</p>
<p>First, in your new Tournament, in the sidebar click <strong>Rooms</strong> to navigate to the Rooms list.</p>
<p>Then click <strong>Create Room</strong>:</p>
${shot('creating-a-tournament-8', 'Create Room on the Tournament’s Rooms page.')}
${shot('creating-a-tournament-9', 'The Create Room form.')}
<p>Provide the name of the Room and perhaps some comments about it: whether it’s in a certain building, in a certain
hard-to-find location in a building, or what its identifiable characteristics are (like “room with accordion
divider”, “sanctuary” or “first room on the right when entering building’s main entrance”). The form also has:</p>
<ul>
  <li><strong>Building</strong>: the Building the Room is in, if you added Buildings. Leave it as
  <strong>None</strong> otherwise. Assigning a Building is optional, but it lets people filter the Tournament’s
  Schedule by Building (see <strong>Adding Buildings</strong> above).</li>
  <li><strong>Supports 3-team games</strong>: tick this if the Room has the seats and Gear for three Teams.</li>
</ul>
<p>You can use any Room-naming scheme that you prefer. Many choose to label Rooms with letters (‘A’, ‘B’, ‘C’, etc.).
You can use numbers, although you may prefer to keep numbering for Rounds only, to keep a clear separation of what is
what (letters = Rooms, numbers = Rounds). If Rooms have their own names already (such as can be found on a college
campus), then you may wish to use that (i.e. “Benson 203” could be “BEN203” - or you could just use “Benson 203”) -
or you could name the Rooms something fun! Just make sure everyone knows where to go to get to where they need to be
when they need to be there.</p>
<p class="doc-note">QuizMachine matches each Room by its name, so settle on Room names before tournament day. If you
rename a Room after its QuizMachine has connected, reconnect that Room.</p>
<p>You can always come back and edit these later, including assigning a Room you’ve already created to a Building.
Once a Room exists, its Edit form also lets you assign its <strong>Quizmaster</strong> and <strong>Content
Judge</strong>.</p>
<p>Click <strong>Save</strong> and confirm to create the Room. For reference, a list of Rooms could look like this for a
Tournament:</p>
${shot('creating-a-tournament-10', 'A Tournament’s list of Rooms.')}
<p>With that, you have a Tournament and Rooms. Next, you will need to
<a href="doc:divisions-and-registration">set up <strong>Divisions</strong> and prepare
<strong>Registration</strong></a>.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:applying-to-be-a-tournament-manager">Applying to Be a Tournament Manager (Creator/Owner)</a></li>
  <li><a href="doc:divisions-and-registration">Setting Up Divisions and Registration</a></li>
  <li><a href="doc:setting-up-rooms">Setting Up Rooms using QuizMachine + QView</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'divisions-and-registration',
      title: 'Setting Up Divisions and Registration',
      contentHtml: `
<p>After <a href="doc:applying-to-be-a-tournament-manager">becoming approved to create and manage your own
Tournaments</a> and after <a href="doc:creating-a-tournament">creating a Tournament and its Rooms</a>, you are ready
to set up your Divisions and Registration.</p>
<p>Divisions can fundamentally be considered groups of Teams that compete with each other. In this way, Divisions
are uniquely related to the Registration process since Teams are registered by their coaches/leaders and to register
a Team must be designated to compete under a specific Division.</p>

<h2>Creating Divisions</h2>
<p>Let’s create a Division. In the Tournament profile, click the “Divisions” sidebar item.</p>
<p>Then click “Create Division”.</p>
${shot('divisions-and-registration-1', 'Create Division on the Tournament’s Divisions page.')}
${shot('divisions-and-registration-2', 'The Create Division form.')}
<p>Give your Division a name, select its initial “Visibility” status, and provide a short description of the
Division. Note that the name and Short Info will be visible to others when the Visibility is public (when the
Tournament is also public).</p>
<p>Once entered, click “Save” and confirm. A list of Divisions may look something like this:</p>
${shot('divisions-and-registration-3', 'A Tournament’s list of Divisions.')}
<p>You can click a Division’s name to navigate to its Division profile page. This is also where you can go to edit
the Divisions general information:</p>
${shot('divisions-and-registration-4', 'A Division’s profile page.')}

<h2>Preparing Registration</h2>
<p>QView can organize your registration for <strong>Teams</strong> and for <strong>Gear (equipment)</strong>.</p>
<p>To start, navigate to the Tournament’s Overview page.</p>
${shot('divisions-and-registration-5', 'The Registration Window on the Tournament’s Overview page.')}
<p>Note the “Registration Window” information shown below. It gives the date range for when the Tournament is/will
be/had been open for registering Teams and Gear. In this case, the date is 9/8/2026, so it falls in this range.
Because of this, its status is “OPEN”, we are given a “Register” button right under the Tournament name and a
“Registration” item in the sidebar. The button and sidebar items show/hide based on today’s current date given the
Registration Window’s date range.</p>
<p>To control whether registration is currently OPEN or CLOSED, modify the Registration Window dates by clicking the
“Edit” button, making the change, and then saving the change.</p>
<p>It’s important to understand the registration process if you are going to have coaches and other program leaders
register Teams and Gear for your Tournament. For more on this topic please see the knowledge article on
<strong><a href="doc:registering-for-a-tournament">Registering for a Tournament: Teams and Gear</a>.</strong></p>
<p>Once you have reviewed that article, the next steps to take after your Teams and Gear have been registered is to
<strong><a href="doc:rounds-and-games">Prepare your Rounds and Games</a>.</strong></p>

<h2>See also</h2>
<ul>
  <li><a href="doc:registering-for-a-tournament">Registering for a Tournament: Teams, Gear and Volunteers</a></li>
  <li><a href="doc:rounds-and-games">Post-Registration Prep: Rounds and Games</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'rounds-and-games',
      title: 'Post-Registration Prep: Rounds and Games',
      contentHtml: `
<p>At this point, you should have a Tournament that you own and manage and have Divisions, Rooms (Buildings
optional) and Teams fully entered.</p>
<p>If you haven’t added your Rooms (and/or Buildings) yet, you will need to do so now before proceeding. Take a look
at the section on adding Rooms to a Tournament in <a href="doc:creating-a-tournament">this article</a>. If your
Tournament uses more than one building, assign each Room to its Building too, so people can filter the Schedule by
Building.</p>
<p>In QView, the Scheduling module has <strong>2 modes</strong>: “Normal” and “Advanced”.</p>
<p><strong>Normal Mode</strong> deals only with Rounds and Games.</p>
<p><strong>Advanced Mode</strong> includes what is included in Normal Mode and adds to it Sessions, Pools, Brackets,
PoolGroups and BracketGroups.</p>
<p>Both Scheduling modes can be used to achieve the same goal. Advanced Mode is more for organization of large
schedules in an effort to make the sheer amount of Games and Rounds make more sense in their given planned
contexts.</p>
<p>With this being said, let’s cover the basics - <strong>Normal Mode:</strong></p>

<h2>Normal Mode</h2>
<p>Let’s start with terms.</p>
<p>A <strong>Game</strong> in QView is a standard 20-question quiz set asked to 2 or 3 Teams of Quizzers by a
quizmaster (in Nazarene-style teen Bible quizzing).</p>
<p>A <strong>Round</strong> is a fundamental, <em>time-based grouping</em> of Games. Each Round will therefore have
a different time than the other Rounds of its Division.</p>
<p>An important note: <strong>Each Division has its own set of Rounds</strong>, from a data perspective. You may be
used to seeing a schedule table that looks something like this:</p>
${shot('rounds-and-games-1', 'A typical schedule table of Rounds and Rooms.')}
<p>..except with more Rooms and with <em>multiple Divisions</em>. This last part deserves clarity concerning QView:
<strong>Rounds don’t span multiple Divisions in QView</strong>. This is a deliberate design choice so that
scheduling is encapsulated to the scope of the Division. At the same time, it is a design decision to NOT require
the Division to (otherwise) be tied to a Round such that the Round’s Start Time is shared across Divisions, creating
inflexibility in scheduling across all Divisions of the Tournament.</p>
<p>Since the Round has to have a single Start Time, it was decided to allow Divisions to have their own Rounds and,
therefore, their own Start Times to allow for more flexible scheduling in-Division.</p>
<p>In <em>Normal Mode</em>, all that is needed for scheduling is Rounds and Games. Games get their Start Times from
the Round they are assigned to. Without Rounds, Games have no scheduling context.</p>
<p>One of the most popular scheduling styles is <strong>Round-Robin</strong> matchups. In the examples here we will
be using 2-team Game matchups since they are most common. There are some added complexities for 3-team Game
matchups which will be addressed later.</p>
<p>Let’s get started! Go to your Tournament and click Scheduling. Then click “Create / Edit” at the top:</p>
${shot('rounds-and-games-2', 'Schedule in the Tournament sidebar.')}
${shot('rounds-and-games-3', 'The Create / Edit button on the Schedule page.')}
${shot('rounds-and-games-4', 'The schedule editor’s mode, Division tabs and buttons, labelled A to E.')}
${shot('rounds-and-games-5', 'The schedule editor’s filter, Rounds and Games, labelled F to H.')}
<p>Here you will see the following:</p>
<ol type="A">
  <li><strong>Mode</strong>: Controls toggle between “Normal” and “Advanced” mode. This is a setting at the
  Tournament level which is updated whenever it is changed by any user with access to it. Data for both modes is the
  same underneath; you can use either mode interchangeably.</li>
  <li><strong>Divisions in a tabbed row</strong>: Select which Division to create/edit the schedule of.</li>
  <li><strong>‘Create Schedule’ button</strong>: Use this to create Rounds and Games (or more in Advanced Mode) in a
  Round-Robin or Tournament Bracket configuration.</li>
  <li><strong>‘Add Round’ button</strong>: Add a single new Round to the current list of Rounds.</li>
  <li><strong>‘Edit Start Times’ button</strong>: Edit the Start times of all Rounds in bulk with a given Round
  duration (default = 30 minutes). (The default Round duration is set at the Tournament-level.)</li>
  <li><strong>Filter</strong>: Select/Deselect which Games are shown in the schedule. This does not delete the items,
  it only hides them.
    <ul>
      <li>For example, clicking ‘Room 9’ in the screenshot above will make Games that happen in Room 9 disappear.
      Clicking it again brings them back.</li>
      <li>Clicking the <strong>‘eye’ icon</strong> will cause all Rooms (or Teams) to be hidden, effectively hiding
      all Games. Click it again to bring them back. When some are hidden, clicking the ‘eye’ icon makes all items
      visible again.</li>
      <li>When the <strong>‘Show Round When Empty’ checkbox</strong> is selected, the Round will remain visible even
      when there are no Games currently showing in its Round. When deselected, the Round also disappears when all of
      its Games are hidden.</li>
      <li>Clicking the <strong>‘Reset/Show All’ button</strong> resets the filter so that all Games of the Division
      are visible. Use this if you ever feel like you may be missing something.</li>
      <li>Note: The ‘Teams’ filter works a little differently than the other filter options. Since there are
      two/three Teams per Game, hiding one Team normally won’t hide any Games. Hiding a 2nd Team will hide all Games
      where those two Teams compete without a third Team, if any exist.</li>
    </ul>
  </li>
  <li><strong>Round</strong>: This is the time-context for the Game. In it you can:
    <ul>
      <li><strong>‘Add a Game’</strong>: Add a new Game to that specific Round.</li>
      <li><strong>‘Edit Round’</strong>: Allows you to edit the Round’s details.</li>
      <li><strong>‘Delete Round’</strong>: Allows for deleting the Round from the Division.</li>
    </ul>
  </li>
  <li><strong>Game</strong>: This is the 2- or 3-team matchup of 20 questions. Click the Game to enter the editor for
  the Game. In the editor you also have the options to:
    <ul>
      <li><strong>‘Delete’</strong>: Delete the Game.</li>
      <li><strong>‘View Game’</strong>: Navigate to the Game’s profile.</li>
      <li><strong>‘Save’</strong>: Save the edits made to the Game.</li>
    </ul>
  </li>
</ol>

<h2>Create Schedule button</h2>
<p>It is recommended that when creating Rounds and Games for the first time for a Division to use the “Create
Schedule” button.</p>
<p>Alternatively, you can delete all Rounds (which deletes its associated Games with it) to clear the Division of
all Scheduling and then use the “Create Schedule” feature to recreate the Schedule from scratch if
needed/desired.</p>
<p>Clicking the button will show you the following:</p>
${shot('rounds-and-games-6', 'The Create Schedule dialog.')}
<p>The top dropdown selector gives the options to schedule using “Round Robin” or “Tournament Brackets”. At this time
only Round Robin scheduling is supported by this feature; Tournament Brackets will be supported in a future release
of QView.</p>
<p>Making modifications to each field should be fairly intuitive. This dialog helps you by calculating the result of
what would be scheduled if you were to use the current settings.</p>
<p>Once everything looks correct, click “Create Schedule” to create the Rounds and Games that you described using the
fields of the dialog.</p>

<h2>Edit Start Times button</h2>
<p>Use this button to adjust the Start Times of the Division’s existing Rounds in bulk. When you click the button,
you are presented with the following options:</p>
${shot('rounds-and-games-7', 'The Edit Start Times dialog.')}
<p>Please take some time to read the notice in this dialog box. It is a simple interface: Set the date and time that
the first Round is supposed to start, then define how much time should be in between the start of each Round. When
you click ‘Apply’ it will adjust the start times of all Rounds by that offset. For instance, Round 1 = 7am, Round 2 =
7:30am, Round 3 = 8am, Round 4 = 8:30am, etc. (This modifies existing Rounds of the Division only.)</p>

<h2>Once Done With Scheduling:</h2>
<p>Please keep in mind, Scheduling may need to be updated if you ever add new Teams to the Division. This is a great
reason to have hard registration cut-off dates - so that you have the appropriate amount of time for scheduling each
Division of your Tournament.</p>
<p>Once scheduling is complete, you are ready to use QView with QuizMachine on Tournament day! Check out the
following articles on how to set up QuizMachine in quiz rooms and connect QuizMachine with QView.</p>
<ul>
  <li><a href="doc:setting-up-rooms">Setting Up Rooms using QuizMachine + QView</a></li>
  <li><a href="doc:monitoring-rooms">Monitoring Rooms and Resending Game Data</a></li>
  <li><a href="doc:computing-stats">Computing Stats: Stats Groups, Sneakernet Importing and Exporting Results</a></li>
</ul>
`.trim(),
    },

    // ── Tournament Day ──────────────────────────────────────────────────────────
    {
      slug: 'setting-up-rooms',
      title: 'Setting Up Rooms using QuizMachine + QView',
      contentHtml: `
<p>You’ve set up your Tournament, Registration has closed and it’s the (first) morning of the Tournament. You need to
set up your quiz rooms so that when the first Round starts, everything goes smoothly.</p>
<p>Ahead of time, if you are having your various Teams bring their Gear for use at the Tournament (which has been
registered for use at the Tournament), then they will need to arrive early enough for this setup. Typically 1 hour
before the first Round begins works for a regular sized Tournament. With larger Tournaments you may need more time and
perhaps some setup the day before.</p>
<p>If you are using QView then you are also using QuizMachine on computers (usually laptops), 1 per Room (plus other
Gear; this article assumes you know how to set that Gear up as needed).</p>

<h2>Before you start</h2>
<ul>
  <li><strong>QuizMachine 6.3.0-e76 or newer</strong> on every Room’s computer. Older versions of QuizMachine will not
  work with QView. For the latest install of QuizMachine please see
  <a href="https://www.quizstuff.com/downloads.php" target="_blank" rel="noopener noreferrer">QuizStuff.com</a>.</li>
  <li><strong>An internet connection</strong> in each Room. (If a Room can’t get online, see
  <a href="doc:computing-stats">importing its Games afterwards</a>.)</li>
  <li><strong>The Tournament set to Public.</strong> QuizMachine can’t see Private Tournaments.</li>
  <li><strong>The Tournament’s Pairing Code.</strong> It’s on the Tournament’s Overview page (click the eye icon to
  show it). See <a href="doc:creating-a-tournament">Creating a Tournament</a>.</li>
  <li><strong>Rooms and a schedule</strong> in QView, so each Room has Games to load (see
  <a href="doc:rounds-and-games">Post-Registration Prep: Rounds and Games</a>).</li>
</ul>

<h2>Pointing QuizMachine at QView</h2>
<p>Once installed, start the QuizMachine desktop application (client). We will need to be using the QView
configuration along with the address of the QView website for communicating data to it.</p>
<ol>
  <li>Click <strong>More</strong> or press the <strong>M</strong> key to pull up the <strong>More Menu</strong>. Then click
  <strong>Setup</strong>.
<figure>
  <img src="/docs/qview/setting-up-rooms-1.webp" alt="Setup in QuizMachine’s More Menu." loading="lazy" />
  <figcaption>Setup in QuizMachine’s More Menu.</figcaption>
</figure>
  </li>
  <li>At the top of the Setup window, click the second tab, <strong>Server/Sneakernet</strong>.
<figure>
  <img src="/docs/qview/setting-up-rooms-2.webp" alt="QuizMachine’s Server/Sneakernet tab set up for QView." loading="lazy" />
  <figcaption>QuizMachine’s Server/Sneakernet tab set up for QView.</figcaption>
</figure>
  </li>
  <li>Set the Server to <strong>QView</strong> and the IP address to <strong>qview.teenbiblequiz.com</strong>.</li>
  <li>Leave <strong>Tournament key</strong> and <strong>Physical Bldg/Room</strong> blank. QView doesn’t use them: you’ll
  enter the Tournament’s Pairing Code and pick the Room in a later step instead.</li>
  <li>Click <strong>OK</strong> to apply the changes and close the Setup window. (You do not need to click
  <strong>Apply</strong> before clicking <strong>OK</strong>.)</li>
</ol>

<h2>Choosing the Tournament and Room</h2>
<ol>
  <li>Click <strong>More</strong> or press <strong>M</strong> again. Now click <strong>Start a Tournament</strong>.
<figure>
  <img src="/docs/qview/setting-up-rooms-3.webp" alt="Start a Tournament in QuizMachine’s More Menu." loading="lazy" />
  <figcaption>Start a Tournament in QuizMachine’s More Menu.</figcaption>
</figure>
  <p>QuizMachine will get the available Tournaments from QView. If this is successful you will very briefly see a
  loading pop-up window, then the Tournament picker:</p>
<figure>
  <img src="/docs/qview/setting-up-rooms-4.webp" alt="QuizMachine’s Tournament picker." loading="lazy" />
  <figcaption>QuizMachine’s Tournament picker.</figcaption>
</figure>
  </li>
  <li>In the first column, select your Tournament. The list shows Public Tournaments from about 1 month in the past
  to 1 month in the future. The second column shows the selected Tournament’s Short Information, which helps you
  check it’s the right one.</li>
  <li>In the third column, select the Room that this QuizMachine computer is for.</li>
  <li>Enter the Tournament’s Pairing Code in the <strong>Pairing Code</strong> field. This is a security measure: only
  people with the Pairing Code can “pair” a QuizMachine client with that Tournament in QView. It also protects you
  from connecting to the wrong Tournament, since that Tournament’s code won’t match.</li>
  <li>Click <strong>Join</strong>.</li>
</ol>

<h2>Checking the first Game’s lineups</h2>
<p>On success you will see the Lineups window appear with the first Game of that Room populated:</p>
<figure>
  <img src="/docs/qview/setting-up-rooms-5.webp" alt="The Lineups window filled in from QView." loading="lazy" />
  <figcaption>The Lineups window filled in from QView.</figcaption>
</figure>
<p>QView fills in the seats for you, but feel free to reorder each Team’s Quizzers by moving them to different seats
as needed. You may also need to change their Captain/Co-captain designations. When ready, click <strong>OK</strong>
to save the changes, close the Lineups window and load the Game.</p>
<p>You are now ready to use this Room for the first Game of the Room!</p>

<h2>How do you know it’s working?</h2>
<p>Back in QView, open the Tournament and click <strong>Server Monitor</strong> in the sidebar. Every Room that has
connected appears there with a recent <strong>Check In</strong> time. Once a Game starts, it also shows on
<strong>LIVE Stats</strong> under <strong>Games in Progress</strong>. See
<a href="doc:monitoring-rooms">Monitoring Rooms and Resending Game Data</a>.</p>

<h2>When it doesn’t work</h2>
<p>If the Tournament list doesn’t load, you’ll see an error instead:</p>
<figure>
  <img src="/docs/qview/setting-up-rooms-6.webp" alt="QuizMachine’s error when it can’t reach QView." loading="lazy" />
  <figcaption>QuizMachine’s error when it can’t reach QView.</figcaption>
</figure>
<ul>
  <li><strong>The error above appears.</strong> Either the server settings aren’t right (usually a typo in the
  address), or QView or the internet connection isn’t available. More often than not it’s the first: re-check the
  <strong>Server/Sneakernet</strong> tab. In the example above, the address was typed with a comma instead of a period
  (<code>qview,teenbiblequiz.com</code>), so QuizMachine couldn’t find QView.</li>
  <li><strong>Your Tournament isn’t in the list.</strong> Either its dates aren’t within about a month of today, or
  it is set to <strong>Private</strong> (the default). Fix either one in the Tournament’s Edit form, from its Overview
  page.</li>
  <li><strong>Your Room isn’t in the list.</strong> Check that the Room exists on the Tournament’s
  <strong>Rooms</strong> page.</li>
  <li><strong>Join fails.</strong> Check the Pairing Code on the Tournament’s Overview page. If the code was changed
  recently, use the new one.</li>
  <li><strong>The Lineups window is empty, or shows the wrong Game.</strong> Check the Room’s Games on the
  Tournament’s <strong>Schedule</strong> page. Games are loaded in order of their Round’s start time.</li>
</ul>
<p class="doc-note">When a QuizMachine client is loaded as a certain Room of a Tournament, QuizMachine sees the Games as
completely new with no history. QView, however, may already have some history for those Games if a different
QuizMachine was connected as that Room and was used earlier.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:monitoring-rooms">Monitoring Rooms and Resending Game Data</a></li>
  <li><a href="doc:computing-stats">Computing Stats: Stats Groups, Sneakernet Importing and Exporting Results</a></li>
  <li><a href="doc:quizmachine/entering-lineups">Entering Lineups (QuizMachine)</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'monitoring-rooms',
      title: 'Monitoring Rooms and Resending Game Data',
      contentHtml: `
<p>Once your Rooms are connected, the <strong>Server Monitor</strong> lets you watch every Room from one screen and
catch problems before they reach the stats: a Room that has stopped checking in, a Game whose data didn’t all arrive,
or a QuizMachine whose send queue is stuck. This article is for the Tournament’s owner and admins on tournament
day.</p>
<p><strong>Before you start:</strong> you need to be the Tournament’s owner or one of its admins to see Server Monitor.
Resending and clearing also need permission to update Games, which Tournament Managers and Tournament Admins
have.</p>

<h2>Opening the Server Monitor</h2>
<ol>
  <li>Open the Tournament and click <strong>Server Monitor</strong> in the sidebar.</li>
</ol>
<figure>
  <img src="/docs/qview/monitoring-rooms-1.webp" alt="The Server Monitor with one Room needing attention." loading="lazy" />
  <figcaption>The Server Monitor with one Room needing attention.</figcaption>
</figure>
<p>Each Room appears once its QuizMachine has checked in. The page updates by itself: a green <strong>Live</strong>
means changes appear as soon as QView receives them. <strong>Reconnecting…</strong> or <strong>Refreshing every
30s</strong> means the live connection dropped, and the page is still updating, just more slowly.</p>

<h2>What does each column mean?</h2>
<ul>
  <li><strong>Check In</strong>: when the Room last checked in with QView. A healthy Room checks in regularly.</li>
  <li><strong>Room</strong>, <strong>Round</strong>: the Room and the Round it’s on.</li>
  <li><strong>Question</strong>: the question the Room says it’s on. If QView hasn’t received all of the Room’s
  events yet, a second line shows how far QView has actually scored (for example, “Scored through Q7”).</li>
  <li><strong>QMVersion</strong>: the Room’s QuizMachine version.</li>
  <li><strong>Pending</strong>: how many events are waiting in the Room’s QuizMachine to be sent. It should stay at or
  near 0.</li>
  <li><strong>Quarantine</strong>: events QView received but didn’t apply (see below).</li>
  <li><strong>Status-Error</strong>: what’s wrong, if anything.</li>
  <li><strong>Resend</strong>: asks the Room to send a Game’s events again.</li>
</ul>

<h2>When is a row red?</h2>
<p>A row turns red when its Status-Error column shows one of these:</p>
<ul>
  <li><strong>Room not communicating?? Late??</strong> The Game is in progress but the Room hasn’t checked in for
  more than 2 minutes. Go to the Room: the computer may have lost its internet connection, gone to sleep, or
  QuizMachine may have been closed.</li>
  <li><strong>Data Incomplete. Resend Advised.</strong> QView is missing some of the Game’s events (a question or an
  event in the middle is missing). Use <strong>Resend</strong>.</li>
</ul>
<p>A Room can have more than one row. Besides its current Game, it gets an extra row for any earlier Game with
missing data, so you can resend that too. The extra row disappears once that Game’s data is complete.</p>

<h2>Resending a Game’s events</h2>
<ol>
  <li>Find the red row with <strong>Data Incomplete. Resend Advised.</strong></li>
  <li>Click <strong>Resend</strong> on that row.</li>
  <li>Wait for the Room’s next check-in. The note under the button tracks progress.</li>
</ol>
<figure>
  <img src="/docs/qview/monitoring-rooms-2.webp" alt="A resend waiting for the Room’s next check-in." loading="lazy" />
  <figcaption>A resend waiting for the Room’s next check-in.</figcaption>
</figure>
<p>QView can’t reach into the Room directly. Instead, it hands the request to QuizMachine the next time the Room checks
in, and QuizMachine sends the whole Game again. The note tells you where it’s at:</p>
<ul>
  <li><strong>Requested — awaiting next ping</strong>: the Room hasn’t checked in since you clicked.</li>
  <li><strong>Resend request received by room.</strong>: the Room has the request and is resending.</li>
  <li><strong>Room failed to resend game events. Please issue another resend request.</strong>: it’s been more than
  30 seconds and the data is still incomplete. Click <strong>Resend</strong> again.</li>
</ul>
<p><strong>How can you tell it worked?</strong> The note disappears, the row is no longer red, and an extra row for an
earlier Game drops off the monitor. The <strong>Resend</strong> button greys out once the data is complete.</p>
<p class="doc-note">If <strong>Resend</strong> keeps failing, check that the Room is still checking in (look at
<strong>Check In</strong>). A Room that isn’t online can’t pick up the request. If it can’t get online, export the
Game from QuizMachine and import it instead (see <a href="doc:computing-stats">Importing Games from an offline
Room</a>).</p>

<h2>What if a Room’s Pending number is stuck?</h2>
<p>Sometimes one event gets refused every time QuizMachine sends it, and everything queued behind it waits. When a
Room’s <strong>Pending</strong> hasn’t gone down for 90 seconds, QView shows a red <strong>Clear Pending +
Resend?</strong> button in that cell. (Users without permission see <strong>Stuck</strong> instead.)</p>
<ol>
  <li>Click <strong>Clear Pending + Resend?</strong>.</li>
  <li>Read the dialog, then click <strong>Clear Pending + Resend</strong>.</li>
  <li>Leave the Room’s QuizMachine running and connected, and watch the chip in the Pending cell.</li>
</ol>
<figure>
  <img src="/docs/qview/monitoring-rooms-3.webp" alt="The Clear Pending + Resend dialog." loading="lazy" />
  <figcaption>The Clear Pending + Resend dialog.</figcaption>
</figure>
<p>Here’s what happens:</p>
<ol>
  <li><strong>Clearing…</strong> For up to 5 minutes, QView tells the Room each event arrived but doesn’t apply it,
  so the Room’s queue empties. Every one of those events is kept in the Room’s Quarantine. Nothing is deleted.</li>
  <li><strong>Resending…</strong> Once the Room reports nothing pending, QView asks it to resend every affected Game in
  full, once a minute for up to 10 minutes.</li>
  <li>It finishes as one of:
    <ul>
      <li><strong>Recovered</strong> (green): every set-aside event came back and was stored. You’re done.</li>
      <li><strong>Resend not confirmed</strong> (red): the Room didn’t resend everything within 10 minutes. Check the
      Room is online, then click <strong>Resend</strong> on any row still marked Data Incomplete.</li>
      <li><strong>Expired</strong>: the 5 minutes ran out with nothing set aside, so nothing changed.</li>
      <li><strong>Cancelled</strong>: you stopped it before anything was set aside.</li>
    </ul>
  </li>
</ol>
<p>The final chip stays on the row for 15 minutes.</p>
<p class="doc-warning">While a Room is being cleared and resent, its scores in Games in Progress and Server Stats may
be behind. Wait for <strong>Recovered</strong> before you rely on that Room’s stats.</p>
<p>You can stop it with <strong>Cancel</strong> next to the <strong>Clearing…</strong> chip. Anything already set
aside will still be asked for. Once it’s on <strong>Resending…</strong>, it can’t be cancelled.</p>

<h2>What is Quarantine?</h2>
<p>Quarantine holds events QView told the Room it received, so the Room could keep sending, but didn’t apply. Click the
shield icon in a Room’s <strong>Quarantine</strong> column to see them. An amber shield with a number means there are
some; a grey shield means none.</p>
<figure>
  <img src="/docs/qview/monitoring-rooms-4.webp" alt="A Room with one quarantined event." loading="lazy" />
  <figcaption>A Room with one quarantined event.</figcaption>
</figure>
<figure>
  <img src="/docs/qview/monitoring-rooms-5.webp" alt="A Room’s quarantined events." loading="lazy" />
  <figcaption>A Room’s quarantined events.</figcaption>
</figure>
<ul>
  <li><strong>Set aside while clearing</strong>: held during a Clear Pending + Resend. These come back on their own
  when the Room resends.</li>
  <li><strong>Rejected</strong>: QView couldn’t use the event, and the <strong>Reason</strong> column says why (for
  example, the Game no longer exists in QView). Use the copy icon to copy the event exactly as the Room sent it, if
  you need to pass it to a QView administrator.</li>
</ul>
<p class="doc-note">The Quarantine count is a history: it doesn’t go down when set-aside events are recovered. Use the
row’s Status-Error and Resend columns, not the Quarantine count, to tell whether a Game’s data is complete.</p>

<h2>Watching scores live</h2>
<p>To watch the Games themselves, click <strong>LIVE Stats</strong> in the Tournament sidebar. Its <strong>Games in
Progress</strong> tab shows every Room that has checked in within the last 30 minutes, with each Team’s score, and
anyone can view it. See <a href="doc:computing-stats">Computing Stats</a> for the Team Stats tab.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:setting-up-rooms">Setting Up Rooms using QuizMachine + QView</a></li>
  <li><a href="doc:computing-stats">Computing Stats: Stats Groups, Sneakernet Importing and Exporting Results</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'computing-stats',
      title: 'Computing Stats: Stats Groups, Sneakernet Importing and Exporting Results',
      contentHtml: `
<p>This article is for Tournament owners and admins who want standings for each Division. It covers how QView
groups Games for stats, how to read Team and Individual stats, how to pull in Games from a Room that was offline
(“sneakernet”), and how to export the results.</p>

<p><strong>Before you start:</strong> you need to be the Tournament’s owner or one of its admins. Only those users see
<strong>Server Stats</strong> in the Tournament sidebar.</p>

<h2>What is a Stats Group?</h2>
<p>A Stats Group is the set of Games that stats are computed from. <strong>QView makes one for you</strong> every time
you create a Division, using the Division’s name. You don’t create or fill it yourself: when a Game is added to the
Division (or moved into it), it joins the Division’s Stats Group automatically.</p>
<p class="doc-note">If you’ve used QMServer, this replaces creating a Stats Group/Tournament by hand. In QView, every
Division already has one.</p>

<h2>Where do you find stats?</h2>
<p>There are two places:</p>
<ul>
  <li><strong>Server Stats</strong> (owners and admins): Team and Individual stats for any Division, the
  tie-breaking rules, sneakernet importing and exporting.</li>
  <li><strong>LIVE Stats</strong> (everyone, even if they aren’t signed in): the Division’s Team standings, updating
  live as Games finish. It’s a good page to put on a projector.</li>
</ul>

<h2>Checking that every Game is ready</h2>
<p>Stats only count Games that are finished and have complete data, so check the Games first.</p>
<ol>
  <li>Open the Tournament and click <strong>Server Stats</strong> in the sidebar.</li>
  <li>In <strong>Divisions / Groups</strong>, pick the Division.</li>
  <li>In <strong>Data</strong>, pick <strong>Games Selection</strong> (it’s the default).</li>
</ol>
<figure>
  <img src="/docs/qview/computing-stats-1.webp" alt="The Games Selection view on Server Stats." loading="lazy" />
  <figcaption>The Games Selection view on Server Stats.</figcaption>
</figure>
<p>Each row is a Game. Games already in this Division’s Stats Group are ticked and listed first. The columns that
matter most:</p>
<ul>
  <li><strong>Question</strong>: how far the Game has got.</li>
  <li><strong>Done</strong>: <strong>Yes</strong> once the Game is finished.</li>
  <li><strong>DataOk</strong>: <strong>Yes</strong> when QView has every event it needs to score the Game.</li>
</ul>
<p>If any Games aren’t ready, a banner above the table tells you how many. A <strong>red row</strong> means the Game
reached question 20 but its data is incomplete. Its Information column says <strong>Data Incomplete. Resend
Advised.</strong></p>
<p>To fix a Game with missing data, click the <strong>No</strong> in its DataOk column. That opens the Room Monitor,
where you can ask that Room’s QuizMachine to resend (see <a href="doc:monitoring-rooms">Monitoring Rooms and
Resending Game Data</a>). If the Room never had a connection, import its Games instead (below).</p>
<p class="doc-note">The <strong>Selected</strong> checkboxes only change what you see and what goes into an export.
They don’t add Games to or remove Games from the Stats Group, and they aren’t saved.</p>

<h2>Reading Team Stats</h2>
<ol>
  <li>On <strong>Server Stats</strong>, set <strong>Data</strong> to <strong>Team Stats</strong>.</li>
</ol>
<figure>
  <img src="/docs/qview/computing-stats-2.webp" alt="Team Stats on Server Stats." loading="lazy" />
  <figcaption>Team Stats on Server Stats.</figcaption>
</figure>
<p>You’ll see each Team’s <strong>Place</strong>, <strong># Games</strong>, <strong>Wins</strong>,
<strong>Losses</strong>, <strong>Olympic Points</strong>, <strong>Mod. Olympic Points</strong>, <strong>Total
Points</strong> and <strong>Total Avg Pts</strong>.</p>
<p>A few things worth knowing:</p>
<ul>
  <li><strong>Wins and losses are counted per opponent.</strong> Say three Teams quiz together: the 1st-place Team
  goes 2-0 for that Game, 2nd goes 1-1, and 3rd goes 0-2. In a 2-team Game it’s simply 1-0 and 0-1.</li>
  <li><strong>Olympic points</strong> are 8 and 3 in a 2-team Game, and 10, 5 and 1 in a 3-team Game.
  <strong>Modified Olympic points</strong> add to those based on each Team’s score.</li>
  <li>Overtime questions don’t count toward the Team score used here.</li>
  <li>Only finished Games count. A Game still in progress doesn’t change the standings until it ends.</li>
</ul>

<h3>Why is one Team above another?</h3>
<p>Click the <strong>(i)</strong> in a Team’s <strong>Rank Rules</strong> column. A dialog explains which rule put
the Team above the Team below it (and below the Team above it), with both Teams’ numbers. For example,
“Rule 1 — Team W-L Record: 6-2, 4-4.”</p>

<h3>Changing the tie-breaking rules</h3>
<p>When Teams are level, QView separates them using a list of rules, top first. To change the order:</p>
<ol>
  <li>On <strong>Server Stats</strong> with <strong>Data</strong> set to <strong>Team Stats</strong>, click
  <strong>Tie-breaking Rules</strong>.</li>
  <li>Drag a rule (or use its up/down arrows) to move it. Untick a rule to leave it out.</li>
  <li>Click <strong>Save</strong>.</li>
</ol>
<figure>
  <img src="/docs/qview/computing-stats-3.webp" alt="The Tie-breaking Rules dialog." loading="lazy" />
  <figcaption>The Tie-breaking Rules dialog.</figcaption>
</figure>
<p>The default order is <strong>Team W-L Record</strong>, <strong>Head-to-Head W-L</strong>, <strong>Total
Points</strong>, <strong>Total Avg Pts</strong>, <strong>Olympic Pts</strong>, <strong>Mod Olympic Pts</strong>.
Head-to-Head only looks at the Games those Teams played against each other.</p>
<p>When you save, Team Stats re-sorts straight away, and so does LIVE Stats for everyone watching.</p>
<p class="doc-note">If you untick every rule, level Teams are simply listed in alphabetical order. Places are never
shared between Teams: if two Teams are still level after every rule, alphabetical order decides.</p>

<h2>Reading Individual Stats</h2>
<ol>
  <li>On <strong>Server Stats</strong>, set <strong>Data</strong> to <strong>Individual Stats</strong>.</li>
</ol>
<figure>
  <img src="/docs/qview/computing-stats-4.webp" alt="Individual Stats on Server Stats." loading="lazy" />
  <figcaption>Individual Stats on Server Stats.</figcaption>
</figure>
<ul>
  <li><strong>Score</strong> is the Quizzer’s points <em>without</em> bonus points. Bonus points are shown on their
  own in <strong>Bonus Pts</strong>.</li>
  <li><strong>Avg</strong> is Score divided by # Games.</li>
  <li><strong>Errors</strong> counts wrong answers on toss-up questions. <strong>Bonus Attempts</strong> counts every
  bonus they tried, right or wrong.</li>
  <li>Quizzers are ranked by highest Score, then fewest Errors. Quizzers with the same Score and Errors share a place,
  and the next place is skipped (1, 1, 3).</li>
</ul>
<p class="doc-note">The question-type columns (<strong>Generals</strong>, <strong>Memory</strong>,
<strong>According</strong>, <strong>Context</strong>, <strong>Special</strong>) and <strong>Errs 16+/5+</strong> are
blank for now.</p>
<p class="doc-note">Teams and Quizzers are matched by name. If a name was typed differently in a Room, it shows up as a
separate row, so fix the name in QView and resend or re-import that Game.</p>

<h2>Showing live standings with LIVE Stats</h2>
<ol>
  <li>Open the Tournament and click <strong>LIVE Stats</strong>.</li>
  <li>Click the <strong>Team Stats</strong> tab and pick the <strong>Division</strong>.</li>
</ol>
<figure>
  <img src="/docs/qview/computing-stats-5.webp" alt="Team standings on LIVE Stats." loading="lazy" />
  <figcaption>Team standings on LIVE Stats.</figcaption>
</figure>
<p>The table updates by itself as Games finish. The indicator in the corner tells you how it’s staying up to date:
<strong>Live</strong> is best. <strong>Refreshing every 30s</strong> means the live connection dropped and the
page is checking every 30 seconds instead.</p>
<p>The page’s address remembers the Division you picked, so you can bookmark it or open it on a projector and it will
stay on that Division.</p>
<p>If you see <strong>No games in this division have finished yet</strong>, nothing has ended in that Division yet.
Teams appear as their first Games end.</p>
<p class="doc-note">LIVE Stats and Server Stats always agree. Server Stats doesn’t update by itself, though: switch
views or reload it to see the latest.</p>

<h2>Importing Games from an offline Room (sneakernet)</h2>
<p>If a Room’s QuizMachine couldn’t reach QView (no internet, say), its Games never arrived. You can bring them in by
exporting them from QuizMachine to a USB drive and importing the file into QView. <i>When you use your <b>sneakers</b> to 
physically transport the stats and then import them into QView, you're using "sneakernet".</i></p>
<p><strong>You’ll need:</strong> permission to update Games in this Tournament, and the Room’s Games exported from
QuizMachine as a <code>.csv</code> file (see
<a href="doc:quizmachine/exporting-quiz-round-data">Exporting Quiz Round Data from QuizMachine</a>). Only Games
marked <strong>DONE</strong> in QuizMachine are exported.</p>
<ol>
  <li>On <strong>Server Stats</strong>, set <strong>Data</strong> to <strong>Games Selection</strong>.</li>
  <li>Click <strong>Import (SneakerNet)</strong> and choose the <code>.csv</code> file.</li>
  <li>Read the preview. Nothing has been saved yet.</li>
  <li>If the <strong>Games to Import</strong> list looks right, click <strong>Import</strong>.</li>
  <li>When you see <strong>Imported game events for N game(s).</strong>, click <strong>Close</strong>.</li>
</ol>
<figure>
  <img src="/docs/qview/computing-stats-6.webp" alt="The Import Game Events preview." loading="lazy" />
  <figcaption>The Import Game Events preview.</figcaption>
</figure>
<p>The preview has four lists:</p>
<ul>
  <li><strong>Games to Import</strong>: Games that will come in, with how many events each has.</li>
  <li><strong>Errors</strong>: Games that will be skipped, and why.</li>
  <li><strong>Games Not Found</strong>: Games in the file that QView couldn’t match to a Game in this Tournament.</li>
  <li><strong>Missing Game Events</strong>: other Games in the same Rooms that aren’t in the file. This is a reminder
  in case you meant to export them too.</li>
</ul>
<p><strong>How does QView match a Game?</strong> By the Tournament name, Division name, Room name and Round name in
the file. They must match what’s in QView exactly (the Tournament name can also be blank). Games are imported
together, and any Game with a problem is skipped rather than stopping the import.</p>

<h3>When the import doesn’t work</h3>
<ul>
  <li><strong>“Only .csv files are accepted.”</strong> Choose the <code>.csv</code> file QuizMachine exported
  (usually <code>Quizzes.csv</code>).</li>
  <li><strong>A Game is under Games Not Found.</strong> A Division, Room or Round name in QuizMachine doesn’t match
  QView. Check the spelling in both places.</li>
  <li><strong>“This game already has game events beyond initialization; import skipped.”</strong> QView already has
  that Game’s quiz data, from the Room or an earlier import. Importing the same file twice is safe; this is why.</li>
  <li><strong>“Parsing error: …”</strong> The file isn’t in QuizMachine’s export format (for example, it was edited
  in a spreadsheet). Export it again from QuizMachine.</li>
  <li><strong>“question numbers are not sequential” or “missing event number”</strong> Part of the Game is missing
  from the file. Re-export it from that Room’s QuizMachine.</li>
</ul>

<h2>Exporting results</h2>
<ol>
  <li>On <strong>Server Stats</strong>, pick the Division and the <strong>Data</strong> view you want:
  <strong>Team Stats</strong>, <strong>Individual Stats</strong> or <strong>Games Selection</strong>.</li>
  <li>Click <strong>Export</strong> and choose <strong>CSV (.csv)</strong>, <strong>Excel (.xlsx)</strong> or
  <strong>PDF (.pdf)</strong>.</li>
</ol>
<figure>
  <img src="/docs/qview/computing-stats-7.webp" alt="The Export menu on Server Stats." loading="lazy" />
  <figcaption>The Export menu on Server Stats.</figcaption>
</figure>
<p>The file downloads straight away, named after the view (<code>team-stats</code>, <code>individual-stats</code>
or <code>game-selection</code>). It contains the whole table, not just the rows on screen. In Team Stats exports,
the Rank Rules column is written out in words. PDFs of Team and Individual stats print in landscape.</p>
<p class="doc-note">The <strong>Export</strong> button is greyed out when the table is empty. Pick a Division that has
finished Games.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:monitoring-rooms">Monitoring Rooms and Resending Game Data</a></li>
  <li><a href="doc:setting-up-rooms">Setting Up Rooms using QuizMachine + QView</a></li>
  <li><a href="doc:divisions-and-registration">Setting Up Divisions and Registration</a></li>
</ul>
`.trim(),
    },
  ],
}
