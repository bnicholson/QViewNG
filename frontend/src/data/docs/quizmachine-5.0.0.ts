// QuizMachine 5.0.0 — content from the "QuizMachine Documentation v.5.0.0" user guide
// (April 2, 2025). Sections the manual performs *inside QMServer* live in the QMServer category.
//
// Differences from 6.0.0: no "New With Version 6"; QBox setup lives inside Hardware rather than its
// own page; the shortcut page covers a Desktop shortcut rather than Start Menu/Taskbar; and there is
// no Tournament-Defaults-vs-Modified, Practice (random bonus / ghost team) or Properties/Features page.
//
// The manual's "(link)" bookmarks are preserved as `doc:` links — see resolveDocHref in ./index.

import type { DocVersion } from './types'

export const quizmachine500: DocVersion = {
  version: '5.0.0',
  released: 'April 2, 2025',
  nav: [
    {
      title: 'Before Using QuizMachine',
      children: [
        { title: 'Compatibility', slug: 'compatibility' },
        { title: 'Installing QuizMachine', slug: 'installing-quizmachine' },
        {
          title: 'Launching QuizMachine',
          children: [
            { title: 'Launching QuizMachine', slug: 'launching-quizmachine' },
            { title: 'Navigating In and Out Without Closing', slug: 'navigating-in-and-out' },
          ],
        },
        { title: 'Registering QuizMachine', slug: 'registering-quizmachine' },
        { title: 'Creating a Desktop Shortcut', slug: 'shortcuts' },
      ],
    },
    {
      title: 'General Use',
      children: [
        { title: 'Scoring Interface (Home)', slug: 'scoring-interface' },
        { title: 'Hot Keys & Shortcuts', slug: 'hot-keys-and-shortcuts' },
        { title: 'What NOT to Do in QuizMachine', slug: 'what-not-to-do' },
        { title: 'Demo Mode', slug: 'demo-mode' },
        { title: 'Hardware (incl. QBox)', slug: 'hardware' },
      ],
    },
    {
      title: 'Quiz Round Preparation',
      children: [
        { title: 'Entering Lineups (Teams & Quizzers)', slug: 'entering-lineups' },
        {
          title: 'Options for Quiz Round',
          children: [
            { title: 'Options for Quiz Round', slug: 'options-for-quiz-round' },
            { title: 'Changing Correct Answers to Quiz Out', slug: 'option-quiz-out' },
            { title: 'Changing Incorrect Answers to Error Out', slug: 'option-error-out' },
            { title: 'Changing Fouls Issued to Foul Out', slug: 'option-foul-out' },
            { title: 'Team Error Deductions', slug: 'option-team-error-deduction' },
            { title: 'Individual Error Deductions', slug: 'option-individual-error-deduction' },
            { title: 'Toss-Up Clock Start Delay', slug: 'option-tossup-clock-delay' },
            { title: 'Bonus Clock Start Delay', slug: 'option-bonus-clock-delay' },
          ],
        },
      ],
    },
    {
      title: 'QuizMastering a Quiz Round',
      children: [
        {
          title: 'QuizMastering Workflow Basics',
          children: [
            { title: 'QuizMastering Workflow Basics', slug: 'quizmastering-workflow-basics' },
            { title: 'Question Types', slug: 'question-types' },
          ],
        },
        {
          title: 'Quizzer Scoring Window',
          children: [
            { title: 'Quizzer Scoring Window', slug: 'quizzer-scoring-window' },
            { title: 'Multipling', slug: 'multipling' },
          ],
        },
        { title: 'Working with the Timer', slug: 'working-with-the-timer' },
        {
          title: 'Scoresheet',
          children: [
            { title: 'Scoresheet', slug: 'scoresheet' },
            { title: 'Viewing the Scoresheet', slug: 'viewing-the-scoresheet' },
            { title: 'Editing Past Questions (Making Corrections)', slug: 'editing-the-scoresheet' },
            { title: 'Printing the Scoresheet', slug: 'printing-the-scoresheet' },
          ],
        },
        { title: 'Timeouts', slug: 'timeouts' },
        { title: 'Substituting a Quizzer', slug: 'substituting-a-quizzer' },
        { title: 'Challenging a Ruling', slug: 'challenging-a-ruling' },
        { title: 'Appealing a Question', slug: 'appealing-a-question' },
        { title: 'Issuing Fouls', slug: 'issuing-fouls' },
        { title: 'Using the Audio Playback Feature', slug: 'audio-playback' },
      ],
    },
    {
      title: 'Working with Data',
      children: [
        { title: 'Connecting QuizMachine to QMServer', slug: 'connecting-to-qmserver' },
        { title: 'Sending Messages to QMServer', slug: 'sending-messages-to-qmserver' },
        { title: 'Getting Team & Quizzer Names from QMServer', slug: 'getting-names-from-qmserver' },
        { title: 'Importing/Loading Names & Schedule Data', slug: 'importing-names-and-schedule' },
        { title: 'Setting Up Rooms Without a Names/Schedule File', slug: 'rooms-without-schedule-file' },
        { title: 'Exporting Quiz Round Data to a File', slug: 'exporting-quiz-round-data' },
      ],
    },
  ],
  articles: [
    {
      slug: 'compatibility',
      title: 'Compatibility',
      contentHtml: `
<p>To have a great time with QuizMachine and avoid technical issues it is important to install the
right version of QuizMachine for your version of computer OS. This section will help you with
choosing the right version of QuizMachine to install.</p>

<p>QuizMachine version 5 was released in December 2023. As of March 2024 QuizMachine version 5 is the
latest version available on QuizStuff. It is available for download at
<a href="https://quizstuff.com" target="_blank" rel="noopener noreferrer">QuizStuff.com</a>. You can get
a free 30-day trial download, but to continue use after this time a registration code is required. You
can purchase a QuizMachine registration code at QuizStuff.com, or your Quiz Director may have a unique
copy from a District Director's multiple purchased copies.</p>

<p>If you have any questions, feel free to share them with us at
<a href="mailto:quizstuff@quizstuff.com">quizstuff@quizstuff.com</a>.</p>

<h2>Current available versions</h2>
<ul>
  <li>Version 5.4.J30 (online download)</li>
  <li>Version 4.2.L55 (online download)</li>
</ul>

<h2>Interface box compatibility</h2>
<ul>
  <li><strong>QBox (USB Interface Box)</strong> — Version 4 &amp; 5 only</li>
  <li><strong>Parallel Box</strong> — all versions except for version 5</li>
</ul>

<h2>Operating system compatibility</h2>
<table>
  <tr><th>Operating system</th><th>QuizMachine version</th></tr>
  <tr><td>Any OS other than Windows</td><td>Not compatible</td></tr>
  <tr><td>Windows 11</td><td>version 5 (5.4.J30 or newer, x86) — recommended; could also use version 6 (6.2.1-U21 or newer, x64)</td></tr>
  <tr><td>Windows 10</td><td>version 5 (5.4.J30 or newer, x86) — recommended; could also use version 6 (6.2.1-U21 or newer, x64)</td></tr>
  <tr><td>Windows 7</td><td>version 5 (5.4.J30 or newer, x86)</td></tr>
  <tr><td>Windows 8.1</td><td>version 5 (5.4.J30 or newer, x86); Windows 7 is recommended over Windows 8.1</td></tr>
  <tr><td>Windows Y2K</td><td>version 4 (4.2.L55)</td></tr>
  <tr><td>Windows XP</td><td>version 4 (4.2.L55)</td></tr>
  <tr><td>Windows Vista</td><td>version 4 (4.2.L55)</td></tr>
  <tr><td>Windows 95</td><td>version 1 (1.0.a44; not compatible with QBox; may not be available)</td></tr>
</table>

<h2>See also</h2>
<ul>
  <li><a href="doc:installing-quizmachine">Installing QuizMachine</a></li>
  <li><a href="doc:registering-quizmachine">Registering QuizMachine</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'installing-quizmachine',
      title: 'Installing QuizMachine',
      contentHtml: `
<ol>
  <li>
    <p>Download the right QuizMachine program for your computer from
    <a href="https://quizstuff.com" target="_blank" rel="noopener noreferrer">QuizStuff.com</a>.</p>
    <p class="doc-note"><strong>Note on compatibility:</strong> It is important that you install the
    version of QuizMachine that is compatible with your computer's operating system (OS) or you may
    experience undesirable results and/or bugs. For information on which version of QuizMachine is
    right for you, see <a href="doc:compatibility">Compatibility</a>.</p>
  </li>
  <li>Once downloaded, run the install program by clicking the <strong>QuizMachineInstaller.exe</strong>
  file. Then review the QuizMachine Installation Options screen.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/installing-quizmachine-1.webp" alt="The QuizMachine Installation Options screen." loading="lazy" />
  <figcaption>The QuizMachine Installation Options screen.</figcaption>
</figure>
  <li>Leave the selected component selected. Then click <strong>Next</strong> to continue.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/installing-quizmachine-2.webp" alt="The component selection — leave it as it is and click Next." loading="lazy" />
  <figcaption>The component selection — leave it as it is and click Next.</figcaption>
</figure>
  <li>
    <p>Choose the program installation folder. The default location is recommended. Make sure that
    your computer's internal hard drive has enough space to install the QuizMachine program.</p>
    <p>If the amount of space available is less than the amount of space required for installation,
    click <strong>Cancel</strong>, remove enough data and/or programs to have the required space
    available, and then relaunch the installer (restart installation from step 2 above).</p>
  </li>
<figure>
  <img src="/docs/quizmachine-5.0.0/installing-quizmachine-3.webp" alt="Choosing the installation folder, with space required and space available." loading="lazy" />
  <figcaption>Choosing the installation folder, with space required and space available.</figcaption>
</figure>
  <li>Once you have confirmed the installation location is where you want it to be installed, click
  <strong>Install</strong>. You will see a progress bar during the installation.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/installing-quizmachine-4.webp" alt="The installation progress bar." loading="lazy" />
  <figcaption>The installation progress bar.</figcaption>
</figure>
  <li>If the installation was successful, click the <strong>OK</strong> button to continue. Then click
  <strong>Close</strong> to exit the installer.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/installing-quizmachine-5.webp" alt="The success message, then Close to exit the installer." loading="lazy" />
  <figcaption>The success message, then Close to exit the installer.</figcaption>
</figure>
</ol>

<p>Your QuizMachine program is now ready to run! If you receive an error message or are having trouble
with the installation, please contact QuizStuff by email via
<a href="mailto:quizstuff@quizstuff.com">quizstuff@quizstuff.com</a>.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:compatibility">Compatibility</a></li>
  <li><a href="doc:launching-quizmachine">Launching QuizMachine</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'launching-quizmachine',
      title: 'Launching QuizMachine',
      contentHtml: `
<p>To start the QuizMachine program, <strong>double click the icon</strong> on your desktop.</p>

<p>If you do not have a QuizMachine shortcut icon on your desktop, see
<a href="doc:shortcuts">Creating a Desktop Shortcut for Launching QuizMachine</a> on how to create one
for easy access.</p>

<p>You will see the QuizMachine welcome screen, an audio tone will play, and you will then see the
scoring (home) interface. You may also be presented with the window for registering your QuizMachine
program.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:navigating-in-and-out">Navigating in and out of QuizMachine without closing the program</a></li>
  <li><a href="doc:registering-quizmachine">Registering QuizMachine</a></li>
  <li><a href="doc:scoring-interface">Scoring Interface (Home)</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'navigating-in-and-out',
      title: 'Navigating In and Out of QuizMachine Without Closing the Program',
      contentHtml: `
<p>While QuizMachine is open and running, it will automatically enter fullscreen mode. There are
<strong>two ways</strong> that you can navigate out of QuizMachine to a different program on your
computer:</p>

<ol>
  <li>By clicking <strong>Setup/Exit</strong>, then clicking <strong>Exit</strong> and then clicking
  <strong>Yes</strong> to confirm — this closes QuizMachine.</li>
  <li>Or, if you want to keep QuizMachine running, by pressing the shortcut <strong>Alt + Tab</strong>
  and then releasing both. You can also navigate back into QuizMachine by the same means, or by
  clicking on the program icon that shows in the Windows taskbar at the bottom of your screen.</li>
</ol>

<h2>See also</h2>
<ul>
  <li><a href="doc:launching-quizmachine">Launching QuizMachine</a></li>
  <li><a href="doc:hot-keys-and-shortcuts">Hot Keys &amp; Shortcuts</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'registering-quizmachine',
      title: 'Registering QuizMachine',
      contentHtml: `
<p>These instructions serve as supplements for the instructions that were included with your
registration code that is delivered by email.</p>

<p class="doc-warning"><strong>Please read the instructions here in their entirety before getting
started!</strong></p>

<h2>About the 30-day trial period</h2>
<p>The first time the program is executed (or each time during the 30-day trial period) a registration
code entry screen will appear. If you plan on using the 30-day trial version, click the
<strong>OK</strong> button. If you choose to utilize the 30-day trial period, once it expires the
program performs a "sanity check" and will cease to work. A registration code will need to be entered
to continue using the program. Once expired, QuizMachine will allow entry of question number 1 but
will fail at question number 2.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/registering-quizmachine-1.webp" alt="The registration code entry screen shown during the trial period." loading="lazy" />
  <figcaption>The registration code entry screen shown during the trial period.</figcaption>
</figure>

<h2>Registering QuizMachine using your registration code</h2>
<p>You will need a QuizMachine registration code for registration. If you don't have one you can
purchase it by placing an order at
<a href="https://quizstuff.com" target="_blank" rel="noopener noreferrer">QuizStuff.com</a>. After
purchase, your registration code will be sent to the email address you entered for the purchase.</p>

<p class="doc-note"><strong>Important to note about the registration code:</strong></p>
<ul>
  <li>The registration code cannot be opened with any other program except QuizMachine.</li>
  <li>Only one QuizMachine program can be registered at a time per registration code.</li>
</ul>

<p>You will also need to have the latest compatible version of QuizMachine installed. If you have not
yet installed QuizMachine, see
<a href="doc:installing-quizmachine">how to download and install the latest version of QuizMachine</a>.</p>

<h2>Steps for registering</h2>
<ol>
  <li>Download the code file attached to the email you received. The file has a name similar to
  <code>qmkey{your_name}.qm</code>. Save the file to a folder on your computer whose location you can
  easily remember for access later on.</li>
  <li>Start QuizMachine. (For more information see
  <a href="doc:launching-quizmachine">launching QuizMachine</a>.)</li>
  <li>Click the <strong>Setup/Exit</strong> button (lower left-hand corner).</li>
  <li>Click <strong>Yes</strong>. You should see the General tab of the Setup window.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/registering-quizmachine-2.webp" alt="The General tab of the Setup window, with Register/Change Registration." loading="lazy" />
  <figcaption>The General tab of the Setup window, with Register/Change Registration.</figcaption>
</figure>
  <li>Click <strong>Register/Change Registration</strong> (or, if this is the first time you have run
  QuizMachine, the window may have already opened).</li>
  <li>Click <strong>Load License File from QuizStuff.com</strong>.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/registering-quizmachine-3.webp" alt="The registration window — Load License File from QuizStuff.com." loading="lazy" />
  <figcaption>The registration window — Load License File from QuizStuff.com.</figcaption>
</figure>
  <li>In the file explorer window, select the registration code file you just saved. Then click
  <strong>Open</strong>.</li>
  <li>You should see the message, "Thank you for choosing QuizMachine".</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/registering-quizmachine-4.webp" alt="“Thank you for choosing QuizMachine” confirms registration." loading="lazy" />
  <figcaption>“Thank you for choosing QuizMachine” confirms registration.</figcaption>
</figure>
  <li>Click <strong>OK</strong>.</li>
  <li>Click <strong>OK</strong> again. You should be all set.</li>
</ol>

<p class="doc-note">You are responsible for maintaining a copy of each registration code and which
computer is registered with it.</p>
`.trim(),
    },
    {
      slug: 'shortcuts',
      title: 'Creating a Desktop Shortcut for Launching QuizMachine',
      contentHtml: `
<p>If you do not have a QuizMachine shortcut icon on your desktop, you can create one:</p>

<ol>
  <li>Open File Explorer and go to the file location
  <strong>This PC &gt; Local Disk (C:) &gt; Program Files (x86) &gt; QuizStuff &gt;
  QuizMachine{version#}</strong>.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/shortcuts-1.webp" alt="Navigating to Program Files (x86) in File Explorer." loading="lazy" />
  <figcaption>Navigating to Program Files (x86) in File Explorer.</figcaption>
</figure>
<figure>
  <img src="/docs/quizmachine-5.0.0/shortcuts-2.webp" alt="The QuizStuff folder, containing the QuizMachine{version#} folder." loading="lazy" />
  <figcaption>The QuizStuff folder, containing the QuizMachine{version#} folder.</figcaption>
</figure>
  <li>Find the <strong>quizmachine.exe</strong> file (it may appear as "quizmachine" without the
  <code>.exe</code> file extension).</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/shortcuts-3.webp" alt="The quizmachine.exe file inside the version folder." loading="lazy" />
  <figcaption>The quizmachine.exe file inside the version folder.</figcaption>
</figure>
  <li><strong>Left click</strong> the file to select it, then <strong>right click</strong> the file to
  reveal the mouse menu options.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/shortcuts-4.webp" alt="The mouse menu for the selected file." loading="lazy" />
  <figcaption>The mouse menu for the selected file.</figcaption>
</figure>
  <li>Then click <strong>Create Shortcut</strong>. You may be prompted to create the shortcut by
  placing it on your Desktop — click <strong>Yes</strong> to create it. You should then have a Desktop
  shortcut on your Desktop screen that you can launch QuizMachine from.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/shortcuts-5.webp" alt="The “Create Shortcut” prompt offering to place it on the Desktop." loading="lazy" />
  <figcaption>The “Create Shortcut” prompt offering to place it on the Desktop.</figcaption>
</figure>
<figure>
  <img src="/docs/quizmachine-5.0.0/shortcuts-6.webp" alt="The finished QuizMachine shortcut on the Desktop." loading="lazy" />
  <figcaption>The finished QuizMachine shortcut on the Desktop.</figcaption>
</figure>
</ol>

<p>Alternatively, you can also add this to your Taskbar, your Home dock/menu, or your File Explorer's
Quick Start menu as well.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/shortcuts-7.webp" alt="Pinning QuizMachine to the Taskbar." loading="lazy" />
  <figcaption>Pinning QuizMachine to the Taskbar.</figcaption>
</figure>
<figure>
  <img src="/docs/quizmachine-5.0.0/shortcuts-8.webp" alt="QuizMachine pinned to the Start menu." loading="lazy" />
  <figcaption>QuizMachine pinned to the Start menu.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:qmserver/shortcuts">Creating a Desktop Shortcut for Launching QMServer</a></li>
  <li><a href="doc:launching-quizmachine">Launching QuizMachine</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'scoring-interface',
      title: 'Scoring Interface (Home)',
      contentHtml: `
<p>The scoring interface (home) is the scoreboard that QuizMachine shows throughout a quiz round.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/scoring-interface-1.webp" alt="The QuizMachine scoring interface (home)." loading="lazy" />
  <figcaption>The QuizMachine scoring interface (home).</figcaption>
</figure>

<h2>Quiz round information</h2>
<h3>Question number</h3>
<ul>
  <li>Starts at Question 1 even before starting the quiz round.</li>
  <li>Increases by 1 with each completed question.</li>
  <li>After question 20 is answered, it will display <strong>DONE</strong>, indicating the quiz round
  has been completed.</li>
</ul>
<figure>
  <img src="/docs/quizmachine-5.0.0/scoring-interface-2.webp" alt="The question number, which shows DONE after question 20." loading="lazy" />
  <figcaption>The question number, which shows DONE after question 20.</figcaption>
</figure>

<h3>General quiz round information</h3>
<ul>
  <li>Tournament Title</li>
  <li>Division</li>
  <li>Room Number &amp; Round Number</li>
  <li>QuizMachine Rules/Method/Mode (Tournament or Practice)</li>
</ul>
<figure>
  <img src="/docs/quizmachine-5.0.0/scoring-interface-3.webp" alt="General quiz round information: tournament, division, room and round." loading="lazy" />
  <figcaption>General quiz round information: tournament, division, room and round.</figcaption>
</figure>

<h3>Timer</h3>
<p>Used for various parts of the quiz round such as toss-up question answers, bonus question answers,
no-jumps on toss-up questions, timeouts, challenges, rebuttals and appeals.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/scoring-interface-4.webp" alt="The timer." loading="lazy" />
  <figcaption>The timer.</figcaption>
</figure>

<h2>Team information</h2>
<ul>
  <li>Team Names</li>
  <li>Team Scores</li>
</ul>
<figure>
  <img src="/docs/quizmachine-5.0.0/scoring-interface-5.webp" alt="Team names and scores." loading="lazy" />
  <figcaption>Team names and scores.</figcaption>
</figure>

<h2>Quizzer information</h2>
<p>For each quizzer:</p>
<ul>
  <li><strong>Quizzer Light</strong>
    <p class="doc-note">As a QuizMaster, you can click a quizzer's light to open the toss-up ruling
    window (as an additional option to a quizzer jumping on a toss-up question).</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/scoring-interface-6.webp" alt="A quizzer's light — clicking it opens the toss-up ruling window." loading="lazy" />
  <figcaption>A quizzer's light — clicking it opens the toss-up ruling window.</figcaption>
</figure>
  </li>
  <li><strong>Quizzer Name</strong></li>
  <li><strong>Quizzer Correct/Error count</strong> (for toss-ups only, not for bonuses)</li>
</ul>
<figure>
  <img src="/docs/quizmachine-5.0.0/scoring-interface-7.webp" alt="Quizzer names with their correct/error counts." loading="lazy" />
  <figcaption>Quizzer names with their correct/error counts.</figcaption>
</figure>

<h2>QuizMaster menu / navigation bar</h2>
<ul>
  <li>Setup/Exit windows</li>
  <li>Lineups window</li>
  <li>Time Out window</li>
  <li>Reset Timer button</li>
  <li>Next Jump button</li>
  <li>No Jump button</li>
  <li>Info button (*takes you to QuizStuff.com)</li>
  <li>Challenge / Appeal / Foul windows</li>
  <li>Scoresheet window</li>
</ul>
<figure>
  <img src="/docs/quizmachine-5.0.0/scoring-interface-8.webp" alt="The QuizMaster menu / navigation bar along the bottom." loading="lazy" />
  <figcaption>The QuizMaster menu / navigation bar along the bottom.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:hot-keys-and-shortcuts">Hot Keys &amp; Shortcuts</a></li>
  <li><a href="doc:quizmastering-workflow-basics">QuizMastering Workflow Basics</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'hot-keys-and-shortcuts',
      title: 'Hot Keys & Shortcuts',
      contentHtml: `
<p>QuizMachine comes with a number of keyboard shortcuts (hot keys) for operating and navigating
QuizMachine in a quicker and/or different way.</p>

<h2>From the Home interface</h2>
<table>
  <tr><th>Key</th><th>Action</th></tr>
  <tr><td>X</td><td>Opens the "Setup/Exit" window</td></tr>
  <tr><td>L</td><td>Opens the "Lineups" window</td></tr>
  <tr><td>T</td><td>Opens the "Timer" window</td></tr>
  <tr><td>R</td><td>Resets the Timer (*after jump on a toss-up)</td></tr>
  <tr><td>Spacebar</td><td>Engages the jump seats / starts the "no jump" timer</td></tr>
  <tr><td>C</td><td>Opens the "Challenge / Appeal / Foul" window</td></tr>
  <tr><td>A</td><td>Opens the "Challenge / Appeal / Foul" window</td></tr>
  <tr><td>F</td><td>Opens the "Challenge / Appeal / Foul" window</td></tr>
</table>
<figure>
  <img src="/docs/quizmachine-5.0.0/hot-keys-and-shortcuts-1.webp" alt="The Home interface, where these hot keys apply." loading="lazy" />
  <figcaption>The Home interface, where these hot keys apply.</figcaption>
</figure>

<h2>From the Toss-Up / Bonus window</h2>
<table>
  <tr><th>Key</th><th>Action</th></tr>
  <tr><td>C</td><td>Selects "Correct" and does not submit result</td></tr>
  <tr><td>E</td><td>Selects "Error" and does not submit result</td></tr>
  <tr><td>Enter</td><td>Submits selected ruling as result</td></tr>
  <tr><td>F</td><td>Opens the "Foul" window</td></tr>
  <tr><td>P</td><td>Opens the audio playback window for the question</td></tr>
</table>
<figure>
  <img src="/docs/quizmachine-5.0.0/hot-keys-and-shortcuts-2.webp" alt="The Toss-Up / Bonus ruling window." loading="lazy" />
  <figcaption>The Toss-Up / Bonus ruling window.</figcaption>
</figure>

<h2>From the "Foul" window</h2>
<p>(During the ruling of a toss-up or bonus, or from Challenge/Appeal/Foul &gt; Foul.)</p>
<table>
  <tr><th>Key</th><th>Action</th></tr>
  <tr><td>A</td><td>Applies the foul to the selected team or quizzer</td></tr>
</table>
<figure>
  <img src="/docs/quizmachine-5.0.0/hot-keys-and-shortcuts-3.webp" alt="The Foul window." loading="lazy" />
  <figcaption>The Foul window.</figcaption>
</figure>

<h2>From the "Setup/Exit" window</h2>
<table>
  <tr><th>Key</th><th>Action</th></tr>
  <tr><td>S</td><td>(v5) Opens the "Setup" window</td></tr>
  <tr><td>Y</td><td>(v4) Opens the "Setup" window</td></tr>
  <tr><td>N</td><td>(v4) Exits/quits the QuizMachine program</td></tr>
</table>
<figure>
  <img src="/docs/quizmachine-5.0.0/hot-keys-and-shortcuts-4.webp" alt="The Setup/Exit window." loading="lazy" />
  <figcaption>The Setup/Exit window.</figcaption>
</figure>

<h2>From the Exit confirmation window</h2>
<table>
  <tr><th>Key</th><th>Action</th></tr>
  <tr><td>Y</td><td>Yes</td></tr>
  <tr><td>N</td><td>No</td></tr>
</table>
<figure>
  <img src="/docs/quizmachine-5.0.0/hot-keys-and-shortcuts-5.webp" alt="The Exit confirmation window." loading="lazy" />
  <figcaption>The Exit confirmation window.</figcaption>
</figure>

<h2>From the "Timeout" window</h2>
<table>
  <tr><th>Key</th><th>Action</th></tr>
  <tr><td>S</td><td>Opens the Substitution window</td></tr>
  <tr><td>C</td><td>Opens the Scoresheet</td></tr>
  <tr><td>T</td><td>Resets the Timer</td></tr>
</table>
<figure>
  <img src="/docs/quizmachine-5.0.0/hot-keys-and-shortcuts-6.webp" alt="The Timeout window." loading="lazy" />
  <figcaption>The Timeout window.</figcaption>
</figure>

<h2>From the "Challenge" window</h2>
<p>(Challenge / Appeal / Foul &gt; Challenge, then select a team.)</p>
<table>
  <tr><th>Key</th><th>Action</th></tr>
  <tr><td>A</td><td>Accept the challenge</td></tr>
  <tr><td>O</td><td>Overrule the challenge</td></tr>
  <tr><td>C</td><td>Cancels the challenge</td></tr>
  <tr><td>P</td><td>Playback the audio recording for the challenge</td></tr>
  <tr><td>T</td><td>Add time back to the timer</td></tr>
  <tr><td>R</td><td>Reset the timer to zero</td></tr>
</table>
<figure>
  <img src="/docs/quizmachine-5.0.0/hot-keys-and-shortcuts-7.webp" alt="The Challenge window." loading="lazy" />
  <figcaption>The Challenge window.</figcaption>
</figure>

<h2>From the "Appeal" window</h2>
<p>(Challenge / Appeal / Foul &gt; Appeal.)</p>
<table>
  <tr><th>Key</th><th>Action</th></tr>
  <tr><td>A</td><td>Accept the appeal</td></tr>
  <tr><td>O</td><td>Overrule the appeal</td></tr>
  <tr><td>C</td><td>Cancels the appeal</td></tr>
</table>
<figure>
  <img src="/docs/quizmachine-5.0.0/hot-keys-and-shortcuts-8.webp" alt="The Appeal window." loading="lazy" />
  <figcaption>The Appeal window.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:scoring-interface">Scoring Interface (Home)</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'what-not-to-do',
      title: 'What NOT to Do in QuizMachine',
      contentHtml: `
<h2>(1 of 2) Do not click the "Change Resolution" button in Setup &gt; General</h2>
<p class="doc-warning">Do not click this button.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/what-not-to-do-1.webp" alt="The “Change Resolution” button in Setup › General — do not click it." loading="lazy" />
  <figcaption>The “Change Resolution” button in Setup › General — do not click it.</figcaption>
</figure>

<h2>(2 of 2) Do not click the "Check for Updates" button in Setup &gt; Updates</h2>
<p class="doc-note">This tab is included in QuizMachine version 4 but is <strong>not</strong> included
in QuizMachine version 5.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/what-not-to-do-2.webp" alt="The Updates tab, with “Check for Updates” — present in v4 only." loading="lazy" />
  <figcaption>The Updates tab, with “Check for Updates” — present in v4 only.</figcaption>
</figure>

<p><strong>DISABLED!</strong> The Software Update Information screen allows computers that have
lifetime upgrades to check in with the server for any new upgrades/updates/fixes. An internet
connection is required before clicking <strong>Check for updates Now</strong>.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/what-not-to-do-3.webp" alt="The Software Update Information screen." loading="lazy" />
  <figcaption>The Software Update Information screen.</figcaption>
</figure>

<p>This screen also displays the last update date and time, and the next scheduled update; this is
provided an internet connection is available. Click <strong>Cancel</strong> to return to the
QuizMachine Properties menu without making any changes. Click <strong>Apply</strong> to remain in the
Updates window. Click <strong>OK</strong> when finished to return to the QuizMachine Properties
window.</p>

<p>If the version does not require an update, a window saying so is displayed. Click
<strong>OK</strong> to return to the QuizMachine Properties window.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/what-not-to-do-4.webp" alt="The “no update required” message." loading="lazy" />
  <figcaption>The “no update required” message.</figcaption>
</figure>

<p>If updates are available, the QuizStuff server will display a download window with a progress
window indicating the status of the download and the update progress. When the update is complete,
the QuizMachine software will be updated to the newest release with all fixes applied.</p>
`.trim(),
    },
    {
      slug: 'demo-mode',
      title: 'Demo Mode',
      contentHtml: `
<p>QuizMachine comes with a mode that will demonstrate the scoreboard appearance as it goes through
the changes that will happen throughout multiple rounds (though it will not reset scores after
resetting Q20 to Q1 in transitions between rounds).</p>

<p>To enter into Demo Mode:</p>
<ol>
  <li>Go to <strong>Setup/Exit &gt; Setup &gt; General</strong>.</li>
  <li>Select the <strong>Demo Mode</strong> checkbox.</li>
  <li>Click <strong>Apply</strong>. Demo Mode will begin playing.</li>
  <li>Click <strong>OK</strong> to exit the Setup window to view the demo mode.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/demo-mode-1.webp" alt="The Demo Mode checkbox in Setup › General." loading="lazy" />
  <figcaption>The Demo Mode checkbox in Setup › General.</figcaption>
</figure>
</ol>
`.trim(),
    },
    {
      slug: 'hardware',
      title: 'Hardware (including QBox)',
      contentHtml: `
<p>Navigate to <strong>Setup &gt; Hardware</strong>.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/hardware-1.webp" alt="The Hardware tab of the QuizMachine Properties window." loading="lazy" />
  <figcaption>The Hardware tab of the QuizMachine Properties window.</figcaption>
</figure>

<h2>Interface Type</h2>
<ul>
  <li>The default selection is <strong>Lpt Interface</strong>. The LPT interface used to utilize
  connection via parallel port, PCMCIA (or Express) card and LPT box. These have since been
  deprecated; please use QBox instead. To purchase a QBox please visit
  <a href="https://quizstuff.com" target="_blank" rel="noopener noreferrer">QuizStuff.com</a>, or if
  you would like to convert your LPT box to a QBox please submit this request to
  <a href="mailto:quizstuff@quizstuff.com">quizstuff@quizstuff.com</a>.</li>
  <li>When using a QBox, the <strong>QuizStuff USB QBox</strong> option should be selected.
    <p>The QBox requires the installation of a driver on the computer that QuizMachine is installed
    on. To start the installation of the driver, first launch QuizMachine and then connect the QBox to
    the computer. A Windows operating system message will appear indicating the driver is loading.
    Once it is installed, QBox can be used with QuizMachine. A message will appear whenever the
    connection status of QBox changes between being connected or disconnected.</p>
  </li>
</ul>

<h2>Diagnose Interface Box button</h2>
<p>This button is for use with QBox and not with LPT boxes. This feature has fallen out of use in
recent years due to improvement in QBox technology over time. It was originally added to check the
connection performance between the QBox and the QuizMachine program. This button can be
disregarded.</p>

<h2>Seat Type</h2>
<p>Identifies which type of interface box is present. The technical difference between these two types
is that one is for normally-open jump seats and the other is for normally-closed jump seats.</p>
<ul>
  <li><strong>Type 1</strong> <em>was</em> for use with QuizTime seats (makers of the "Red Box"), which
  is an older brand and NOT sold by QuizStuff.com. You will likely never need to use Type 1.</li>
  <li><strong>Type 2 (default)</strong> is for seats sold by QuizStuff.com. If you are using QBox, you
  will need this to be the setting for connecting to QBox.</li>
</ul>

<h2>Drop/Add Seats</h2>
<p>This option allows a specific seat (or seats) to be dropped. This could be used when there is a bad
pad in a string of otherwise good jump pads, or if a handicapped person needs to use seat 5 on a
specific team. More than one seat may be dropped for a team or across teams.</p>
<ol>
  <li>Click the number of the seat(s) to be dropped.</li>
  <li>Click <strong>OK</strong> to apply the changes and return to the QuizMachine Properties window.</li>
</ol>
<p class="doc-note">Please remember that the pad must then be removed from use. The default is all
boxes checked.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/hardware-2.webp" alt="Drop/Add Seats — click a seat number to drop it from use." loading="lazy" />
  <figcaption>Drop/Add Seats — click a seat number to drop it from use.</figcaption>
</figure>

<h2>Test Seats checkbox</h2>
<p>Mark the check box to have the name of the seat spoken. When the pad for a given seat is pressed,
QuizMachine will speak the name of the seat ("Red One"). This only verifies that a seat is working.</p>
<p class="doc-warning">Make sure to uncheck this box prior to using QuizMachine for a tournament or
district quiz.</p>

<h2>Use System Speaker on Jumps</h2>
<p>Supposedly turns off the tone that is made when a quizzer jumps on a toss-up question.</p>

<h2>USB Interface Box (QBox)</h2>
<p>This USB Interface Box comes with a USB cable to plug into the computer (it should be obvious which
end is which). This cable serves as the power source for the QBox; no secondary cable is required for
powering QBox.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/hardware-3.webp" alt="The USB QBox and its USB A-B cable." loading="lazy" />
  <figcaption>The USB QBox and its USB A-B cable.</figcaption>
</figure>

<h3>QBox driver installation</h3>
<p>The QBox driver installs immediately/automatically upon connecting to a computer that has internet
access:</p>
<ol>
  <li>To start, ensure your computer is connected to the internet <em>before</em> connecting the QBox
  to your computer.</li>
  <li>Then plug in QBox to your computer using the USB A-B cable that came with your QBox.</li>
  <li>Once the QBox is connected to the computer, the driver should begin installing automatically.
  Watch the installation windows in the bottom right-hand corner to make sure the driver install
  messages appear and that there are no errors.</li>
  <li>Start QuizMachine and a window indicating the USB Box is connected should be displayed. When you
  see this window message then you are ready to use QuizMachine with the QBox.</li>
</ol>

<h3>QBox driver troubleshooting</h3>
<p>If the QBox driver does not work or was not installed properly, check to make sure the driver has
been loaded successfully:</p>
<ol>
  <li>Click <strong>Start</strong></li>
  <li>Click <strong>Control Panel</strong></li>
  <li>Click <strong>System</strong> (may need to double click)</li>
  <li>Click <strong>Device Manager</strong></li>
  <li>If there is an entry for <strong>Other Devices</strong> with a yellow mark, then the driver did
  not load successfully. If this happens, close QuizMachine and disconnect QBox from your computer.
  Then start from the top of this section again to successfully install the QBox driver.</li>
</ol>

<h2>See also</h2>
<ul>
  <li><a href="doc:compatibility">Compatibility</a> — QBox works with versions 4 &amp; 5 only</li>
</ul>
`.trim(),
    },
    {
      slug: 'entering-lineups',
      title: 'Entering Lineups (Teams & Quizzers)',
      contentHtml: `
<p class="doc-warning">What is important to remember with Lineups &amp; Options is that all settings
must be decided <strong>before the ruling of the first question is made</strong> or before moving to
question 2, whichever happens first. Otherwise, if you want to change these settings you will first
have to erase all question data from the scoresheet through the beginning of question 1.</p>

<p>The Lineups window includes a button for <strong>Options</strong>, which contains quiz round rules
that apply to the current quiz round. It is important to check that these are correct before the start
of each quiz round. For more information see
<a href="doc:options-for-quiz-round">the "Options" window</a>.</p>

<h2>Teams &amp; Quizzers</h2>
<p>Before every quiz, each quiz team and quizzer's name must be entered and a captain and co-captain
specified. The teams and quizzers can be entered in one of two ways:</p>
<ol>
  <li>From a Names/Schedule file; for more information see
  <a href="doc:importing-names-and-schedule">importing &amp; loading a Names/Schedule file into
  QuizMachine</a>.</li>
  <li>By manually typing in (or selecting from the drop-downs) the names of each team and quizzer.</li>
</ol>

<p class="doc-warning">If a schedule file is being used, <strong>do not manually type in any team's or
quizzer's name</strong>.</p>
<ul>
  <li>If a team or quizzer's name is incorrectly spelt then this will mess up any statistics that may
  be taken from the saved quiz round data for that team and/or quizzer. Instead, select the correct
  team and quizzer from the available drop-downs for the current scheduled round.</li>
  <li>If you do not see the team or quizzer you are looking for, check that you are on the right round.
  If you are, then the schedule file may not have been loaded correctly, or a team/quizzer may not have
  been included in the schedule file. In either of these cases, please refer to your district
  coordinator for assistance.</li>
</ul>

<p>If a schedule file is not being used then the quizmaster can enter each team's name and quizzers by
manually typing them in.</p>

<h2>Number of teams</h2>
<p>To have a round with two teams, leave one of the team names blank (usually the middle/blue team).
The same applies for quizzing with a single team. If you are quizzing with three teams then enter all
three team names.</p>

<h2>Captain and Co-Captain</h2>
<p>The <strong>C</strong> and <strong>CC</strong> columns are for designating the Captain and
Co-Captain of each team. The current captain is allowed to do some things that the other quizzers of
their team cannot, such as Challenging and Appealing. If the Captain and Co-Captain of each team become
no longer eligible for jumping on toss-up questions, then QuizMachine will prompt that a new Captain
and Co-Captain be specified.</p>

<p>Once you have checked that all Lineups and Options are correct, you can click <strong>OK</strong>.
The Lineups window will close and you will be ready to start the quiz round with question 1.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/entering-lineups-1.webp" alt="The Lineups window — team and quizzer names with the C and CC columns." loading="lazy" />
  <figcaption>The Lineups window — team and quizzer names with the C and CC columns.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:options-for-quiz-round">Options for Quiz Round</a></li>
  <li><a href="doc:challenging-a-ruling">Challenging a Ruling</a></li>
  <li><a href="doc:appealing-a-question">Appealing a Question</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'options-for-quiz-round',
      title: 'Options for Quiz Round',
      contentHtml: `
<p>The values in the Options window define an essential part of the setup for the following quiz
round(s).</p>

<p class="doc-warning">The values in this window must be set <strong>before the quiz master makes a
ruling on question one</strong>. Otherwise, these values can only be changed if the round ends or if
the round is reset, wiping out all of that round's data.</p>

<h2>Where to find it</h2>
<p>The Options window can be accessed from two locations:</p>
<ol>
  <li>Setup/Exit &gt; Setup &gt; Rules/Scoring &gt; Options</li>
  <li>Lineups &gt; Options</li>
</ol>

<p class="doc-note">Quizzing <em>Out</em>, Erroring <em>Out</em>, and Fouling <em>Out</em> makes the
quizzer ineligible for toss-up &amp; bonus questions for the remainder of the quiz round.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/options-for-quiz-round-1.webp" alt="The Options window, reached from Lineups › Options." loading="lazy" />
  <figcaption>The Options window, reached from Lineups › Options.</figcaption>
</figure>

<h2>The options</h2>
<ul>
  <li><strong>Organization: Nazarene</strong> — indicates the quizzing organization; currently only
  "Nazarene" is an acceptable option. Located in Setup/Exit &gt; Setup &gt; Rules/Scoring only.</li>
  <li><strong>Rules/Method</strong>
    <ul>
      <li><strong>Tournament</strong> — for use during Quiz Meets and competitions</li>
      <li><strong>Practice</strong> — for use during quiz practices or any non-competition setting</li>
    </ul>
  </li>
  <li><a href="doc:option-quiz-out"><strong>Quiz Out</strong></a> — number of correctly-answered
  toss-up questions before a quizzer "quizzes out".</li>
  <li><a href="doc:option-error-out"><strong>Error Out</strong></a> — number of incorrectly-answered
  toss-up questions before a quizzer "errors out".</li>
  <li><a href="doc:option-foul-out"><strong>Foul Out</strong></a> — number of fouls required for a
  quizzer to "foul out".</li>
  <li><a href="doc:option-team-error-deduction"><strong>Team Error Deduction</strong></a> — number of
  incorrectly-answered toss-up questions (N) before the team begins accruing error penalties.</li>
  <li><a href="doc:option-individual-error-deduction"><strong>Individual Error Deduction</strong></a> —
  the same, but for the individual quizzer.</li>
  <li><a href="doc:option-tossup-clock-delay"><strong>Clock Start Delay</strong></a> — in tenths of a
  second; how long the system waits after a jump before starting the timer.</li>
  <li><a href="doc:option-bonus-clock-delay"><strong>Bonus Clock Delay</strong></a> — in tenths of a
  second; how long the system waits between the bonus window appearing and the bonus timer starting.</li>
</ul>

<h2>Tournament default values</h2>
<p>These are the values each QuizMachine system should show for quiz tournament use:</p>
<table>
  <tr><th>Option</th><th>Default</th></tr>
  <tr><td>Rules/Method</td><td>Tournament</td></tr>
  <tr><td>Quiz Out</td><td>4</td></tr>
  <tr><td>Error Out</td><td>3</td></tr>
  <tr><td>Foul Out</td><td>3</td></tr>
  <tr><td>Team Error Deduction</td><td>5</td></tr>
  <tr><td>Individual Error Deduction</td><td>3</td></tr>
  <tr><td>Clock Start Delay (tenths of seconds)</td><td>35</td></tr>
  <tr><td>Bonus Clock Start Delay (tenths of seconds)</td><td>101</td></tr>
</table>

<h2>See also</h2>
<ul>
  <li><a href="doc:entering-lineups">Entering Lineups (Teams &amp; Quizzers)</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'option-quiz-out',
      title: 'Changing Correct Answers to Quiz Out',
      contentHtml: `
<p><strong>Quiz Out</strong> is the number of correctly-answered toss-up questions before a quizzer
"quizzes out". The tournament default is <strong>4</strong>.</p>

<p class="doc-note">Quizzing out makes the quizzer ineligible for toss-up &amp; bonus questions for the
remainder of the quiz round. If the team has a 5th quizzer entered for the round, that quizzer
automatically substitutes in.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:options-for-quiz-round">Options for Quiz Round</a></li>
  <li><a href="doc:substituting-a-quizzer">Substituting a Quizzer</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'option-error-out',
      title: 'Changing Incorrect Answers to Error Out',
      contentHtml: `
<p><strong>Error Out</strong> is the number of incorrectly-answered toss-up questions before a quizzer
"errors out". The tournament default is <strong>3</strong>.</p>

<p class="doc-note">Erroring out makes the quizzer ineligible for toss-up &amp; bonus questions for the
remainder of the quiz round. If the team has a 5th quizzer entered for the round, that quizzer
automatically substitutes in.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:options-for-quiz-round">Options for Quiz Round</a></li>
  <li><a href="doc:option-individual-error-deduction">Individual Error Deductions</a></li>
  <li><a href="doc:substituting-a-quizzer">Substituting a Quizzer</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'option-foul-out',
      title: 'Changing Fouls Issued to Foul Out',
      contentHtml: `
<p><strong>Foul Out</strong> is the number of fouls required for a quizzer to "foul out". The
tournament default is <strong>3</strong>.</p>

<p class="doc-note">Fouling out makes the quizzer ineligible for toss-up &amp; bonus questions for the
remainder of the quiz round. If the team has a 5th quizzer entered for the round, that quizzer
automatically substitutes in.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:options-for-quiz-round">Options for Quiz Round</a></li>
  <li><a href="doc:issuing-fouls">Issuing Fouls</a></li>
  <li><a href="doc:substituting-a-quizzer">Substituting a Quizzer</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'option-team-error-deduction',
      title: 'Changing Incorrect Answers for Team to Start Incurring Error Deductions',
      contentHtml: `
<p><strong>Team Error Deduction</strong> is the number of incorrectly-answered toss-up questions (N)
before the team begins accruing error penalties. On the Nth error a point deduction
(<strong>-10</strong>) is issued to the team, as well as for all subsequent team errors of the quiz
round.</p>

<p>The tournament default is <strong>5</strong>.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:options-for-quiz-round">Options for Quiz Round</a></li>
  <li><a href="doc:option-individual-error-deduction">Individual Error Deductions</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'option-individual-error-deduction',
      title: 'Changing Incorrect Answers for Individual to Start Incurring Error Deductions',
      contentHtml: `
<p><strong>Individual Error Deduction</strong> is the same as
<a href="doc:option-team-error-deduction">Team Error Deduction</a> but for the individual quizzer. If
this is equal to <a href="doc:option-error-out">Error Out</a>, then the quizzer will receive a point
deduction (<strong>-10</strong>) when they error out.</p>

<p>The tournament default is <strong>3</strong>.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:options-for-quiz-round">Options for Quiz Round</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'option-tossup-clock-delay',
      title: 'Changing Toss-Up Clock Start Delay',
      contentHtml: `
<p><strong>Clock Start Delay</strong> (in tenths of a second) is the amount of time the system waits
after a quizzer jumps on a toss-up question before starting the timer.</p>

<p>The tournament default is <strong>35</strong> (3.5 seconds).</p>

<p class="doc-note">The default should be used in most cases.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:options-for-quiz-round">Options for Quiz Round</a></li>
  <li><a href="doc:working-with-the-timer">Working with the Timer</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'option-bonus-clock-delay',
      title: 'Changing Bonus Clock Start Delay',
      contentHtml: `
<p><strong>Bonus Clock Delay</strong> (in tenths of a second) is the amount of time the system waits
after the bonus window appears and the start of the bonus timer.</p>

<p>The tournament default is <strong>101</strong> (10.1 seconds).</p>

<p class="doc-note">The default should be used in most cases.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:options-for-quiz-round">Options for Quiz Round</a></li>
  <li><a href="doc:working-with-the-timer">Working with the Timer</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'quizmastering-workflow-basics',
      title: 'QuizMastering Workflow Basics (QuizMasters: Start Here)',
      contentHtml: `
<p>To begin you will need to set up the quiz round. For information on how to do this, see
<a href="doc:entering-lineups">the Lineups window</a> and
<a href="doc:options-for-quiz-round">the Options window</a>.</p>

<p>Once you (the QuizMaster) have set up the quiz round, you are ready for question one. You will see
the global interface/scoreboard populated with the teams and quizzers you entered in the Lineups
window.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/quizmastering-workflow-basics-1.webp" alt="The scoreboard at the start of a round, populated from the Lineups window." loading="lazy" />
  <figcaption>The scoreboard at the start of a round, populated from the Lineups window.</figcaption>
</figure>

<h2>The pattern for each question</h2>

<p class="doc-note"><strong>Suggestion:</strong> It is usually a good habit to read the question ahead
of time before beginning to read the question aloud. This reduces the probability of unintendedly
reading the question wrong and having to replace it with another question.</p>

<ol>
  <li>
    <p>You (the QuizMaster) say: <em>"Question 1 is a General question. Question number 1,
    question:"</em></p>
    <p>Each question will have a different type. If the type is not "General", then you would replace
    "General" with that question type — see <a href="doc:question-types">Question Types</a>. As you
    proceed with the following questions you would say the number of that question in place of "1".</p>
  </li>
  <li>
    <p>Then click the <strong>Next Jump</strong> button at the bottom in the center, or alternatively
    press the <strong>Spacebar</strong> to engage the jump seats. You will know the seats have been
    engaged when the button turns light orange and the text has changed from "Next Jump" to "Waiting
    for Jump".</p>
    <p class="doc-note">If you receive an audio tone when clicking "Next Jump" or pressing spacebar, it
    is because a quizzer's light was on (they were not properly seated) when you engaged the jump
    seats. In this case, allow the quizzer to get situated on the jump seat and then restart this
    process from the top by saying again, "Question 1 is a General question…"</p>
  </li>
  <li>Once the seats are engaged, the next quizzer to jump will trigger the question ruling window.</li>
  <li>
    <p>Proceed to read the rest of the question.</p>
    <p><strong>If a quizzer pre-jumps</strong> (the audio tone goes off before you reach the end of the
    question and the toss-up ruling window appears), stop reading the question. The quizzer will be
    required to both (1) finish the question and (2) provide the answer. The quizzer is allowed to ask
    "Is the question complete?" and receive an answer from the QuizMaster, such as "Yes", "No", "Yes it
    is", "No, it is not", or "It is not", etc.</p>
    <p class="doc-warning">Be cautious to not share any additional information about the question,
    especially information about how long the question has left before it is completed.</p>
    <p><strong>If you reach the end of the question</strong>, you can start the "No Jump" timer (5
    seconds) by any of these three means: (1) clicking <strong>No Jump (5 sec)</strong>, (2) clicking
    the <strong>Waiting for Jump</strong> button, or (3) pressing the spacebar a 2nd time. If no
    quizzers jump by the time that the "No Jump" timer reaches zero, then no one receives points.
    QuizMachine moves to the next question so that you are ready to read the next question right
    away.</p>
    <p class="doc-note"><strong>Sidenote:</strong> It is kind to provide the answer to the question that
    no one jumped on. This allows any curiosities to be satisfied and for quizzers to be ready to jump
    on the next question without being distracted by their curiosity about what the answer to the
    previous question actually was.</p>
  </li>
  <li>When quizzers jump on a question, only the first quizzer to jump is recognized as the winner of
  the toss-up question, and as their reward they get the chance to answer and receive points for the
  question.</li>
</ol>

<h2>Using the Quizzer Scoring Window</h2>
<p>At this point, you (the QuizMaster) would listen to the quizzer for their answer. If the quizzer
pre-jumped before you finished the question then they would have to provide the rest of the question
as well. See <a href="doc:quizzer-scoring-window">Quizzer Scoring Window</a>.</p>

<h2>After the Quizzer Scoring Window</h2>
<p>After a ruling is made, one of the team Captains may want to challenge the ruling or appeal the
question. The Captain may ask the QuizMaster to re-read the question and answer before they decide to
challenge or appeal; if the quizzer asks for this, go ahead and re-read the question and answer. See
<a href="doc:challenging-a-ruling">Challenging a Ruling</a> and/or
<a href="doc:appealing-a-question">Appealing a Question</a>.</p>

<h2>Between questions</h2>
<p>There are certain things that can happen between questions:</p>
<ul>
  <li><a href="doc:timeouts">Timeouts</a></li>
  <li><a href="doc:substituting-a-quizzer">Substituting a Quizzer</a> (automatic and/or during timeouts)</li>
  <li><a href="doc:viewing-the-scoresheet">Viewing the Scoresheet</a></li>
  <li><a href="doc:editing-the-scoresheet">Editing past Questions in the Scoresheet (Making Corrections)</a></li>
  <li><a href="doc:challenging-a-ruling">Challenging a Ruling</a></li>
  <li><a href="doc:appealing-a-question">Appealing a Question</a></li>
  <li><a href="doc:issuing-fouls">Issuing Fouls</a></li>
</ul>

<p>You (the QuizMaster) are now ready for the next question. Continue the process described above for
all 20 questions in the round.</p>

<h2>At the end of the quiz round</h2>
<p>Once a ruling for question 20 has been made, the "Question" window will show the word
<strong>DONE</strong> in all capital letters and the quiz round has reached its final score.</p>

<h2>Preparing for the next quiz round</h2>
<p>If you are using data from an imported schedule file, to set up the next round click the
<strong>Lineups</strong> button at the bottom of the scoring interface and then click <strong>Next
Quiz</strong>. You will know if the next quiz has loaded if the team names have changed from the
previous round. The scoring interface will also appear as it would at the start of the round.</p>
`.trim(),
    },
    {
      slug: 'question-types',
      title: 'Question Types',
      contentHtml: `
<table>
  <tr><th>Code</th><th>Type</th><th>Per round</th></tr>
  <tr><td>G</td><td>General</td><td>11 questions</td></tr>
  <tr><td>A</td><td>According To</td><td>4 questions</td></tr>
  <tr><td>X</td><td>Conte<u>x</u>t</td><td>1 question</td></tr>
  <tr><td>I</td><td>In what book and chapter</td><td>1 question (on years that apply)</td></tr>
  <tr><td>S</td><td>Situation</td><td>1 question (on years that apply)</td></tr>
  <tr><td>V</td><td>Verse — "Finish this verse"</td><td>1 question</td></tr>
  <tr><td>Q</td><td>Quote</td><td>1 question</td></tr>
  <tr><td>R</td><td>Reference — "Finish this verse and give the reference"</td><td>1 question</td></tr>
</table>

<h2>Notes on specific types</h2>
<ul>
  <li><strong>Verse (V)</strong> — the first 5 words of the verse only.</li>
  <li><strong>Quote (Q)</strong> — only the reference is provided.</li>
  <li><strong>Reference (R)</strong> — the first 5 words of the verse only; the reference is required
  after quoting the verse and without a prompt from the QuizMaster for the reference.</li>
</ul>

<h2>See also</h2>
<ul>
  <li><a href="doc:quizmastering-workflow-basics">QuizMastering Workflow Basics</a></li>
  <li><a href="doc:multipling">Multipling</a> — available on According To and other questions that
  provide a Scripture reference</li>
</ul>
`.trim(),
    },
    {
      slug: 'quizzer-scoring-window',
      title: 'Quizzer Scoring Window',
      contentHtml: `
<p>The purpose of this window is to give a ruling on the answer the quizzer gives for the question. If
the quizzer pre-jumped, they would also have to provide the remainder of the question in addition to
the answer.</p>

<h2>When it appears</h2>
<p>The quizzer scoring window appears when:</p>
<ul>
  <li>a quizzer jumps on a toss-up question</li>
  <li>a quizzer gets to answer a bonus question</li>
  <li>the QuizMaster clicks on the light of the quizzer</li>
</ul>
<figure>
  <img src="/docs/quizmachine-5.0.0/quizzer-scoring-window-1.webp" alt="The quizzer scoring window for a toss-up question." loading="lazy" />
  <figcaption>The quizzer scoring window for a toss-up question.</figcaption>
</figure>

<h2>Making a ruling (on a toss-up question)</h2>
<ol>
  <li>Click <strong>Correct</strong> (or press the <strong>C</strong> key) or <strong>Error</strong>
  (or press the <strong>E</strong> key).</li>
  <li>Then click <strong>OK</strong> (or press <strong>Enter</strong>) to confirm the ruling.</li>
</ol>
<p>To cancel out of the quizzer ruling window, click <strong>Cancel</strong> or press the
<strong>N</strong> key.</p>

<h2>Bonuses</h2>
<p>If the quizzer that won the toss-up gets an error for the question, the corresponding quizzer on the
other team will get a chance to answer the same question. For example, if the quizzer on Seat 2 gets
the toss-up wrong, then the quizzer(s) on Seat 2 of the other team(s) will get a chance to answer the
bonus.</p>

<p>For bonuses, the QuizMaster would have the quizzer stand up and then read the whole question for the
quizzer attempting the bonus.</p>

<p>Ruling on bonuses for two-team quiz match-ups is exactly the same as it is for the original toss-up
ruling.</p>

<p>Ruling on bonuses when quizzing with three teams will include two names in the quizzer scoring
window. The only difference is that the QuizMaster will have to make a ruling for each of the quizzers
before clicking <strong>OK</strong> (or pressing <strong>Enter</strong>).</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/quizzer-scoring-window-2.webp" alt="Ruling a bonus with three teams — two names appear, each needing a ruling." loading="lazy" />
  <figcaption>Ruling a bonus with three teams — two names appear, each needing a ruling.</figcaption>
</figure>

<h2>Timer and Timer Reset buttons</h2>
<p>The QuizMaster can control the timer from the quizzer scoring window. They can reset the timer to 0
(zero) by pressing <strong>Timer Reset</strong>. They can also add time to the clock by clicking
<strong>Timer</strong>. See <a href="doc:working-with-the-timer">working with the timer</a>.</p>

<h2>Foul button</h2>
<p>The quizzer scoring window provides the QuizMaster with a quick navigation to issue a foul for
whatever reason. See <a href="doc:issuing-fouls">issuing fouls</a>.</p>

<h2>Play button</h2>
<p>The quizzer might speak really quietly or quickly, or the QuizMaster may have said the wrong thing
while reading the question, or some other third thing. In all of these situations the QuizMaster can
play back the audio of the current question to help make a ruling. See
<a href="doc:audio-playback">using the audio playback feature</a>.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:multipling">Multipling</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'multipling',
      title: 'Multipling',
      contentHtml: `
<p>If a quizzer pre-jumps on an According To and declares "multiple", then the quizzer has chosen to
"multiple", which is to attempt to answer multiple potential questions based on the amount of the
question that they have received from the QuizMaster.</p>

<p>Multipling can be performed on <strong>According To</strong> questions or any other question that
provides a Scripture reference (for instance, Context questions will sometimes provide a reference at
the start of the question).</p>

<h2>How it is ruled</h2>
<p>With multipling, the quizzer must state all of the questions they want to answer and then answer
each question in the order that they stated the questions.</p>
<ul>
  <li>If they answer the correct question with the correct answer in the correct order, and if all
  answers to the prior questions were correct, then the QuizMaster can quick-call the correct answer
  and award the quizzer the points.</li>
  <li>If any incorrect information was provided for a specific question then the quizzer would be ruled
  incorrect and receive an error.</li>
</ul>

<h2>See also</h2>
<ul>
  <li><a href="doc:quizzer-scoring-window">Quizzer Scoring Window</a></li>
  <li><a href="doc:question-types">Question Types</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'working-with-the-timer',
      title: 'Working with the Timer',
      contentHtml: `
<p>The timer shows in the upper right corner of the scoring interface and is used in a number of
scenarios.</p>

<p>The timer <strong>always counts down</strong>. When the timer reaches zero an audio tone will play,
indicating that time is up.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/working-with-the-timer-1.webp" alt="The timer in the upper right of the scoring interface, with Timer and Timer Reset." loading="lazy" />
  <figcaption>The timer in the upper right of the scoring interface, with Timer and Timer Reset.</figcaption>
</figure>

<p>The following buttons will be available everywhere that the use of the timer is applicable:</p>
<ul>
  <li><strong>Timer</strong> button</li>
  <li><strong>Timer Reset</strong> button</li>
</ul>

<h2>The "Timer Reset" button</h2>
<p>This behaves the same way in all scenarios: clicking <strong>Timer Reset</strong> sets the timer to
0 (zero) <em>without</em> playing an audio tone. You can use this button to avoid unexpected
interruptions from QuizMachine when it normally would produce an audio tone.</p>

<h2>The "Timer" button</h2>
<p>This behaves in different ways based on where it is being used.</p>

<h3>For toss-ups and bonuses (Quizzer Scoring Window)</h3>
<ul>
  <li>When the quizzer scoring window opens, the timer will wait based on the specified delay (see
  <a href="doc:options-for-quiz-round">Options</a>); then the timer will start at 30 seconds and count
  down to zero.</li>
  <li>When the "Timer" button is pressed, it will jump back to the nearest 10 seconds above the current
  time (8 → 10; 22 → 30, etc). However, if the "Timer" button is pressed while showing a multiple of
  10, then it will add 10 to the timer (10 → 20; 20 → 30, etc).</li>
  <li>The timer will never show more than <strong>30 seconds</strong> for toss-ups and bonuses.</li>
</ul>

<h3>For no jumps (scoring interface)</h3>
<ul>
  <li>The timer is started by clicking the <strong>Waiting for Jump</strong> button while seats are
  engaged, by pressing spacebar while seats are engaged, or by clicking the <strong>No Jump
  (5 sec)</strong> button to the right of it. The timer will start counting down from 5 seconds.</li>
  <li>Pressing the <strong>No Jump</strong> button will put the timer on Hold. In this state, the timer
  will not be able to be restarted until you press the <strong>Reset Timer</strong> button (on the left
  of the "Waiting for Jump" button), which sets the timer to zero.</li>
  <li>The "No Jump" timer will never show more than <strong>5 seconds</strong>.</li>
</ul>

<h3>For timeouts (Timeouts window)</h3>
<p>Click the team that requested the timeout to start the timer counting down from
<strong>60 seconds</strong>. Other than this, the "Timer" button adds time to the timer the same way
that it does for challenges and appeals (see below).</p>

<h3>For challenges and appeals</h3>
<p>When the Challenge or Appeal window is opened, the timer defaults to 0 (zero). To add time to the
timer press the <strong>Timer</strong> button; the timer will start counting down. The button adds time
in increments of 15 seconds with each click (0, 15, 30, 45, 60).</p>
<ul>
  <li>If it is less than and within 2 seconds of the nearest 15-second increment then it will go to the
  next highest 15-second increment (14 or 15 → 30; 24 or 45 → 60, etc).</li>
  <li>If the timer is not in this range, it will go to the nearest 15-second increment (11 → 15;
  16 → 30, etc).</li>
</ul>

<h2>See also</h2>
<ul>
  <li><a href="doc:option-tossup-clock-delay">Toss-Up Clock Start Delay</a></li>
  <li><a href="doc:option-bonus-clock-delay">Bonus Clock Start Delay</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'scoresheet',
      title: 'Scoresheet',
      contentHtml: `
<p>The Scoresheet maintains the information about what happened during the round in a format that
emulates a coach's or content judge's physical scoresheet.</p>

<p>It shows, for each team: the quizzer names, a column per question (1–20 plus OT), total points,
correct/error counts, bonus/penalty points and the running score — along with the quizzers with correct
toss-ups, over-ruled challenges, errors, time-outs, fouls, coach initials and place.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/scoresheet-1.webp" alt="The Scoresheet, laid out like a coach's or content judge's physical sheet." loading="lazy" />
  <figcaption>The Scoresheet, laid out like a coach's or content judge's physical sheet.</figcaption>
</figure>

<h2>In this section</h2>
<ul>
  <li><a href="doc:viewing-the-scoresheet">Viewing the Scoresheet</a></li>
  <li><a href="doc:editing-the-scoresheet">Editing Past Questions in the Scoresheet (Making Corrections)</a></li>
  <li><a href="doc:printing-the-scoresheet">Printing the Scoresheet</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'viewing-the-scoresheet',
      title: 'Viewing the Scoresheet',
      contentHtml: `
<p>The scoresheet can be accessed from three locations:</p>

<ol>
  <li>From the <strong>scoring interface</strong> by clicking the <strong>Score Sheet</strong> button in
  the bottom right.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/viewing-the-scoresheet-1.webp" alt="The Score Sheet button in the bottom right of the scoring interface." loading="lazy" />
  <figcaption>The Score Sheet button in the bottom right of the scoring interface.</figcaption>
</figure>
  <li>From the <strong>Lineups window</strong> by clicking the <strong>Scoresheet</strong> button.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/viewing-the-scoresheet-2.webp" alt="The Scoresheet button in the Lineups window." loading="lazy" />
  <figcaption>The Scoresheet button in the Lineups window.</figcaption>
</figure>
  <li>From the <strong>Timeout window</strong> by clicking the <strong>Score Sheet</strong> button.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/viewing-the-scoresheet-3.webp" alt="The Score Sheet button in the Timeout window." loading="lazy" />
  <figcaption>The Score Sheet button in the Timeout window.</figcaption>
</figure>
</ol>

<h2>See also</h2>
<ul>
  <li><a href="doc:scoresheet">Scoresheet</a></li>
  <li><a href="doc:editing-the-scoresheet">Editing Past Questions (Making Corrections)</a></li>
  <li><a href="doc:printing-the-scoresheet">Printing the Scoresheet</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'editing-the-scoresheet',
      title: 'Editing Past Questions in the Scoresheet (Making Corrections)',
      contentHtml: `
<p>There is only one way to edit the scoring of a past question, and that is by removing data from the
scoresheet and going back to the last question you want to edit.</p>

<p class="doc-warning"><strong>Required:</strong> This process <strong>deletes quiz round data</strong>.
Before performing this operation ensure that you have a reliable record of what happened in the quiz
round up to the current question. Otherwise, you may lose quiz round data and the round's scoring will
be compromised.</p>

<h2>Removing the questions</h2>
<p>First open up the scoresheet using one of the methods shown in
<a href="doc:viewing-the-scoresheet">Viewing the Scoresheet</a>. Then click the <strong>Fix</strong>
button. The "Backup Fix" window will appear.</p>

<ol>
  <li>Enter the question number you want to erase data for (including the specified number).</li>
  <li>Click the <strong>Remove Questions</strong> button. A window confirming the success of the
  operation will appear. Click <strong>OK</strong> to close that alert.</li>
  <li>Click <strong>Done</strong> to close the Backup Fix window. Close the Scoresheet.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/editing-the-scoresheet-1.webp" alt="The “Backup Fix” window, where questions are removed from the scoresheet." loading="lazy" />
  <figcaption>The “Backup Fix” window, where questions are removed from the scoresheet.</figcaption>
</figure>
</ol>

<h2>Re-entering the data</h2>
<p>Proceed from the current question number, entering the correct data for each question up to the
current question using the external score record.</p>

<p class="doc-note">It is most efficient to open each quizzer's scoring window by clicking their light
in the scoring interface (next to the quizzer's name), making the ruling, and saving the ruling.</p>
`.trim(),
    },
    {
      slug: 'printing-the-scoresheet',
      title: 'Printing the Scoresheet',
      contentHtml: `
<p>If you would like to print the scoresheet, you can do so by clicking the <strong>Print</strong>
option while the scoresheet is open.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/printing-the-scoresheet-1.webp" alt="The Print option on the open scoresheet." loading="lazy" />
  <figcaption>The Print option on the open scoresheet.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:viewing-the-scoresheet">Viewing the Scoresheet</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'timeouts',
      title: 'Timeouts',
      contentHtml: `
<p>The Captain of a team or a coach may request a timeout. The QuizMaster will grant the timeout if the
team has not used up all of their timeouts.</p>

<p class="doc-note">See the official rule book for more on timeouts.</p>

<h2>Starting the timeout</h2>
<p>To start the timeout clock, you (the QuizMaster) would click the <strong>Time Out</strong> button.
The "Timeout" window will appear.</p>

<p>To start the timeout timer, <strong>select the team that requested the timeout</strong>. For example,
if the Captain of the Red team requested the timeout, you would click <strong>1 - Red Team</strong> to
start the timer.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/timeouts-1.webp" alt="The Timeout window — select the team that requested the timeout." loading="lazy" />
  <figcaption>The Timeout window — select the team that requested the timeout.</figcaption>
</figure>

<h2>Options during a timeout</h2>
<p>During the timeout there are some other options available, such as making a substitution and
resetting the timeout timer:</p>
<ul>
  <li><a href="doc:substituting-a-quizzer">Substitutions</a></li>
  <li><a href="doc:viewing-the-scoresheet">Score Sheet</a></li>
  <li><a href="doc:working-with-the-timer">Working with the timer</a></li>
</ul>

<h2>Ending the timeout</h2>
<p>When the timer reaches 0 (zero), the QuizMaster declares <strong>"Time-in"</strong> and the quizzers
and coaches return to their respective seats, ready for the next question.</p>
`.trim(),
    },
    {
      slug: 'substituting-a-quizzer',
      title: 'Substituting a Quizzer',
      contentHtml: `
<p>In Nazarene-style quizzing there are 4 active jump seats per team. However, QuizMachine allows 5
quizzers to be entered in the lineup. If your team has more than 4 quizzers, a 5th quizzer can be
entered for the round as a substitute.</p>

<p class="doc-note">Per the official Bible quizzing rulebook, only one quizzer is allowed to be entered
per round as a substitute (or a "5th"). You may have more than 5 quizzers on your team, however, for
each round only 5 quizzers maximum may be allowed to participate. (The rest of your team's quizzers
would be considered "alternates", meaning they are on your team but are ineligible to quiz in the
current round — even as a substitute.)</p>

<p>There are two ways in which substitution may occur.</p>

<h2>1. Substituting during a timeout (in-between questions)</h2>
<p class="doc-note">Please refer to the official rule book for specifics around substitution. Currently
the rules state that only one substitution can be made per timeout. There may be other more complex
scenarios that arise around substitution; for such scenarios, please refer to the official rule book
for how those scenarios would be handled.</p>

<p>During a timeout, you (the QuizMaster) will have the Timeout window currently open, and in that
window you will have the <strong>Substitution</strong> button available. To make a substitution during
a timeout, click this button. You will see the Substitution window appear:</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/substituting-a-quizzer-1.webp" alt="The Substitution button in the Timeout window." loading="lazy" />
  <figcaption>The Substitution button in the Timeout window.</figcaption>
</figure>

<ol>
  <li>Select the team that wants to make a substitution.</li>
  <li>Then select the quizzer you want to replace your 5th quizzer with. Note that only the active
  quizzers are shown as options and that the 5th quizzer is not.</li>
  <li>Then press <strong>Apply</strong> and <strong>OK</strong> to make the substitution.</li>
</ol>
<figure>
  <img src="/docs/quizmachine-5.0.0/substituting-a-quizzer-2.webp" alt="The Substitution window — only active quizzers are listed, not the 5th." loading="lazy" />
  <figcaption>The Substitution window — only active quizzers are listed, not the 5th.</figcaption>
</figure>

<p>Then wait for the timeout timer to reach zero. The 5th quizzer will now show in the scoring interface
where the previous quizzer used to be, indicating that the 5th quizzer is now on the previous quizzer's
seat number.</p>

<p>The QuizMaster should give the teams and spectators the courtesy of announcing the substitution so
that everyone is informed of the change.</p>

<h2>2. Substituting automatically when a quizzer quizzes out, errors out, or fouls out</h2>
<p>This is very straightforward: when a quizzer quizzes out, errors out, or fouls out, if the team has a
5th quizzer entered for the round, then the 5th quizzer will take the seat number of the quizzer that is
no longer eligible to answer questions.</p>

<p>You (the QuizMaster) do not have to do anything except to announce the substitution so that everyone
in the room is informed of the change. (This announcement would come after the previous announcement of
the quiz out, error out, or foul out of the other quizzer.)</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:timeouts">Timeouts</a></li>
  <li><a href="doc:option-quiz-out">Quiz Out</a>, <a href="doc:option-error-out">Error Out</a>,
  <a href="doc:option-foul-out">Foul Out</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'challenging-a-ruling',
      title: 'Challenging a Ruling',
      contentHtml: `
<p>The Captain on each team may challenge a ruling that a QuizMaster has given.</p>

<p>Before issuing the challenge, the Captain may ask for the complete question and answer from the
QuizMaster to use as the deciding factor as to whether they want to proceed with issuing the challenge
or not. The QuizMaster should honor this request in most scenarios, but the QuizMaster reserves the
authority to deny the Captain's request to challenge for any reason (otherwise, the QuizMaster would
not be the <u>master</u> of the quiz round; the QuizMaster has the final say in all cases).</p>

<p class="doc-note">However, once the quizzer has chosen to challenge, they cannot then receive the
question and answer if they ask for it.</p>

<h2>Opening the Challenge window</h2>
<p>Before the Captain begins their challenge, the QuizMaster must have the Challenge window open.</p>

<p>To open it, click the <strong>Challenge / Appeal / Foul</strong> button in the scoring interface and
then click <strong>Challenge</strong> in the window that appears. Then choose the team of the Captain
that wants to challenge.</p>

<p>The buttons that say <strong>Incorrect Information</strong> in this window are for the QuizMaster
while the Captains are stating the challenge and the rebuttal, so that it reminds the QuizMaster if they
stated any incorrect information. The QuizMaster can click these buttons and they will show as blue to
indicate that incorrect information was given.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/challenging-a-ruling-1.webp" alt="The Challenge window, with the “Incorrect Information” buttons." loading="lazy" />
  <figcaption>The Challenge window, with the “Incorrect Information” buttons.</figcaption>
</figure>

<h2>Running the challenge</h2>
<p>During a challenge, the Captain issuing the challenge states their case for why the ruling should be
overturned. The amount of time for a challenge can be set using the <strong>Timer</strong> button — see
<a href="doc:working-with-the-timer">working with the timer</a>.</p>

<p class="doc-note">Please refer to the official rule book for time given for a challenge.</p>

<p>When the quizzer has finished giving their case for overturning the ruling, the Captain of the other
team has a chance to provide their rebuttal. To maintain fairness, if a certain amount of time was given
to the first quizzer, then provide the same amount of time to the quizzer providing the rebuttal.</p>

<p>When both sides have given their case, the QuizMaster will decide to <strong>Accept</strong> the
challenge (overrule their original ruling) or <strong>Overrule</strong> it and keep the original
ruling.</p>

<p>The QuizMaster can also play back the audio of a question, answer, challenge or rebuttal to confirm
what was said — see <a href="doc:audio-playback">using the audio playback feature</a>.</p>
`.trim(),
    },
    {
      slug: 'appealing-a-question',
      title: 'Appealing a Question',
      contentHtml: `
<p>Sometimes the question was read incorrectly. Sometimes the answer on the page is incorrect. Sometimes
the question is just badly worded or doesn't make any sense.</p>

<p>In all of these cases, a Captain may request to appeal the question. The Captain may also request the
QuizMaster to repeat the full question and answer before choosing to appeal.</p>

<p class="doc-note">However, once the quizzer has chosen to appeal, they cannot then receive the
question and answer if they ask for it.</p>

<h2>Opening the Appeal window</h2>
<p>Before the Captain begins their appeal, the QuizMaster should have the Appeal window open.</p>

<p>To open it, click the <strong>Challenge / Appeal / Foul</strong> button in the scoring interface and
then click <strong>Appeal</strong> in the window that appears. Then choose the team of the Captain that
wants to appeal.</p>

<p>The timer clock will default to 0 (zero) but you may add time to the clock using the
<strong>Timer</strong> button. For specific time given for an appeal please refer to the official rule
book. See also <a href="doc:working-with-the-timer">working with the timer</a>.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/appealing-a-question-1.webp" alt="The Appeal window — the timer defaults to zero." loading="lazy" />
  <figcaption>The Appeal window — the timer defaults to zero.</figcaption>
</figure>

<h2>Ruling on the appeal</h2>
<p>Once the quizzer has given their case for appealing the question, the QuizMaster may choose to
<strong>Accept</strong> the appeal (throw out the question) or to <strong>Overrule</strong> it (keep the
question and the ruling for the question).</p>

<p>If an appeal is accepted, then the system will reset the question as if the previous question had just
finished. The QuizMaster then would choose a question with the <strong>same question type</strong> to
read in place of the appealed question.</p>

<p>The QuizMaster can also play back the audio of a question, answer, or appeal to confirm what was said
— see <a href="doc:audio-playback">using the audio playback feature</a>.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:question-types">Question Types</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'issuing-fouls',
      title: 'Issuing Fouls',
      contentHtml: `
<p>In the world of quizzing, fouls are sometimes necessary. The purpose of fouls is not to be
mean-spirited but instead in recognition of why everyone is gathered, which is to have fun competing
over knowledge of the Scriptures.</p>

<p>Most fouls are issued either due to audio disruptions and visual distractions. Sometimes fouls are
given due to cheating.</p>

<p>The QuizMaster is the <strong>only</strong> person who can issue fouls. The QuizMaster reserves the
authority to issue a foul to <u>anyone</u> — including spectators (and even themselves). The QuizMaster
is the <u>master</u> of the quiz round. If a spectator receives two fouls, they will be asked to leave
the room for the remainder of the quiz round.</p>

<p class="doc-note">Please see the official rule book on handling fouls for quizzers and coaches.</p>

<p class="doc-note">Some QuizMasters are gracious and will provide warnings before they choose to issue
a foul. Many QuizMasters will issue a foul without any warnings. It is generally a bad idea to expect
that you will receive a warning before receiving a foul; in most cases you will not.</p>

<h2>Issuing the foul</h2>
<p>To open the Foul window, click the <strong>Challenge / Appeal / Foul</strong> button in the scoring
interface and then click <strong>Foul</strong> in the window that appears.</p>

<p>Select one person to receive the foul and then press <strong>Apply</strong> to confirm and issue the
foul.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/issuing-fouls-1.webp" alt="The Foul window — select one person, then Apply." loading="lazy" />
  <figcaption>The Foul window — select one person, then Apply.</figcaption>
</figure>

<p>A point deduction may be applied to the team's total score after a certain number of fouls has been
received across the whole team.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:option-foul-out">Changing Fouls Issued to Foul Out</a></li>
  <li><a href="doc:quizzer-scoring-window">Quizzer Scoring Window</a> — includes a quick Foul button</li>
</ul>
`.trim(),
    },
    {
      slug: 'audio-playback',
      title: 'Using the Audio Playback Feature',
      contentHtml: `
<p>The audio playback feature exists for the playback of:</p>
<ul>
  <li>Questions and Answers (on toss-ups and bonuses)</li>
  <li>Challenges and Rebuttals</li>
  <li>Appeals</li>
</ul>

<h2>Why play back the audio?</h2>
<p>Sometimes a quizzer speaks really fast or really quietly. Sometimes the QuizMaster may want to check
and see if what they heard is correct. In challenges, the QuizMaster may want to play back the audio to
confirm what the quizzer bringing the challenge has claimed to have as an answer by the other team.
Sometimes an appeal of the question is based on a stumble or a mis-read word or words and the QuizMaster
may want to confirm if they really did read and say the question correctly.</p>

<p>In general, the audio playback feature allows for quizzing to be more fair for everyone, which keeps
quizzing fun.</p>

<h2>Using the player</h2>
<p>In all three locations, clicking the <strong>Play</strong> button (or pressing the <strong>P</strong>
key) will open the audio player window.</p>

<p>What you can do in the audio window:</p>
<ul>
  <li>Play audio for the <strong>Current</strong> or <strong>Previous</strong> question</li>
  <li>Start and stop the recording</li>
  <li>Control the volume</li>
  <li>Scrub through the audio by dragging</li>
  <li>Skip ahead and fast forward using the standard media icons</li>
</ul>
<figure>
  <img src="/docs/quizmachine-5.0.0/audio-playback-1.webp" alt="The audio player window, with Current/Previous and the media controls." loading="lazy" />
  <figcaption>The audio player window, with Current/Previous and the media controls.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:challenging-a-ruling">Challenging a Ruling</a></li>
  <li><a href="doc:appealing-a-question">Appealing a Question</a></li>
  <li><a href="doc:quizzer-scoring-window">Quizzer Scoring Window</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'connecting-to-qmserver',
      title: 'Connecting QuizMachine to QMServer via Network Connection',
      contentHtml: `
<p>Connecting QuizMachine to QMServer over a network requires having QMServer currently open &amp;
running on a computer, AND QMServer's network capabilities turned on and connected to a network.</p>

<p>See <a href="doc:qmserver/enabling-network-connectivity">enabling network connectivity in
QMServer</a>.</p>

<h2>Steps</h2>
<ol>
  <li>Launch QuizMachine.</li>
  <li>Click <strong>Setup/Exit</strong>. Then click <strong>Setup</strong> (or press the
  <strong>S</strong> key).</li>
  <li>Click <strong>Server/SneakerNet</strong> to view the server options.</li>
  <li>Near the bottom, select the <strong>Use the Server</strong> checkbox. This tells QuizMachine to
  try to load data over the network connection from QMServer.</li>
  <li>Enter the <strong>IP address of QMServer</strong>. QuizMachine will use this IP address to load
  data when the "Lineups" window is opened. (Use "localhost" or 127.0.0.1 to access QMServer on the same
  computer that QuizMachine is running on.)</li>
  <li>Then close the Setup window.</li>
</ol>
<figure>
  <img src="/docs/quizmachine-5.0.0/connecting-to-qmserver-1.webp" alt="The Server/SneakerNet tab, with “Use the Server” and the QMServer IP address." loading="lazy" />
  <figcaption>The Server/SneakerNet tab, with “Use the Server” and the QMServer IP address.</figcaption>
</figure>

<h2>Testing the connection</h2>
<p>To test the connection, follow the steps in
<a href="doc:getting-names-from-qmserver">getting team &amp; quizzer names from a Schedule file that has
been imported into QMServer</a>.</p>

<p>With a connection between QuizMachine and QMServer established, you should see the Room appear in the
top-level <strong>Rooms</strong> section in QMServer.</p>

<p class="doc-note">Every connected QuizMachine client sends a ping message to QMServer once a minute
with information about the state of the QuizMachine program, letting QMServer know that it is still
connected. The information that is sent with each ping is shown by the columns in the Rooms table.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/connecting-to-qmserver-2.webp" alt="The Rooms section in QMServer, showing the connected room's ping data." loading="lazy" />
  <figcaption>The Rooms section in QMServer, showing the connected room's ping data.</figcaption>
</figure>

<h2>See also</h2>
<ul>
  <li><a href="doc:sending-messages-to-qmserver">Sending messages to QMServer</a></li>
  <li><a href="doc:qmserver/pulling-quiz-round-data">Pulling quiz round data into QMServer</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'sending-messages-to-qmserver',
      title: 'Sending Messages from QuizMachine to QMServer Over Network Connection',
      contentHtml: `
<p>You can send messages from QuizMachine to QMServer when both are connected via network connection.
The message function is normally used at larger quiz meets to communicate with the operator of QMServer
to notify that there is an issue with the QuizMaster's QuizMachine program.</p>

<h2>Sending a message</h2>
<ol>
  <li>Connect to QMServer. Make sure that the Network functions have been turned on in QMServer before
  proceeding — see <a href="doc:connecting-to-qmserver">connecting to QMServer via network
  connection</a>.</li>
  <li>Open the Lineups window to set up the quiz round.</li>
  <li>
    <p>When you close the Lineups window, the scoring interface should update with the changes, as well
    as with the <strong>Msg</strong> (Message) button in the bottom right corner.</p>
    <p class="doc-note">You might not see this button if the right-most team has a 5th quizzer entered
    in Lineups. Otherwise, if you do not see this button then you may have an issue with your connection
    to QMServer.</p>
  </li>
<figure>
  <img src="/docs/quizmachine-5.0.0/sending-messages-to-qmserver-1.webp" alt="The Msg button in the bottom right of the scoring interface." loading="lazy" />
  <figcaption>The Msg button in the bottom right of the scoring interface.</figcaption>
</figure>
  <li>Click the <strong>Msg</strong> button.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/sending-messages-to-qmserver-2.webp" alt="The Send Message window." loading="lazy" />
  <figcaption>The Send Message window.</figcaption>
</figure>
  <li>Type your message in the <strong>Message</strong> text box. When you are ready to send, click
  <strong>Send</strong>. When you see the message in the <strong>Response</strong> text box above, then
  you know that the message was successfully delivered.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/sending-messages-to-qmserver-3.webp" alt="A delivered message echoed back in the Response box." loading="lazy" />
  <figcaption>A delivered message echoed back in the Response box.</figcaption>
</figure>
</ol>

<h2>What the QMServer operator sees</h2>
<ul>
  <li>All messages are received as a ping communication in the top-level <strong>Rooms</strong> section
  of QMServer. The QuizMachine client will show here whenever the client is connected to QMServer.</li>
  <li>When a user opens the "Msg" window, the room still shows in white, as normal.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/sending-messages-to-qmserver-4.webp" alt="In QMServer, the room still shows in white while the Msg window is open." loading="lazy" />
  <figcaption>In QMServer, the room still shows in white while the Msg window is open.</figcaption>
</figure>
  <li>When a message is sent from a room, the room will show in <strong>red</strong>, with the message
  showing in the <strong>Status/Error</strong> column.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/sending-messages-to-qmserver-5.webp" alt="The room turns red, with the message in the Status/Error column." loading="lazy" />
  <figcaption>The room turns red, with the message in the Status/Error column.</figcaption>
</figure>
  <li>This row will remain red as long as the QuizMachine client continues to have the Send Message
  window open.</li>
  <li>When the window is then closed, the room will return back to white, having erased the message sent
  by the QuizMachine client user.</li>
</ul>
`.trim(),
    },
    {
      slug: 'getting-names-from-qmserver',
      title: 'Getting Team & Quizzer Names from QMServer via Network Connection',
      contentHtml: `
<p>Make sure a Schedule file is currently loaded/imported in QMServer at the top level (not a stats
group) with teams and quizzers in it. Also ensure that QuizMachine is connected over the network to
QMServer with the correct IP or website address — see
<a href="doc:connecting-to-qmserver">connecting QuizMachine to QMServer via network connection</a>.</p>

<h2>Steps</h2>
<ol>
  <li>Launch QuizMachine.</li>
  <li>
    <p>Click <strong>Lineups</strong> to open the Lineups window. You may see a loading progress window
    flash on your screen in most cases and then the Lineups window will appear.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/getting-names-from-qmserver-1.webp" alt="The loading progress window while QuizMachine pulls data from QMServer." loading="lazy" />
  <figcaption>The loading progress window while QuizMachine pulls data from QMServer.</figcaption>
</figure>
    <p class="doc-note"><strong>Troubleshooting:</strong> If you get a message stating that QuizMachine
    cannot connect to the server, check the following: (1) that the IP or website address in
    QuizMachine &gt; Setup &gt; Server/SneakerNet is correct; (2) that QMServer is running on the
    computer that has that IP or website address; (3) that network functions are turned on in QMServer;
    and (4) that a Schedule file is loaded in QMServer in the top-level "Schedule" section.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/getting-names-from-qmserver-2.webp" alt="The “cannot connect to the server” message." loading="lazy" />
  <figcaption>The “cannot connect to the server” message.</figcaption>
</figure>
  </li>
  <li>In the Lineups window, click the drop-down field for one of the teams. You should see the team
  names of the Schedule file imported in QMServer. When you select a specific team you should then also
  see the team's quizzers as options in the quizzer drop-down fields.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/getting-names-from-qmserver-3.webp" alt="The Lineups team drop-down, populated from QMServer's Schedule file." loading="lazy" />
  <figcaption>The Lineups team drop-down, populated from QMServer's Schedule file.</figcaption>
</figure>
</ol>

<h2>See also</h2>
<ul>
  <li><a href="doc:qmserver/importing-names-and-schedule">Importing Names &amp; Schedule into QMServer</a></li>
  <li><a href="doc:entering-lineups">Entering Lineups (Teams &amp; Quizzers)</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'importing-names-and-schedule',
      title: 'Importing/Loading Names & Schedule Data into QuizMachine from a File',
      contentHtml: `
<p>You can set up your quiz rooms' QuizMachine programs using a Names/Schedule file so that your
QuizMasters will always have the right teams for their room and the right quizzers for each team. Using
a Names/Schedule file is also ideal if you want to generate stats after the quiz using QMServer.</p>

<p>See <a href="doc:qmserver/creating-names-schedule-file">how to create or get a Names/Schedule file for
your quiz</a>.</p>

<h2>Importing the file</h2>
<ol>
  <li>First, launch the QuizMachine program.</li>
  <li>
    <p>Go to <strong>Setup &gt; Server/Sneakernet</strong>. While here, please check the following:</p>
    <ul>
      <li>Make sure the <strong>Use Imported Names</strong> checkbox <strong>is</strong> checked.</li>
      <li>Make sure the <strong>Use the Server</strong> checkbox is <strong>NOT</strong> checked.</li>
    </ul>
  </li>
<figure>
  <img src="/docs/quizmachine-5.0.0/importing-names-and-schedule-1.webp" alt="The Server/Sneakernet tab — “Use Imported Names” checked, “Use the Server” unchecked." loading="lazy" />
  <figcaption>The Server/Sneakernet tab — “Use Imported Names” checked, “Use the Server” unchecked.</figcaption>
</figure>
  <li>Click the <strong>Export/Import Wizard</strong> button. You will see the Export/Import window.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/importing-names-and-schedule-2.webp" alt="The Export/Import window, with Import Names/Schedules." loading="lazy" />
  <figcaption>The Export/Import window, with Import Names/Schedules.</figcaption>
</figure>
  <li>Make sure the <strong>Use Imported Names</strong> checkbox is selected. Then click <strong>Import
  Names/Schedules</strong>. A file browser window will appear.</li>
  <li>Select the file to load names and schedules from. Then click <strong>Open</strong>. QuizMachine
  will begin loading the names and schedule and will conclude with a confirmation message.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/importing-names-and-schedule-3.webp" alt="The confirmation after the names and schedule finish loading." loading="lazy" />
  <figcaption>The confirmation after the names and schedule finish loading.</figcaption>
</figure>
  <li>Press <strong>OK</strong> to close. Then exit out of the Export/Import window and return to the
  scoring interface.</li>
</ol>

<h2>Loading the first quiz round</h2>
<ol start="7">
  <li>
    <p>Click <strong>Lineups</strong>.</p>
    <p class="doc-note">There is a window that flashes very quickly while the Lineups window is opening.
    This window appears if schedule data has been loaded into QuizMachine; it indicates the progress of
    loading the schedule and names into the Lineups window's fields.</p>
  </li>
  <li>Click <strong>Load Quiz</strong>. The "Load another Quiz" window will appear.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/importing-names-and-schedule-4.webp" alt="The “Load another Quiz” window." loading="lazy" />
  <figcaption>The “Load another Quiz” window.</figcaption>
</figure>
  <li>The drop-down options should populate with the schedule data that you imported if the import
  succeeded. If you don't see the Tournament you are looking for, something may have gone wrong with the
  import.</li>
  <li>
    <p>Select the <strong>Tournament</strong>, <strong>Division</strong>, <strong>Room</strong>, and
    <strong>Round</strong> by clicking the drop-down arrows to the right of each option.</p>
    <p class="doc-note">The schedule included in the file that you imported may have a "dummy" round
    that has no teams attached to it. This is often added as a buffer round for loading quizzes, as well
    as a way to offer teams who are waiting to start their first quiz a practice round that will not be
    saved in any way or apply to the official stats later on.</p>
  </li>
<figure>
  <img src="/docs/quizmachine-5.0.0/importing-names-and-schedule-5.webp" alt="Selecting the Tournament, Division, Room and Round." loading="lazy" />
  <figcaption>Selecting the Tournament, Division, Room and Round.</figcaption>
</figure>
  <li>
    <p>Once you have the correct information selected, click <strong>OK</strong>. The schedule data
    should show as loaded in the Lineups window. Check that the teams shown match the teams for that
    room's first round (if the room has a BYE for the first round, then it would be the second round's
    teams that should match).</p>
    <p>If no data has appeared or changed in the Lineups window, it may be because the first round was a
    dummy round added as a buffer and the true first round starts after clicking <strong>Next
    Quiz</strong>. If this applies to you, click <strong>Next Quiz</strong> in the Lineups window, then
    click <strong>Yes</strong> to confirm. You should then see the team names populated. The quizzer
    lineups may or may not also be pre-populated into the drop-downs; however, the quizzers' names
    should appear as options when you click on the quizzer drop-down for the team that they are on.</p>
  </li>
</ol>

<p>When the QuizMaster sits down for the first round, they will select the names of the quizzers from
the drop-down options and assign the Captain (C) and Co-captain (CC), then click <strong>OK</strong> to
close the Lineups window and start the round.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/importing-names-and-schedule-6.webp" alt="The Lineups window with the schedule data loaded." loading="lazy" />
  <figcaption>The Lineups window with the schedule data loaded.</figcaption>
</figure>

<h2>One last thing…</h2>
<p>QuizMachine computers that are used at a quiz can come from a number of different places, including
churches that use QuizMachine regularly for their practices. For this reason the settings may not be set
up the way they need to be for use in a quiz tournament.</p>

<ol start="12">
  <li>With the Lineups window still open, click <strong>Options</strong>. Ensure that all QuizMachine
  systems show the tournament default values — see
  <a href="doc:options-for-quiz-round">Options for Quiz Round</a>.</li>
</ol>

<p>If everything went as planned, the QuizMachine program should be set up with the data to start the
quiz for the room with the correct options configured. Proceed with repeating this process for each
QuizMachine program in each quiz room to be used in the quiz.</p>

<p class="doc-warning">If a name is spelled wrong in the drop-downs, <strong>do not edit it</strong>
round-by-round. Unless it ends up spelled the same exact way at every occurrence, QuizMachine will treat
each spelling as a distinct quizzer. This can cause problems later on when you try to run stats. It is
best just to leave it alone; it can be corrected later.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:qmserver/format-names-schedule-file">Format: Names/Schedule File</a></li>
  <li><a href="doc:entering-lineups">Entering Lineups (Teams &amp; Quizzers)</a></li>
  <li><a href="doc:qmserver/renaming-a-quizzer-or-team">Renaming/correcting a name afterward in QMServer</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'rooms-without-schedule-file',
      title: 'Setting Up Quiz Rooms Without a Names/Schedule File with Plans to Run Stats Afterward',
      contentHtml: `
<p>If you are wanting to run stats at the end of your quiz it is <u>highly suggested</u> that you use a
Names/Schedule file to seed the QuizMachine programs that you will use in each quiz room before the start
of the quiz.</p>

<p>The major reason for this is because running stats is dependent on the <strong>exact spelling</strong>
of the names of the teams and quizzers/individuals. It is the same reason why it is advised that names
that were imported into QuizMachine are not changed during the quiz rounds — because the changes could
cause problems when calculating stats.</p>

<p class="doc-warning">This method is suggested so strongly that it has been decided to not include
instructions for entering data for setting up quiz rooms without a Names/Schedule file with plans to run
stats afterward.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:importing-names-and-schedule">How to set up QuizMachine using a Names/Schedule file</a></li>
  <li><a href="doc:qmserver/creating-names-schedule-file">Creating/Getting a Names/Schedule File</a></li>
</ul>
`.trim(),
    },
    {
      slug: 'exporting-quiz-round-data',
      title: 'Exporting Quiz Round Data from QuizMachine to a File',
      contentHtml: `
<p>QuizMachine data is written to a file throughout the progression of each quiz round. This allows you
to export the quiz round data later after the last quiz round.</p>

<p>There are two ways to transfer data from QuizMachine (client) to QMServer: (1) over a LAN (local
WiFi), and (2) via a file on a USB thumb drive. This section covers the USB method — for sending over
LAN see <a href="doc:qmserver/pulling-quiz-round-data">sending quiz round data over LAN (WiFi)</a>.</p>

<p class="doc-note">For testing QMServer and QuizMachine together it may be best to use the same computer
for the client (QuizMachine) and the server (QMServer) so that the file can be saved somewhere on that
computer's hard drive rather than a USB thumb drive.</p>

<h2>Exporting quiz data to a file</h2>
<ol>
  <li>First, note whether the current quiz round in the scoring interface shows <strong>DONE</strong> in
  the current Question box. If it does not, then the current round will <em>not</em> be included in the
  export; if it does say "DONE" then the current round <em>will</em> be included.</li>
  <li>
    <p>Next, navigate to the Export window. This can be reached from:</p>
    <ul>
      <li>Scoresheet &gt; Export/Import</li>
      <li>Setup &gt; Server/Sneakernet &gt; Export/Import Wizard</li>
      <li>Lineups &gt; Scoresheet &gt; Export/Import</li>
      <li>Timeout &gt; Scoresheet &gt; Export/Import</li>
    </ul>
  </li>
  <li>It is best to select <strong>Today's</strong> quiz rounds to export only today's quizzes, but any
  option will work for date range.</li>
  <li>Make sure the <strong>Use Imported Names</strong> checkbox is selected.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/exporting-quiz-round-data-1.webp" alt="The Export window, with the date-range options and Export Quizzes." loading="lazy" />
  <figcaption>The Export window, with the date-range options and Export Quizzes.</figcaption>
</figure>
  <li>Then click <strong>Export Quizzes</strong>. A file browser window will open.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/exporting-quiz-round-data-2.webp" alt="The file browser for naming and saving the quizzes file." loading="lazy" />
  <figcaption>The file browser for naming and saving the quizzes file.</figcaption>
</figure>
  <li>
    <p>In the file browser window give the file a name and choose the location that you want to save the
    file to (which is probably the inserted USB thumb drive). Then click <strong>Save</strong>.</p>
    <p class="doc-note">It is suggested that you name the quiz file with the tournament name and room
    designation (such as "Indy2007R1" for Room 1 of the Indy 2007 quiz). Ultimately what matters is if
    you can tell where the data came from.</p>
  </li>
  <li>If QuizMachine asks whether to create the file, press <strong>Yes</strong> to start the export
  process.</li>
<figure>
  <img src="/docs/quizmachine-5.0.0/exporting-quiz-round-data-3.webp" alt="QuizMachine asking whether to create the file." loading="lazy" />
  <figcaption>QuizMachine asking whether to create the file.</figcaption>
</figure>
  <li>
    <p class="doc-warning">Please be patient while QuizMachine exports the quiz round data. <strong>Do
    not shut off QuizMachine. Do not strike any keys on your keyboard or click any buttons with the
    mouse</strong>, as this may interfere with the export process and cause it to fail. This is
    especially the case for slower computers.</p>
    <p>While the file is exporting a status window will show. Wait until you have received the second
    window with a confirmation message from QuizMachine that the file has been created.</p>
<figure>
  <img src="/docs/quizmachine-5.0.0/exporting-quiz-round-data-4.webp" alt="The confirmation that the file has been created." loading="lazy" />
  <figcaption>The confirmation that the file has been created.</figcaption>
</figure>
  </li>
</ol>

<p class="doc-note">QuizMachine exports quizzes with the default file name <code>Quizzes.csv</code>.</p>

<h2>See also</h2>
<ul>
  <li><a href="doc:qmserver/importing-quiz-round-data">Importing quiz round data into QMServer</a></li>
  <li><a href="doc:qmserver/format-quizzes-file">Format: Quizzes File (Quiz Round Data File)</a></li>
</ul>
`.trim(),
    },
  ],
}
