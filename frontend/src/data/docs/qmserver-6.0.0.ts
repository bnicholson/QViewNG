// QMServer 6.0.0 — the QMServer half of the "QuizMachine Documentation v.6.0.0" user guide
// (April 2, 2025). QMServer ships with QuizMachine, so the two manuals share a source file;
// sections the manual performs *inside QuizMachine* live in the QuizMachine category instead.
//
// The manual's "(link)" bookmarks are preserved as `doc:` links — see resolveDocHref in ./index.

import type { DocVersion } from './types'

export const qmserver600: DocVersion = {
  version: '6.0.0',
  released: 'April 2, 2025',
  nav: [
    {
      title: 'Getting Started',
      children: [
        { title: 'QMServer Workflow Overview', slug: 'qmserver-workflow-overview' },
        { title: 'Start Menu & Taskbar Shortcuts', slug: 'shortcuts' },
      ],
    },
    {
      title: 'Network-Based Functions',
      children: [
        { title: 'Enabling Network Connectivity to QMServer', slug: 'enabling-network-connectivity' },
        { title: 'Pulling Quiz Round Data from QuizMachine Clients', slug: 'pulling-quiz-round-data' },
      ],
    },
    {
      title: 'Names/Schedule Data',
      children: [
        { title: 'Creating/Getting a Names/Schedule File', slug: 'creating-names-schedule-file' },
        { title: 'Importing Names & Schedule Data into QMServer', slug: 'importing-names-and-schedule' },
        { title: 'Exporting Names & Schedule Data from QMServer', slug: 'exporting-names-and-schedule' },
      ],
    },
    {
      title: 'Quizzes Data',
      children: [
        { title: 'Importing Quiz Round Data into QMServer', slug: 'importing-quiz-round-data' },
      ],
    },
    {
      title: 'Stats',
      children: [
        { title: 'Creating a Stats Group/Tournament', slug: 'creating-a-stats-group' },
        {
          title: 'Getting Stats from Quiz Round Data',
          children: [
            { title: 'Getting Stats (Team & Individual)', slug: 'getting-stats' },
            { title: 'Quizzers Who Quizzed Fewer Rounds', slug: 'stats-fewer-rounds' },
          ],
        },
        {
          title: 'Sorting Stats',
          children: [
            { title: 'Sorting Stats', slug: 'sorting-stats' },
            { title: 'By Team: Win/Loss, Olympic, Modified Olympic', slug: 'sorting-stats-by-team' },
          ],
        },
        { title: "Renaming/Correcting a Quizzer's or Team's Name", slug: 'renaming-a-quizzer-or-team' },
        { title: 'Saving/Printing Stats', slug: 'saving-and-printing-stats' },
        {
          title: 'Stats Catalog',
          children: [
            { title: 'Stats Catalog: Generating More Stats', slug: 'stats-catalog' },
            { title: 'Top 3 Teams', slug: 'stat-top-3-teams' },
            { title: 'Top 15 Individual Quizzers, District', slug: 'stat-top-15-district' },
            { title: 'Top 15 Individual Quizzers, Field/Regional', slug: 'stat-top-15-field' },
            { title: 'Error King and Error Queen', slug: 'stat-error-king-and-queen' },
            { title: 'Top 3 Memory Verse Quizzers, Monthly', slug: 'stat-memory-verse-monthly' },
            { title: 'Top 3 Memory Verse Quizzers, End-Of-Season', slug: 'stat-memory-verse-season' },
            { title: 'Most Prolific Jumper', slug: 'stat-most-prolific-jumper' },
            { title: 'Highest Individual Average, QuizMachine Method', slug: 'stat-highest-average-quizmachine' },
            { title: 'Highest Individual Average, Manual Method', slug: 'stat-highest-average-manual' },
            { title: 'Quizzers Who Memorized All Memory Verses', slug: 'stat-memorized-all-verses' },
          ],
        },
      ],
    },
    {
      title: 'Data File Formats',
      children: [
        { title: 'Format: Names/Schedule File', slug: 'format-names-schedule-file' },
        { title: 'Format: Quizzes File (Quiz Round Data)', slug: 'format-quizzes-file' },
      ],
    },
  ],
  articles: [
    {
      slug: 'qmserver-workflow-overview',
      title: 'QMServer Workflow Overview (for use at a Quiz)',
      contentHtml: `
<p><em>QMServer: A QuizMachine companion program.</em></p>

<h2>Introduction to QMServer</h2>
<p>You can automate some of the steps of your quiz meet and generate stats by using the QMServer companion
program.</p>

<p>When you install QuizMachine the compatible version of QMServer is also installed. To get started, open
QMServer. If you don't know how to launch QMServer, see
<a href="doc:shortcuts">how you can create Start Menu and Taskbar shortcuts for launching the QMServer
program</a>.</p>
<figure>
  <img src="/docs/qmserver-6.0.0/qmserver-workflow-overview-1.webp" alt="QuizMachine's Server/Sneakernet tab, where the server address and “Use the Server” live." loading="lazy" />
  <figcaption>QuizMachine's Server/Sneakernet tab, where the server address and “Use the Server” live.</figcaption>
</figure>

<p>QMServer's left-hand menu contains: General, Network, Rooms, Schedule, Quizzes, Teams, Individuals,
Buildings, Stats, QuizMaster Stats, Kiosks and Help Requests.</p>
<figure>
  <img src="/docs/qmserver-6.0.0/qmserver-workflow-overview-2.webp" alt="The QMServer window on opening, with its left-hand section list." loading="lazy" />
  <figcaption>The QMServer window on opening, with its left-hand section list.</figcaption>
</figure>

<p class="doc-note">In version 6, QMServer has a new <strong>embedded web server</strong> that is faster and
able to perform better under heavy loads at bigger tournaments. See
<a href="doc:quizmachine/new-in-version-6">New With Version 6</a>.</p>

<h2>Tips and suggestions when using QMServer</h2>
<ul>
  <li>When testing and becoming more familiar with QMServer, use the same computer for the server (QMServer) as
  for the client (QuizMachine) so that both are being used on the same machine.</li>
  <li>Always use the latest version of QMServer to minimize risk of bugs.</li>
</ul>

<h3>Registrations (data/flow optimization)</h3>
<p>One thing that an event coordinator can do is to require online team and quizzer registrations with specific
registration open and close dates. Many districts hold online registration for their district quiz events on
their district website (most notably different sub-domains of the
<a href="https://nazquizzing.org" target="_blank" rel="noopener noreferrer">NazQuizzing.org</a> website). Once
registration has closed, a Names/Schedule file is exported and created, ready to be used at the quiz meet to
seed QuizMachine (client) programs with data for the quiz. This is much less trouble than doing entry while at
the quiz.</p>

<h2>QMServer workflow (for use at a quiz)</h2>

<h3>1. Required before starting</h3>
<p>All QuizMachine programs being used in quiz rooms should be installed with the same version of QuizMachine
and the same version of QMServer. With one QMServer system being used between all QuizMachine programs, this
means that all QuizMachine system versions that are being used should match.</p>
<p>The most current version of QuizMachine (and therefore QMServer) can be downloaded from
<a href="https://quizstuff.com" target="_blank" rel="noopener noreferrer">QuizStuff.com</a>. If you have any
questions please submit them to <a href="mailto:quizstuff@quizstuff.com">quizstuff@quizstuff.com</a>.</p>

<h3>2. Optional: Import Names/Schedule file into QMServer</h3>
<p>This step is <u>optional</u>. If you are not familiar with the Names/Schedule file then skip to the next
step.</p>
<p>If you would like to have the Names/Schedule file on-hand in QMServer available for export whenever
necessary, then you can import it — see
<a href="doc:importing-names-and-schedule">importing Names/Schedule into QMServer from a file</a>.</p>
<p>If the Names/Schedule file needs to be edited in any way, this can be done in a spreadsheet editor such as
Microsoft Excel or Google Sheets. Once the file is correct, you can import it into QMServer to have on-hand for
the quiz day.</p>

<h3>3. Data Transfer 1, Part 1: Get or create a Names/Schedule file</h3>
<p>Before the server functions can be used for a quiz, quizzer and team names need to be entered into the client
(QuizMachine program) in each quiz room. This is so that rounds can be entered using the names and schedule data
from the file for ease of use throughout the quiz day for each quiz room.</p>
<p>See <a href="doc:creating-names-schedule-file">creating/getting a Names/Schedule file</a>.</p>

<h3>4. Data Transfer 1, Part 2: Import and load into QuizMachine</h3>
<p>See <a href="doc:quizmachine/importing-names-and-schedule">importing and loading Names/Schedule data into
QuizMachine from a file</a>.</p>

<h3>5. Now you are ready to quiz!</h3>
<p>Allow all rooms to proceed as usual with each quiz round.</p>

<p class="doc-warning">If it is discovered that a name (team or quizzer) was misspelled in the Names/Schedule
file that was imported into the QuizMachine programs, <strong>DO NOT try to change it during the quiz</strong>.
Make sure your QuizMasters are aware that any name changes or typo corrections can be done later when computing
statistics at the end of the quiz day, and that they should NOT manually make a typo correction to the
name.</p>

<p><strong>Reason #1:</strong> This creates more work to correct for those compiling the stats. With certain
events having many rooms and many quiz rounds with many teams and many quizzers, this can amount to a lot of
work. It can even lead to loss of statistical data, which nobody wants to happen. In thinking of the interests
of others above our own and doing unto others as you would have them do unto you, please be considerate and kind
to the people who are working hard to give everyone a great quizzing experience during the event by following
this policy.</p>

<p><strong>Reason #2:</strong> QuizMachine recognizes each unique name as a separate entity (team or quizzer),
and if the typo is manually corrected in one room but not all rooms across all rounds, then the stats for that
team or quizzer could be split between two or more "teams" or two or more "quizzers". If a name with a different
spelling is added by a QuizMaster it will cause a new individual to exist for each different spelling in the end
results. The best policy on name changes is to not tolerate them until the end of the day when stats are being
computed.</p>

<h3>6. Data Transfer 2, Part 1: Export quiz round data from QuizMachine (client)</h3>
<p>See <a href="doc:quizmachine/exporting-quiz-round-data">how to export quiz round data from QuizMachine to a
file</a>. Alternatively, you may want to
<a href="doc:pulling-quiz-round-data">transfer quiz round data to QMServer via your local WiFi network
(LAN)</a>.</p>

<h3>7. Data Transfer 2, Part 2: Import quiz round data into QMServer</h3>
<p>See <a href="doc:importing-quiz-round-data">importing quiz round data into QMServer</a>.</p>

<h3>8. Set up a tournament for getting stats in QMServer</h3>
<p>The remainder of this workflow is covered in
<a href="doc:getting-stats">getting team and individual statistics in QMServer</a>. That section contains many
more sets of instructions related to statistics in QMServer — if you haven't checked it out go ahead and take a
look!</p>
`.trim(),
    },
    {
      slug: 'shortcuts',
      title: 'Creating Start Menu and Taskbar Shortcuts for Launching QMServer',
      contentHtml: `
<p>To make it easier to access QMServer, you can create shortcuts on your computer. This process is very much
the same as
<a href="doc:quizmachine/shortcuts">creating shortcuts for the Start Menu and Taskbar for QuizMachine</a>, so
you may want to review how that works.</p>

<p>The only difference is that you would search <strong>"QMServer"</strong> instead of "QuizMachine" when
searching for the program to create a shortcut for.</p>
<figure>
  <img src="/docs/qmserver-6.0.0/shortcuts-1.webp" alt="Searching for QMServer instead of QuizMachine, then Pin to Start / Pin to taskbar." loading="lazy" />
  <figcaption>Searching for QMServer instead of QuizMachine, then Pin to Start / Pin to taskbar.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:qmserver-workflow-overview">QMServer Workflow Overview</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'enabling-network-connectivity',
      title: 'Enabling Network Connectivity to QMServer',
      contentHtml: `
<p>This is a required step in connecting QuizMachine to QMServer via a network connection.</p>

<p class="doc-note">QMServer opens on <strong>Port 80</strong>. QuizMachine opens on
<strong>Port 6258</strong>.</p>

<h2>Steps</h2>
<ol>
  <li>Launch QMServer. Once QMServer is running, note that it opens Port 80 for sending data. You will notice
  QMServer's <strong>Network</strong> menu item starts out in a <strong>red</strong> font color.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/enabling-network-connectivity-1.webp" alt="“Network” shown in red — network availability is off." loading="lazy" />
  <figcaption>“Network” shown in red — network availability is off.</figcaption>
</figure>
  <li>Right click <strong>Network</strong>. Then click <strong>On</strong> to turn on network availability.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/enabling-network-connectivity-2.webp" alt="The Network menu, with On and Off." loading="lazy" />
  <figcaption>The Network menu, with On and Off.</figcaption>
</figure>
  <li>When the mouse menu closes upon clicking "On" you will see "Network" is now in a <strong>black</strong>
  font color. When "Network" is black, you know that the network availability of QMServer is currently
  active.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/enabling-network-connectivity-3.webp" alt="“Network” shown in black — the server is now active." loading="lazy" />
  <figcaption>“Network” shown in black — the server is now active.</figcaption>
</figure>
</ol>

<p class="doc-warning">Note that the server is active on the connected network only so long as the QMServer
application is open. If QMServer closes, the server shuts off and all connections are terminated between
QMServer and all previously connected QuizMachine clients.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:quizmachine/connecting-to-qmserver">Connecting QuizMachine to QMServer</a></li>
  <li><a href="doc:pulling-quiz-round-data">Pulling quiz round data from QuizMachine clients</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'pulling-quiz-round-data',
      title: 'Pulling Quiz Round Data into QMServer from QuizMachine Clients via Network Connection',
      contentHtml: `
<p>To get quiz round data from QuizMachine clients, all that is needed is to maintain a network connection
between QuizMachine (client) and QMServer.</p>

<h2>Troubleshooting the connection</h2>
<p>These factors contribute to maintaining that connection, and can be used to troubleshoot connectivity
issues:</p>
<ul>
  <li>Check that the IP or website address in QuizMachine &gt; Setup &gt; Server/SneakerNet is correct.</li>
  <li>Check that QMServer is open and running on the computer that has that IP or website address.</li>
  <li>Make sure that network functions are turned on in QMServer —
  <a href="doc:enabling-network-connectivity">more info</a>.</li>
  <li>Ensure that a Schedule file is loaded in QMServer in the top-level "Schedule" section.</li>
</ul>

<h2>How do you know if data is coming across correctly?</h2>

<h3>The primary way</h3>
<p>Ensure that the QuizMachine client for that specific room currently has a connection to QMServer by looking at
the top-level <strong>Rooms</strong> section in QMServer. The room will show here if the QuizMachine client in
that room is connected. If it is not here then it is not connected to this instance of QMServer.</p>
<figure>
  <img src="/docs/qmserver-6.0.0/pulling-quiz-round-data-1.webp" alt="A connected room listed in QMServer's Rooms section." loading="lazy" />
  <figcaption>A connected room listed in QMServer's Rooms section.</figcaption>
</figure>

<h3>An additional way</h3>
<p>Check the data coming into the top-level <strong>Quizzes</strong> section (or the equivalent section in any
stats group) to see if the data that currently shows there for that room matches the expected round that the quiz
room should be on.</p>

<p>One example of something that should not happen is if a later round, say round 5, is complete while a previous
round, like round 4, is not complete — this would need manual attention to resolve. Another example is when you
are expecting to see all rooms on round 8 or 9 but you see one of the rooms is in the middle of round 6; this
would require finding out what has happened in that room.</p>
<figure>
  <img src="/docs/qmserver-6.0.0/pulling-quiz-round-data-2.webp" alt="The Quizzes section — note Round 4 with DataOk “N” and “Missing an event”." loading="lazy" />
  <figcaption>The Quizzes section — note Round 4 with DataOk “N” and “Missing an event”.</figcaption>
</figure>

<p class="doc-note">It is important to work with the most recent quiz round data. To view the latest quiz round
information you will need to click on any other section and then navigate back to the "Quizzes" section.</p>

<h2>Reading the "DataOk" column</h2>
<p>A round with value <strong>N</strong> for column <strong>DataOk</strong> and a message of "Missing an event"
means that a required event for the quiz round is absent and needs attention. To troubleshoot:</p>
<ol>
  <li>Click the correct round to highlight it in blue.</li>
  <li>Right-click to reveal the mouse menu.</li>
  <li>Select <strong>Scoresheet</strong>.</li>
</ol>
<figure>
  <img src="/docs/qmserver-6.0.0/pulling-quiz-round-data-3.webp" alt="The scoresheet for the flagged round, showing the gap in recorded questions." loading="lazy" />
  <figcaption>The scoresheet for the flagged round, showing the gap in recorded questions.</figcaption>
</figure>
<p>If, for example, only one team shows with two quizzers and records for only 5 of the 20 questions, with a big
gap of missing quiz round records, there is clearly an issue with the data recorded for that round. Resolving this
would involve communicating with the QuizMaster, the Coaches and anyone else who kept score for the round to
manually correct the quiz round for this room on the client QuizMachine program.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:quizmachine/exporting-quiz-round-data">Exporting quiz round data to a file instead</a></li>
  <li><a href="doc:importing-quiz-round-data">Importing quiz round data into QMServer</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'creating-names-schedule-file',
      title: 'Creating/Getting a Names/Schedule File',
      contentHtml: `
<p>The schedule file is used to import quiz team, quizzer, room and round seed data into each QuizMachine program
in each quiz room before the start of a quiz.</p>

<p class="doc-note">You may want to have the Names/Schedule file on-hand at all times by importing it into
QMServer so that you can export it at any time for your own convenience. See
<a href="doc:importing-names-and-schedule">how to import a Names/Schedule file into QMServer</a>.</p>

<h2>Benefits of use</h2>
<p>There are a handful of benefits that come with using a schedule file to set up quiz rooms:</p>
<ol>
  <li>Allows for consistent and accurate data throughout the quiz across all QuizMachine programs.</li>
  <li>Decreases probability of quiz meet procedural errors concerning which team is competing against which, in
  what room, and on what round (in other words, it keeps track of which round is next for a room).</li>
  <li>Reduces work done by the quizmaster before each round (smoother experience).</li>
  <li>Allows quiz round data (from the digital scoresheet) to be exported and processed much more efficiently
  after all quizzing for that room has concluded.</li>
</ol>

<h2>Obtaining the file</h2>
<p>You can get the schedule file by:</p>
<ul>
  <li><a href="doc:exporting-names-and-schedule">exporting it from QMServer</a>,</li>
  <li>retrieving it from an online source (such as from registrations for the quiz meet on the District Bible
  Quizzing website),</li>
  <li>getting it from your quiz director, or</li>
  <li>creating the file yourself.</li>
</ul>

<h2>Creating the file</h2>
<p>If you need to, you can create a Names/Schedule file with the appropriate information using
<a href="doc:format-names-schedule-file">Names/Schedule data file formatting</a>.</p>
<p class="doc-note">It is recommended that this be done only as a last resort.</p>

<h2>Editing the file</h2>
<p>If you would like to edit the Names/Schedule file — say, if there is a typo or a team name change — then you
can edit the CSV file using spreadsheet editing software such as Microsoft Excel or Google Sheets.</p>
<p class="doc-warning">Be sure that if you do this you also export/save the file in <strong>CSV
format</strong>. Otherwise, the file will not be able to be imported into QuizMachine or QMServer.</p>

<h2>Manually validating the data</h2>
<p>Before importing you should always verify that the spellings of the names for each team and quizzer in the
schedule file are correct and consistent throughout each line. This is necessary to ensure proper use of the
schedule file by QuizMachine once it is imported.</p>

<p>For example, if a team name is spelt one way on one line and another way on another line, when the file is
loaded into QuizMachine it will create one team for each different spelling of the team name. (Additionally,
quizzers appreciate when their names are shown with the correct spelling in the QuizMachine interface, just the
same as you or I would appreciate our names having the correct spellings!)</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:format-names-schedule-file">Format: Names/Schedule File</a></li>
  <li><a href="doc:quizmachine/importing-names-and-schedule">Loading it into QuizMachine</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'importing-names-and-schedule',
      title: 'Importing Names and Schedule Data into QMServer from a File',
      contentHtml: `
<ol>
  <li>Launch QMServer. (If you don't know how, see
  <a href="doc:shortcuts">how you can create Start Menu and Taskbar shortcuts for launching the QMServer
  program</a>.)</li>
  <li>In the top menu to the left click <strong>Export/Import</strong>.</li>
  <li>Then in that menu click <strong>Import Names/Schedule</strong>.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/importing-names-and-schedule-1.webp" alt="The Export/Import menu in QMServer." loading="lazy" />
  <figcaption>The Export/Import menu in QMServer.</figcaption>
</figure>
  <li>A file explorer window will appear. In the file tree of your computer's operating system, navigate to the
  file you want to import — often on the inserted USB thumb drive that you are using to transfer data between the
  quiz rooms and the QMServer computer.</li>
</ol>

<p class="doc-note">If you ever want to remove all schedule data from QMServer (clearing the data out) you can
always create a blank CSV file and import that file by the same means described above.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:creating-names-schedule-file">Creating/Getting a Names/Schedule File</a></li>
  <li><a href="doc:exporting-names-and-schedule">Exporting Names &amp; Schedule Data from QMServer</a></li>
  <li><a href="doc:format-names-schedule-file">Format: Names/Schedule File</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'exporting-names-and-schedule',
      title: 'Exporting Names & Schedule Data from QMServer to a File',
      contentHtml: `
<p>To export names and schedule data from QMServer (for top-level only and not stats groups):</p>

<ol>
  <li>Click <strong>Export/Import</strong> in the upper left-hand corner.</li>
  <li>Then click <strong>Export Names</strong>.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/exporting-names-and-schedule-1.webp" alt="Export Names in the Export/Import menu." loading="lazy" />
  <figcaption>Export Names in the Export/Import menu.</figcaption>
</figure>
  <li>
    <p>A file browser window will appear. Give the file a name and choose the location to save the file to. Then
    click <strong>Save</strong>. The names &amp; schedule will begin exporting to a file at that location.</p>
    <p class="doc-note">If this is to be imported later into QuizMachine (client) then you may want to save it
    directly to the USB thumb drive.</p>
  </li>
</ol>

<h2>See also</h2>
<ul>
  <li><a href="doc:quizmachine/importing-names-and-schedule">Loading the file into QuizMachine</a></li>
  <li><a href="doc:creating-names-schedule-file">Creating/Getting a Names/Schedule File</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'importing-quiz-round-data',
      title: 'Importing Quiz Round Data into QMServer from a File',
      contentHtml: `
<p>Importing quiz round data usually happens at the end of a quiz day. The data is normally exported from
QuizMachine onto a USB thumb drive to be imported into QMServer for running stats. See
<a href="doc:quizmachine/exporting-quiz-round-data">how to export quiz round data from QuizMachine to a
file</a>.</p>

<p>Once you have the quizzes file (default name is <code>Quizzes.csv</code>):</p>

<ol>
  <li>Launch QMServer. (If you don't know how, see
  <a href="doc:shortcuts">how you can create Start Menu and Taskbar shortcuts for launching the QMServer
  program</a>.)</li>
  <li>Click <strong>Export/Import</strong> in the upper left.</li>
  <li>Then click <strong>Import Quizzes</strong>.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/importing-quiz-round-data-1.webp" alt="Import Quizzes in the Export/Import menu." loading="lazy" />
  <figcaption>Import Quizzes in the Export/Import menu.</figcaption>
</figure>
  <li>A file browser window will appear. Find and select the file with the quiz round data in it. If you are
  importing data from multiple rooms you will need to select multiple quiz files (using
  <strong>Ctrl+click</strong> or <strong>Shift+click</strong>).</li>
  <li>Once all quizzes are selected then click <strong>Open</strong>. You will see a loading progress window while
  loading the quizzes. Once complete you will see a confirmation window such as "13 quizzes were imported or
  updated".</li>
<figure>
  <img src="/docs/qmserver-6.0.0/importing-quiz-round-data-2.webp" alt="The import confirmation." loading="lazy" />
  <figcaption>The import confirmation.</figcaption>
</figure>
  <li>You can check in the <strong>Quizzes</strong> tab at either the top level or in a stats group/tournament
  level. You should see the quizzes populate there.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/importing-quiz-round-data-3.webp" alt="The Quizzes section after import." loading="lazy" />
  <figcaption>The Quizzes section after import.</figcaption>
</figure>
</ol>

<p class="doc-note">It is important that all quiz rooms' data is included before proceeding to run stats.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:format-quizzes-file">Format: Quizzes File (Quiz Round Data File)</a></li>
  <li><a href="doc:getting-stats">Getting stats from quiz round data</a></li>
  <li><a href="doc:pulling-quiz-round-data">Pulling quiz round data over the network instead</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'creating-a-stats-group',
      title: 'Creating a Stats Group/Tournament in QMServer',
      contentHtml: `
<p>Some people refer to the following as how to set up a "Tournament" in QMServer. In this documentation this
"Tournament" will be referred to as a <strong>stats group</strong> due to its structure and functionality. For
each quiz you run stats for you will want to create a new stats group in QMServer.</p>

<h2>Steps</h2>
<ol>
  <li>Launch QMServer. (If you don't know how, see
  <a href="doc:shortcuts">how to create Start Menu and Taskbar shortcuts for launching the QMServer
  program</a>.)</li>
  <li>At the bottom of the left-hand menu you will see an item called <strong>Stats</strong>. When there is
  neither a plus sign (+) nor a minus sign (−) to the left of this menu item then you will know that there are
  currently no stats groups entered in this QMServer program. If you do see (+) or (−) next to "Stats" then stats
  groups currently exist.</li>
  <li>Right click <strong>Stats</strong> and click <strong>Add</strong>.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/creating-a-stats-group-1.webp" alt="The Stats menu with Add." loading="lazy" />
  <figcaption>The Stats menu with Add.</figcaption>
</figure>
  <li>An "Enter Stat Name" window will appear.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/creating-a-stats-group-2.webp" alt="The Enter Stat Name window — this name cannot be changed later." loading="lazy" />
  <figcaption>The Enter Stat Name window — this name cannot be changed later.</figcaption>
</figure>
  <li>
    <p>Type in a name for the stats group and click <strong>OK</strong>.</p>
    <ul>
      <li>You will probably want the name to be the same as the Tournament the stats will be run for.</li>
      <li class="doc-warning">Please note that you <strong>cannot change this name later</strong>, but you can
      delete the stats group entirely and create a new one with the proper name.</li>
    </ul>
  </li>
  <li>With the new stats group added, click the plus icon (+) to the left of "Stats" to open it and view the stats
  groups nested below it. When "Stats" is open, the (+) will change to (−) and you will be able to see the items
  nested under it: Quizzes, Schedule, Team, Team (by Round), Individual, Individual (by Round) and Summary.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/creating-a-stats-group-3.webp" alt="The new stats group expanded, showing its nested sections." loading="lazy" />
  <figcaption>The new stats group expanded, showing its nested sections.</figcaption>
</figure>
</ol>

<h2>See also</h2>
<ul>
  <li><a href="doc:getting-stats">Getting Stats (Team &amp; Individual) from Quiz Round Data</a></li>
  <li><a href="doc:sorting-stats">Sorting Stats</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'getting-stats',
      title: 'Getting Stats (Team & Individual) from Quiz Round Data',
      contentHtml: `
<ol>
  <li>First, you will need to
  <a href="doc:importing-quiz-round-data">import quiz round data into QMServer from a file</a>. Alternatively,
  you can also <a href="doc:pulling-quiz-round-data">transfer this data from QuizMachine to QMServer over your
  local WiFi network</a>.</li>
  <li>Next, <a href="doc:creating-a-stats-group">create a stats group ("Tournament") in QMServer</a>.</li>
  <li>Under the new stats grouping that you created, click <strong>Quizzes</strong>; the quizzes currently in your
  QMServer program (stored locally) will display to the right.</li>
  <li>
    <p>Select which quizzes you want to include in your stats group. You will see all quizzes currently uploaded
    to the QMServer program installed on your computer, but you will most likely want to run stats only for the
    quizzes from today.</p>
    <p class="doc-note">You can click the column headers to change the sorting of the quizzes list to make it
    easier to find the quizzes that you want to select.</p>
  </li>
</ol>

<h2>Ways to select quizzes</h2>
<figure>
  <img src="/docs/qmserver-6.0.0/getting-stats-1.webp" alt="Selecting quiz rounds in the stats group's Quizzes list." loading="lazy" />
  <figcaption>Selecting quiz rounds in the stats group's Quizzes list.</figcaption>
</figure>
<ul>
  <li><strong>One-by-one:</strong> Right click on the quiz you want to select for stats and then click
  <strong>Select</strong>.</li>
  <li><strong>Click and drag</strong> your mouse to select multiple quizzes that are directly above or below each
  other, then right click the blue selection area and click <strong>Select</strong>.</li>
  <li><strong>Shift</strong> to select the first and last item in a range of quizzes. Then right click one of the
  highlighted quizzes (in blue) and click <strong>Select</strong>.</li>
  <li><strong>CTRL+click</strong> specific quizzes that you want to select. Then right click one of the
  highlighted quizzes and click <strong>Select</strong>.</li>
</ul>

<p>You can tell which quizzes are selected by looking at the <strong>Selected</strong> column for each quiz. If
the quiz has "Y" in that column, then it is currently selected. Otherwise, the quizzes that are not selected will
show nothing in the column.</p>
<figure>
  <img src="/docs/qmserver-6.0.0/getting-stats-2.webp" alt="The Selected column showing “Y” for each included round." loading="lazy" />
  <figcaption>The Selected column showing “Y” for each included round.</figcaption>
</figure>

<p class="doc-note">You may want to check that each quiz you select is Done ("Y" in the <strong>Done</strong>
column) and the <strong>Question</strong> equals "21" to ensure the round's data is complete for that room.</p>

<p class="doc-note">If you do not see "Select" as an option when right-clicking on a highlighted quiz, you may be
looking at the wrong "Quizzes" section. Make sure you are looking at the "Quizzes" section that is nested below
the stats group you created.</p>

<h2>Viewing the stats</h2>
<p>With the appropriate quizzes selected the stats are now ready to be viewed. Click the other menu items under
the stats section, such as <strong>Team</strong> to view team stats or <strong>Individual</strong> to view stats
for each individual quizzer.</p>

<p>If you want to deselect quizzes you can use the above methods in the same manner, with the only difference
being that you click the <strong>Unselect</strong> mouse menu option instead of "Select".</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:stats-fewer-rounds">Handling stats for quizzers who quizzed fewer rounds</a></li>
  <li><a href="doc:sorting-stats">Sorting Stats</a></li>
  <li><a href="doc:saving-and-printing-stats">Saving/Printing Stats</a></li>
  <li><a href="doc:renaming-a-quizzer-or-team">Renaming/correcting a quizzer's or team's name</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'stats-fewer-rounds',
      title: 'Handling Stats for Quizzers who have Quizzed in Fewer Rounds than the Total Rounds for the Day',
      contentHtml: `
<p>For individuals, if someone quizzes in fewer rounds than their team, the team average is based on the number
of rounds for the <strong>team</strong>, not the individual.</p>

<p>Otherwise, for instance, a quizzer could potentially quiz one perfect round and — if anyone else missed a
question across all their rounds — the quizzer who quizzed one round would have a higher point percentage.</p>
<figure>
  <img src="/docs/qmserver-6.0.0/stats-fewer-rounds-1.webp" alt="Individual stats showing the Rnds column used for the average." loading="lazy" />
  <figcaption>Individual stats showing the Rnds column used for the average.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:getting-stats">Getting Stats (Team &amp; Individual)</a></li>
  <li><a href="doc:stat-highest-average-quizmachine">Highest Individual Average, QuizMachine Method</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'sorting-stats',
      title: 'Sorting Stats',
      contentHtml: `
<p>You can adjust the settings for sorting and scoring stats.</p>

<p class="doc-note">You need to have created a stats group before viewing the stats group's properties
window.</p>

<h2>Viewing a stats group's properties</h2>
<ol>
  <li>Click the name you gave to the stats group that you created (to focus your mouse on it).</li>
  <li>Then right-click on the stats group's name. A mouse menu will appear.</li>
  <li>Click <strong>Properties</strong>.</li>
</ol>

<p class="doc-note">If the Stats Properties window does not appear when you click "Properties", then you likely
did not click the name of the stats group before right-clicking it.</p>
<figure>
  <img src="/docs/qmserver-6.0.0/sorting-stats-1.webp" alt="The Stats Properties window, with Olympic point options and “Winner determined by”." loading="lazy" />
  <figcaption>The Stats Properties window, with Olympic point options and “Winner determined by”.</figcaption>
</figure>

<p>Each of the tabbed sections at the top of the window (Team, Individual, IndividualByRound) pertains to that
specific item in the sidebar menu in the chosen stats group.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:sorting-stats-by-team">By Team: Win/Loss, Olympic, Modified Olympic</a></li>
  <li><a href="doc:creating-a-stats-group">Creating a Stats Group/Tournament</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'sorting-stats-by-team',
      title: 'By Team: Win/Loss, Olympic, Modified Olympic',
      contentHtml: `
<p>In the first tab named <strong>Team</strong> you have the following options.</p>

<h2>Winner determined by</h2>
<p>At the bottom, "Winner is determined by" is the most significant field here.</p>
<ul>
  <li><strong>Win/Loss</strong> (the default) sorts teams by their team win/loss record.</li>
  <li><strong>Olympic Pts</strong> utilizes Olympic scoring methods, involving only the top-most box located in
  the same window tab.</li>
  <li><strong>Modified Olympic Pts</strong> — if you would like to score using Olympic scoring and have quiz
  rounds of both 2-team and 3-team match-ups, then you may want to select this option.</li>
</ul>

<h2>Olympic point options</h2>
<p>You can sort stats based on Olympic points awarded. Choose the number of points you want to award for 1st and
2nd place for 2-team quiz rounds, and 1st, 2nd and 3rd place for 3-team quiz rounds if these are also being
utilized.</p>

<p>Example defaults: a 3-team quiz awards 10 / 5 / 1; a 2-team quiz awards 8 / 3.</p>

<h2>Modified Olympic point options</h2>
<p>If 2-team AND 3-team quiz rounds are being utilized, then you can set the <strong>2 Team Percentage</strong>
modifier to adjust the point values awarded to teams that participate in 2-team match-ups, to make the points
awarded near-equivalent to the points awarded during 3-team quiz rounds. (The example default is 75.)</p>

<p>If you ever want to reset the system default for this value, click <strong>Reset to Defaults</strong>.</p>

<h2>Sort Method</h2>
<p class="doc-note"><em>What does "Sort Method" do?</em> It has values "Old" and "New" and does not gray out when
the "Winner is determined by" drop-down value changes, meaning it supposedly applies to all of the options. This
would suggest there is an old and a new option for one of the available options, and since "Win/Loss" is the
default/most popular it would be expected to apply to that one.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:sorting-stats">Sorting Stats</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'renaming-a-quizzer-or-team',
      title: "Renaming/Correcting a Quizzer's or Team's Name",
      contentHtml: `
<p>Renaming requires that (1) you have already
<a href="doc:creating-a-stats-group">created a stats group in QMServer</a> for the current quiz tournament and
(2) have already <a href="doc:getting-stats">selected the appropriate quizzes to include in stats</a>.</p>

<h2>Steps</h2>
<ol>
  <li>Go to <strong>Stats &gt; {stats group/tournament you created} &gt; Individual</strong> (or
  <strong>Team</strong>).</li>
  <li>Click the quizzer or team that you would like to make a name correction to. Then right-click to view the
  mouse options.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/renaming-a-quizzer-or-team-1.webp" alt="The Individual Stats menu with “Rename a Quizzer”." loading="lazy" />
  <figcaption>The Individual Stats menu with “Rename a Quizzer”.</figcaption>
</figure>
  <li>Click <strong>Rename a Quizzer</strong> (or <strong>Rename a Team</strong>). A rename window will appear
  showing the old name and the team name.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/renaming-a-quizzer-or-team-2.webp" alt="The Quizzer Rename window." loading="lazy" />
  <figcaption>The Quizzer Rename window.</figcaption>
</figure>
  <li>Type in a name where the drop-down selector is. Then click <strong>OK</strong> or press
  <strong>Enter</strong>. <strong>All occurrences of that name found in the scoresheets will be changed to the
  new one.</strong></li>
</ol>

<p class="doc-note">This is the correct way to fix a name typo — and the reason QuizMasters should never correct
a spelling mid-quiz. See <a href="doc:qmserver-workflow-overview">QMServer Workflow Overview</a>.</p>
`.trim(),
    },
    {
      slug: 'saving-and-printing-stats',
      title: 'Saving/Printing Stats',
      contentHtml: `
<p>After having completed the steps for <a href="doc:getting-stats">getting stats</a>:</p>

<ol>
  <li>
    <p>Make sure you have a printer connected to your computer, or one that is WiFi capable of connecting over
    your building's local network. Some things to keep in mind:</p>
    <ul>
      <li>You will want to choose the printer option that is more reliable based on what is available.</li>
      <li>Ensure the printer has adequate paper and ink.</li>
    </ul>
  </li>
  <li>Right click the menu item, such as <strong>Team</strong> or <strong>Individual</strong>, of the stats
  grouping that you want to print.</li>
  <li>Click <strong>Print</strong>. Your computer's printer dialog window will open. If you want to save the stats
  to a PDF, select the option for saving to a PDF. Otherwise select the printer to print to. Once you have chosen
  the options you want for printing, click <strong>Print</strong>.</li>
</ol>

<p class="doc-note">For individual stats, the Print option — even in landscape orientation — leaves off some
columns. For those, exporting to CSV and formatting in a spreadsheet is the better option. See
<a href="doc:stats-catalog">Stats Catalog</a>.</p>
`.trim(),
    },
    {
      slug: 'stats-catalog',
      title: 'Stats Catalog: Generating More Stats (for more Awards and such)',
      contentHtml: `
<p>This section serves as a catalog of different stats that can be generated from QuizMachine, as well as some
stats that are not but which you may find useful.</p>

<h2>Legend</h2>
<p>There are three descriptive terms used for each stat below:</p>
<table>
  <tr><th>Term</th><th>Values</th></tr>
  <tr><td>When?</td><td>District-Monthly / District-Year-End / Fields (Regionals)</td></tr>
  <tr><td>Who?</td><td>Per Division / Most Competitive Division Only</td></tr>
  <tr><td>How?</td><td>QuizMachine Stats / Manually-Calculated Stats</td></tr>
</table>

<p class="doc-note">The following stats and awards were taken from the regular awards that the MidSouth District
and SouthEast Field incorporate into their events. These are being shared here as a sample of stats that you can
use in your district and/or field events. Feel free to use these, not use these, modify these, change the award
names as you see fit, etc. for the purposes of your specific district / field.</p>

<p class="doc-warning">Please make sure to test each stats generation method before using it in a quiz meet.</p>

<h2>The catalog</h2>
<ul>
  <li><a href="doc:stat-top-3-teams">Top 3 Teams</a></li>
  <li><a href="doc:stat-top-15-district">Top 15 Individual Quizzers, District Lvl</a></li>
  <li><a href="doc:stat-top-15-field">Top 15 Individual Quizzers, Field / Regional Lvl</a></li>
  <li><a href="doc:stat-error-king-and-queen">Error King and Error Queen</a></li>
  <li><a href="doc:stat-memory-verse-monthly">Top 3 Memory Verse Quizzers, District Monthly</a></li>
  <li><a href="doc:stat-memory-verse-season">Top 3 Memory Verse Quizzers, District End-Of-Season</a></li>
  <li><a href="doc:stat-most-prolific-jumper">Most Prolific Jumper (Speed and Quantity)</a></li>
  <li><a href="doc:stat-highest-average-quizmachine">Highest Individual Average, QuizMachine Method</a></li>
  <li><a href="doc:stat-highest-average-manual">Highest Individual Average, Manual Method</a></li>
  <li><a href="doc:stat-memorized-all-verses">Awarding Quizzers Who Memorized All Memory Verses</a></li>
</ul>

<p class="doc-note">All of these awards can also be run for the entire season; the only difference is that the
End-of-Season awards include quiz rounds from the same division for ALL past quiz meets for the season instead of
just the current quiz meet. All of them can be used at the Field / Regional level also.</p>
`.trim(),
    },
    {
      slug: 'stat-top-3-teams',
      title: 'Top 3 Teams',
      contentHtml: `
<p><strong>District-Monthly, Per Division, QuizMachine Stats</strong></p>

<ol>
  <li><a href="doc:creating-a-stats-group">Create a stats group</a> for a specific division's current quiz meet
  stats.</li>
  <li>If you haven't already, import the quiz rounds you need into QMServer. You can do this
  <a href="doc:importing-quiz-round-data">by importing quizzes from a file</a> or
  <a href="doc:pulling-quiz-round-data">by transfer over WiFi/LAN</a>.</li>
  <li>Then in the newly created stats group, click <strong>Quizzes</strong> and select quiz rounds to include that
  are for the current quiz meet day and for the division that the stats group is for.</li>
  <li>Click <strong>Team</strong>, then view the top 3 teams by <strong>Total Pts</strong>.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/stat-top-3-teams-1.webp" alt="Team stats with Export Team Stats in the mouse menu." loading="lazy" />
  <figcaption>Team stats with Export Team Stats in the mouse menu.</figcaption>
</figure>
  <li>Left-click, then right-click <strong>Team</strong>, then click <strong>Print</strong> or <strong>Export Team
  Stats</strong> in the mouse menu.</li>
  <li>If <strong>Print</strong>: save to PDF in <u>Landscape</u> orientation.</li>
  <li>If <strong>Export Team Stats</strong>: save to local drive. Then in Excel or Sheets, adjust/format as
  necessary and print to PDF.</li>
</ol>

<h2>See also</h2>
<ul>
  <li><a href="doc:stats-catalog">Stats Catalog</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'stat-top-15-district',
      title: 'Top 15 Individual Quizzers (District)',
      contentHtml: `
<p><strong>District-Monthly, Per Division, QuizMachine Stats</strong></p>

<p class="doc-note">This is based on total points accumulated for the quiz. In cases where there is a tie, errors
are normally used for breaking the tie. Number of rounds quizzed by a quizzer is disregarded in these stats.</p>

<p class="doc-note">There is an option to Print and save to PDF. This option, even in landscape orientation,
leaves off some columns from the page. Therefore, the better option is the following.</p>

<ol>
  <li><a href="doc:creating-a-stats-group">Create a stats group</a> for a specific division's current quiz meet
  stats.</li>
  <li>If you haven't already, import the quiz rounds you need into QMServer. You can do this
  <a href="doc:importing-quiz-round-data">by importing quizzes from a file</a> or
  <a href="doc:pulling-quiz-round-data">by transfer over WiFi/LAN</a>.</li>
  <li>Then in the newly created stats group, click <strong>Quizzes</strong> and select quiz rounds to include that
  are for the current quiz meet day and for the division that the stats group is for.</li>
  <li>Click <strong>Individual</strong> to view the top 15 individual quizzers.</li>
  <li>Left-click, then right-click <strong>Individual</strong>, then click <strong>Export Individual
  Stats</strong> in the mouse menu. This will save the data in a CSV file on your local computer.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/stat-top-15-district-1.webp" alt="Individual stats with Export Individuals Stats in the mouse menu." loading="lazy" />
  <figcaption>Individual stats with Export Individuals Stats in the mouse menu.</figcaption>
</figure>
  <li>
    <p>In a spreadsheet editor such as Microsoft Excel or Google Sheets you can proceed with saving the
    spreadsheet as a PDF after formatting it the way you want to display it. Make sure to check the PDF preview
    contains all columns of data before saving.</p>
    <p class="doc-note">It is recommended that you save as a PDF before printing so that you are able to maintain
    a copy of the stats the way they were printed.</p>
  </li>
</ol>
`.trim(),
    },
    {
      slug: 'stat-top-15-field',
      title: 'Top 15 Individual Quizzers (Field / Regional)',
      contentHtml: `
<p><strong>Fields (Regionals), Per Division, QuizMachine Stats</strong></p>

<p>At field / regional events, quizzers of the same division may be broken up into pools. Each pool may have a
different number of teams and therefore a <u>different number of rounds</u>. For this reason, the individual
quizzer's <strong>average score</strong> is used to determine individual quizzer placement for the entire event
instead of total points scored.</p>

<p>For calculating this, please use one of the two methods for finding "Highest Individual Average":</p>
<ul>
  <li><a href="doc:stat-highest-average-quizmachine">Highest Individual Average, QuizMachine Method</a></li>
  <li><a href="doc:stat-highest-average-manual">Highest Individual Average, Manual Method</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'stat-error-king-and-queen',
      title: 'Error King and Error Queen',
      contentHtml: `
<p><strong>District-Monthly, Per Division, QuizMachine Stats</strong></p>

<p class="doc-note">There is an option to Print and save to PDF. This option, even in landscape orientation,
leaves off some columns from the page. Therefore, the better option is the following.</p>

<ol>
  <li><a href="doc:creating-a-stats-group">Create a stats group</a> for a specific division's current quiz meet
  stats.</li>
  <li>If you haven't already, import the quiz rounds you need into QMServer. You can do this
  <a href="doc:importing-quiz-round-data">by importing quizzes from a file</a> or
  <a href="doc:pulling-quiz-round-data">by transfer over WiFi/LAN</a>.</li>
  <li>Then in the newly created stats group, click <strong>Quizzes</strong> and select quiz rounds to include that
  are for the current quiz meet day and for the division that the stats group is for.</li>
  <li>Export to CSV by left-clicking, then right-clicking <strong>Individual</strong>; then click <strong>Export
  Individual Stats</strong> in the mouse menu.</li>
  <li>Open the CSV file in Excel or Google Sheets.</li>
  <li>Apply a filter to the <strong>Errors</strong> column so that the highest numbers are listed toward the
  top.</li>
  <li>The top rows are the quizzers with the most errors.</li>
  <li>From here, find the highest number for both male and female by looking at the name and determining if they
  are male or female.</li>
  <li>The male with the most errors is the <strong>Error King</strong> for the division, and the female with the
  most errors for her division is the <strong>Error Queen</strong>.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/stat-error-king-and-queen-1.webp" alt="Individual stats — the Errors column drives this award." loading="lazy" />
  <figcaption>Individual stats — the Errors column drives this award.</figcaption>
</figure>
  <li>
    <p>In a spreadsheet editor you can proceed with saving the spreadsheet as a PDF after formatting it the way
    you want to display it. Make sure to check the PDF preview contains all columns of data before saving.</p>
    <p class="doc-note">It is recommended that you save as a PDF before printing so that you are able to maintain
    a copy of the stats the way they were printed.</p>
  </li>
</ol>
`.trim(),
    },
    {
      slug: 'stat-memory-verse-monthly',
      title: 'Top 3 Memory Verse Quizzers (District Monthly)',
      contentHtml: `
<p><strong>District-Monthly, Per Division, Manually-Calculated Stats</strong></p>

<ol>
  <li>
    <p>Have each of your quizmasters record the quizzer + memory verse type per round and division on a piece of
    paper.</p>
    <p class="doc-note">In the MidSouth District there is a formatted colored paper for this specific purpose that
    is put inside of each quizmaster's questions + resources folder.</p>
  </li>
  <li>At the end of the quiz day, collect all of the sheets that the quizmasters filled out from each room.</li>
  <li>
    <p>Tally the stats manually, organizing the information how you see fit to give you the event's memory quizzer
    stats.</p>
    <p>You may wish to put this information into a program that allows you to format and print the results so that
    they can be posted somewhere for all to see, if you so desire.</p>
  </li>
</ol>

<p>At the end of the season, the monthly results can then be combined to find the
<a href="doc:stat-memory-verse-season">Top 3 Memory Verse Quizzers for the season</a>.</p>
`.trim(),
    },
    {
      slug: 'stat-memory-verse-season',
      title: 'Top 3 Memory Verse Quizzers (End-Of-Season)',
      contentHtml: `
<p><strong>District-Year-End, Per Division, Manually-Calculated Stats</strong></p>

<p>This is similar to the <a href="doc:stat-memory-verse-monthly">monthly event version</a>. The only difference
is that for the end-of-year you take the results from all of the past monthly district events and then tally
those along with the current quiz meet's memory verse stats to get the season-long stats for memory quizzers.</p>
`.trim(),
    },
    {
      slug: 'stat-most-prolific-jumper',
      title: 'Most Prolific Jumper (Speed and Quantity)',
      contentHtml: `
<p><strong>District-Year-End, Most Competitive Division Only, QuizMachine Stats</strong></p>

<p class="doc-note"><em>Fun fact:</em> In the MidSouth District this is called the "Jumpmaster" Award.</p>

<p class="doc-note">This is based on the total correct and erroneous toss-up answers. This means these stats /
awards really aren't based on how the quizzer answers at all.</p>

<ol>
  <li><a href="doc:creating-a-stats-group">Create a stats group</a> for a specific division's season-long
  stats.</li>
  <li>If you haven't already, import the quiz rounds you need into QMServer. You can do this
  <a href="doc:importing-quiz-round-data">by importing quizzes from a file</a> or
  <a href="doc:pulling-quiz-round-data">by transfer over WiFi/LAN</a>.</li>
  <li>Then in the newly created stats group, click <strong>Quizzes</strong> and select quiz rounds for the most
  competitive division for all of the district quiz meets that have happened this season.</li>
  <li>Export to CSV by left-clicking, then right-clicking <strong>Individual</strong>; then click <strong>Export
  Individual Stats</strong> in the mouse menu.</li>
  <li>Open the CSV file in Excel or Google Sheets.</li>
  <li>Create a new column with a formula that takes the values of columns <strong>Correct</strong> and
  <strong>Errors</strong>, adds them together and displays them in their own column for each row.</li>
  <li>Once you have this column, apply a filter to sort it so that the highest number of jumps is at the top of
  the list.</li>
  <li>The top row will show the quizzer with the most total jumps.</li>
<figure>
  <img src="/docs/qmserver-6.0.0/stat-most-prolific-jumper-1.webp" alt="Individual stats — Correct plus Errors gives total jumps." loading="lazy" />
  <figcaption>Individual stats — Correct plus Errors gives total jumps.</figcaption>
</figure>
  <li>
    <p>In a spreadsheet editor you can proceed with saving the spreadsheet as a PDF after formatting it the way
    you want to display it. Make sure to check the PDF preview contains all columns of data before saving.</p>
    <p class="doc-note">It is recommended that you save as a PDF before printing so that you are able to maintain
    a copy of the stats the way they were printed.</p>
  </li>
</ol>
`.trim(),
    },
    {
      slug: 'stat-highest-average-quizmachine',
      title: 'Highest Individual Average (QuizMachine Method)',
      contentHtml: `
<p><strong>District-Year-End, Most Competitive Division Only, QuizMachine Stats</strong></p>

<p class="doc-note"><em>Fun fact:</em> In the MidSouth District this is called the "Top Gun" Award.</p>

<ol>
  <li><a href="doc:creating-a-stats-group">Create a stats group</a> for a specific division's season-long
  stats.</li>
  <li>If you haven't already, import the quiz rounds you need into QMServer. You can do this
  <a href="doc:importing-quiz-round-data">by importing quizzes from a file</a> or
  <a href="doc:pulling-quiz-round-data">by transfer over WiFi/LAN</a>.</li>
  <li>Then in the newly created stats group, click <strong>Quizzes</strong> and select quiz rounds for the most
  competitive division for all of the district quiz meets that have happened this season.</li>
  <li>Export to CSV by left-clicking, then right-clicking <strong>Individual</strong>; then click <strong>Export
  Individual Stats</strong> in the mouse menu.</li>
  <li>Open the CSV file in Excel or Google Sheets.</li>
  <li>Apply a filter to the <strong>Avg</strong> column so that the highest numbers are listed toward the
  top.</li>
  <li>The top rows are the quizzers with the highest individual average.</li>
  <li>
    <p>In a spreadsheet editor you can proceed with saving the spreadsheet as a PDF after formatting it the way
    you want to display it. Make sure to check the PDF preview contains all columns of data before saving.</p>
    <p class="doc-note">It is recommended that you save as a PDF before printing so that you are able to maintain
    a copy of the stats the way they were printed.</p>
  </li>
</ol>

<h2>See also</h2>
<ul>
  <li><a href="doc:stat-highest-average-manual">Highest Individual Average, Manual Method</a></li>
  <li><a href="doc:stats-fewer-rounds">How averages handle quizzers who quizzed fewer rounds</a></li>
<figure>
  <img src="/docs/qmserver-6.0.0/stat-highest-average-quizmachine-1.webp" alt="Individual stats sorted by the Avg column." loading="lazy" />
  <figcaption>Individual stats sorted by the Avg column.</figcaption>
</figure>
</ul>
`.trim(),
    },
    {
      slug: 'stat-highest-average-manual',
      title: 'Highest Individual Average (Manual Method)',
      contentHtml: `
<p><strong>District-Year-End, Most Competitive Division Only, QuizMachine Stats</strong></p>

<p class="doc-note"><em>Fun fact:</em> In the MidSouth District this is called the "Top Gun" Award.</p>

<p>This is a manual process. Each quizzer's scores from the monthly quizzes are put into a spreadsheet and
totaled. This is used to determine which quizzer had the highest average score per round for the year.</p>
<figure>
  <img src="/docs/qmserver-6.0.0/stat-highest-average-manual-1.webp" alt="Individual stats — the Avg column for the season." loading="lazy" />
  <figcaption>Individual stats — the Avg column for the season.</figcaption>
</figure>

<ol>
  <li><a href="doc:creating-a-stats-group">Create a stats group</a> for a specific division's season-long
  stats.</li>
  <li>If you haven't already, import the quiz rounds you need into QMServer. You can do this
  <a href="doc:importing-quiz-round-data">by importing quizzes from a file</a> or
  <a href="doc:pulling-quiz-round-data">by transfer over WiFi/LAN</a>.</li>
  <li>Then in the newly created stats group, click <strong>Quizzes</strong> and select quiz rounds for the most
  competitive division for all of the district quiz meets that have happened this season.</li>
  <li>Export to CSV by left-clicking, then right-clicking <strong>Individual</strong>; then click <strong>Export
  Individual Stats</strong> in the mouse menu.</li>
  <li>Open the CSV file in Excel or Google Sheets.</li>
  <li>Apply a filter to the <strong>Avg</strong> column so that the highest numbers are listed toward the
  top.</li>
  <li>The top rows are the quizzers with the highest individual average.</li>
  <li>
    <p>In a spreadsheet editor you can proceed with saving the spreadsheet as a PDF after formatting it the way
    you want to display it. Make sure to check the PDF preview contains all columns of data before saving.</p>
    <p class="doc-note">It is recommended that you save as a PDF before printing so that you are able to maintain
    a copy of the stats the way they were printed.</p>
  </li>
</ol>

<h2>See also</h2>
<ul>
  <li><a href="doc:stat-highest-average-quizmachine">Highest Individual Average, QuizMachine Method</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'stat-memorized-all-verses',
      title: 'Awarding Quizzers Who Memorized All Memory Verses',
      contentHtml: `
<p><strong>District-Year-End, Per Division, Manually-Calculated Stats</strong></p>

<p class="doc-note">You may want to consider recording each quizzer's memory verse recitation for verification
that they have memorized all memory verses. It may also be good to ensure that the quizzer reciting is not reading
off of anything that is hidden off-camera. Plus, there may be relatives — or even the quizzer themselves — who may
like having the recording of the memory verse recitation session to keep as a memory, or as proof of what they
accomplished years ago as a quizzer.</p>

<h2>Two options for this award</h2>
<ul>
  <li><strong>Option 1:</strong> Have quiz program leaders at each church take time and have each memory quizzer
  who has memorized all memory verses this year in their program recite all memory verses. Then have those leaders
  report the results to district leadership.</li>
  <li><strong>Option 2:</strong> Do the same as Option 1, except at the current quiz event. This option offers the
  ability to be there in person to verify the results.</li>
</ul>
`.trim(),
    },
    {
      slug: 'format-names-schedule-file',
      title: 'Format: Names/Schedule File',
      contentHtml: `
<p>The schedule file's format is <strong>.csv</strong> (Comma Separated Value) and can be edited using Microsoft
Excel or Google Sheets.</p>

<p class="doc-warning">If you do edit the file, make sure that you <u>save the file in .csv format</u> —
otherwise, you may not be able to import the file into QuizMachine.</p>

<p>The following formatting is based on what QuizMachine (versions 4 &amp; 5) will accept as input.</p>

<h2>Section 1: Names (Teams and Quizzers)</h2>
<table>
  <tr><th>Column</th><th>Contents</th></tr>
  <tr><td>A</td><td>Name of the team that the Quizzer is on</td></tr>
  <tr><td>B</td><td>First and Last name of the Quizzer</td></tr>
</table>
<ul>
  <li>Both names will be shown exactly as entered in the QuizMachine interface.</li>
  <li>If any quizzer has a different team name then they will not be shown as a quizzer for that team.</li>
</ul>

<h2>Separation</h2>
<p>Three or four blank rows of separation.</p>

<h2>Section 2: Quiz Schedule (all rooms and rounds)</h2>
<table>
  <tr><th>Column</th><th>Contents</th></tr>
  <tr><td>A</td><td>Date and Time</td></tr>
  <tr><td>B</td><td>Tournament</td></tr>
  <tr><td>C</td><td>Division</td></tr>
  <tr><td>D</td><td>Room</td></tr>
  <tr><td>E</td><td>Round</td></tr>
  <tr><td>F</td><td>Red Seats Team Name (Team 1)</td></tr>
  <tr><td>G</td><td>Blue/Yellow Seats Team Name (Team 2) — only used for 3-team quizzing; leave blank if it is a two-team quiz round</td></tr>
  <tr><td>H</td><td>Green Seats Team Name (Team 3)</td></tr>
  <tr><td>I</td><td>Leave blank</td></tr>
  <tr><td>J</td><td>Quiz Division again (this must match the entry in Column C)</td></tr>
</table>

<p class="doc-warning"><strong>Important:</strong> Enter a single <strong>dummy round</strong> as the first entry
for each quiz room. While this is a requirement of QuizMachine for successfully preparing the first quiz round,
this dummy round can also serve as an impromptu practice round of sorts for that room before the first quiz round
begins (something that happens more often at larger quiz meets).</p>
<figure>
  <img src="/docs/qmserver-6.0.0/format-names-schedule-file-1.webp" alt="Schedule File Example — the names section (team and quizzer per row)." loading="lazy" />
  <figcaption>Schedule File Example — the names section (team and quizzer per row).</figcaption>
</figure>
<figure>
  <img src="/docs/qmserver-6.0.0/format-names-schedule-file-2.webp" alt="Schedule File Example — the schedule section, one row per room and round." loading="lazy" />
  <figcaption>Schedule File Example — the schedule section, one row per room and round.</figcaption>
</figure>
<figure>
  <img src="/docs/qmserver-6.0.0/format-names-schedule-file-3.webp" alt="Schedule File Example — the schedule section continued." loading="lazy" />
  <figcaption>Schedule File Example — the schedule section continued.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:creating-names-schedule-file">Creating/Getting a Names/Schedule File</a></li>
  <li><a href="doc:quizmachine/importing-names-and-schedule">Loading it into QuizMachine</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'format-quizzes-file',
      title: 'Format: Quizzes File (Quiz Round Data File)',
      contentHtml: `
<p>This is the format that QuizMachine (client) will export quiz round data in.</p>

<h2>Columns</h2>
<table>
  <tr><th>Column</th><th>Contents</th></tr>
  <tr><td>A</td><td>Key</td></tr>
  <tr><td>B</td><td>Tournament</td></tr>
  <tr><td>C</td><td>Division</td></tr>
  <tr><td>D</td><td>Room Number</td></tr>
  <tr><td>E</td><td>Round Number</td></tr>
  <tr><td>F</td><td>Question Number</td></tr>
  <tr><td>G</td><td>Sub-question record identifier (for multiple rows per specific question)</td></tr>
  <tr><td>H</td><td>Names</td></tr>
  <tr><td>I</td><td>Team Number — Red=0, Blue/Yellow=1, Green=2</td></tr>
  <tr><td>J</td><td>Seat Number — based on entered quizzer name prior to question 1</td></tr>
  <tr><td>K</td><td>Record Descriptor (2-letter identifier; see below)</td></tr>
  <tr><td>L</td><td>NULL — not used</td></tr>
  <tr><td>M</td><td>NULL — not used</td></tr>
  <tr><td>N</td><td>DateTime of when the record was entered</td></tr>
</table>

<h2>Record descriptors (column K)</h2>
<table>
  <tr><th>Code</th><th>Meaning</th></tr>
  <tr><td>RM</td><td>Room setup data</td></tr>
  <tr><td>QT</td><td>Quiz Type (almost always "N" for Nazarene)</td></tr>
  <tr><td>TN</td><td>Team Name</td></tr>
  <tr><td>QN</td><td>Quizzer Name (overwrites quizzer name for that seat on the team)</td></tr>
  <tr><td>SS</td><td>Team Captain or Co-captain</td></tr>
  <tr><td>SC</td><td>Team Captain or Co-captain</td></tr>
  <tr><td>TC</td><td>Toss-up, Correct</td></tr>
  <tr><td>TE</td><td>Toss-up, Error</td></tr>
  <tr><td>BC</td><td>Bonus, Correct</td></tr>
  <tr><td>BE</td><td>Bonus, Error</td></tr>
  <tr><td>QO</td><td>Quiz Out</td></tr>
  <tr><td>EO</td><td>Error Out</td></tr>
  <tr><td>SB</td><td>Substitution</td></tr>
  <tr><td>TO</td><td>TimeOut</td></tr>
  <tr><td>DE</td><td>Data Entry (generic)</td></tr>
  <tr><td>A+</td><td>Appeal, Accepted</td></tr>
  <tr><td>C−</td><td>Challenge, Overruled</td></tr>
  <tr><td>F−</td><td>Foul</td></tr>
</table>
<figure>
  <img src="/docs/qmserver-6.0.0/format-quizzes-file-1.webp" alt="Example of a single exported quiz round (2-team matchup)." loading="lazy" />
  <figcaption>Example of a single exported quiz round (2-team matchup).</figcaption>
</figure>
<figure>
  <img src="/docs/qmserver-6.0.0/format-quizzes-file-2.webp" alt="The same exported round, continued." loading="lazy" />
  <figcaption>The same exported round, continued.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:quizmachine/exporting-quiz-round-data">Exporting quiz round data from QuizMachine</a></li>
  <li><a href="doc:importing-quiz-round-data">Importing quiz round data into QMServer</a></li>
</ul>
`.trim(),
    },
  ],
}
