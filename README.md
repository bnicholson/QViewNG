# QView

[by [QuizStuff](https://www.quizstuff.com)]

QView is a React + Rust web application designed to manage tournaments for Quizzing events 
such as Bible Quizzing, Physics Quizzes, etc.

QView is available for viewing at [qview.teenbiblequiz.com](https://qview.teenbiblequiz.com).

Project/Task management for QView is done in [Jira](https://qviewng.atlassian.net/jira/core/projects/QVNG/board?filter=&groupBy=status). 
Access is limited to 10 developers max. Please contact Collyn Brenner at collynbrenner@gmail.com to request access.

This GH repo is planned to replace the [/bnicholson/QView repo](https://github.com/bnicholson/QView) eventually and then be renamed to 'QView'.

The vision for QView can be found in the [GH Wiki pages](https://github.com/bnicholson/QViewNG/wiki) of this GH repo.

Tech stack overview:
- Rust provides the backend APIs and microservices.
- The frontend is designed using the React JavaScript framework.
- The UI is designed using the Material UI framework and components.
- The backend database is Postgresql.
- Redis/Valkey is planned to be used as a cache and as a inter QView server store.

Note: You may see "QViewNG" in places around the codebase. It stands for "QView Next Generation" and it is the codename used 
for the pre-alpha version of QView. Eventually all instances of it will be replaced by "QView" or "qview".

At this time, contributions are being accepted only in the form of developer code contribubtions. If interested please context 
Collyn Brenner at collynbrenner@gmail.com.

<br/>

## Dev Environment Setup

QView was originally developed using Ubuntu 24.04 and then using Omarchy/Arch. You should be able to accomplish the steps below 
on MacOS, Windows OS and/or WSL2, however this guide will stick to setup on a linux machine.

1) Install your desired non-headless OS on a bare metal device or on a VM.

2) Install 'git'.
   ```
   sudo apt install git
   ```

3) Clone the project in the working directory you desire.
   ```
   git clone https://github.com/bnicholson/qviewng.git
   cd qviewng
   ```

4) Run `getrustdev.sh` (*this script loads all the required linux programs needed to develop
   such as Rust, PostgreSQL, etc.):
	```
   ./getrustdev.sh
   ```

5) Now install the JavaScript libraries needed by the UI frontend:
   ```
   cd frontend
   yarn install  // or 'npm i'
   cd ..
   ```

6) Now create a development database. QView uses PostgreSQL as the database. Create a standard
   PostgreSQL database using the following commands as the PostgreSQL user:
   ```
   sudo bash
   su - postgres
   psql
   CREATE DATABASE qviewdev;
   CREATE USER steve;
   ALTER USER steve PASSWORD 'somepassword';
   ALTER USER steve WITH SUPERUSER;
   \q
   exit
   exit
   ```

   Note:  There is an example script under setup-db-valkey.sh that does this plus other work.
   The script is intended to replace steps #6, #8, #9 however the script has flaws so at this time is
   preferred to stick with the manual steps above until those are sorted out. If adventurous, feel free to try using it!

7) Copy and paste the '.env.example', '.env.dev.example' and '.env.test.example' files in the same directory they are found in (the root dir), remove the '.example' portions from the file names, and manually populate it with the necessary values. Eaxh example file has comments descriing what each ENV VAR is used for and supposed to be (or at least it should if it has been maintained well).

8) Run the DB migrations on the database you just created to establish in the DB the schema and perhaps some data:
   ```
   cd {root}/backend
   diesel migration run
   ```
   (Docker deployments don't need this step: `docker compose up` runs a one-shot `migrate` service that applies any pending migrations before the app starts.)

9) Remove the superuser permissions from the user you created earlier:
   ```
   sudo bash
   su - postgres
   psql
   ALTER USER steve with NOSUPERUSER;
   \q
   exit
   exit
   ```

- Now you are ready to run QView's frontend and backend together with one command. (Note: The Vite server usually listens on http://localhost:5173):
  ```
  cd {root}/frontend/
  npm run dev
  ```

  This also starts the Valkey cache container and waits for it to report healthy before the
  backend boots, so the backend finds the cache on its first connection attempt. Docker must be
  running; if it is not, `npm run dev` prints a warning and runs uncached rather than failing.
  The cache is purely an optimization -- see backend/src/cache.rs. To develop with it off, set
  QVIEW_SKIP_VALKEY=1. To start or stop it on its own: `npm run valkey:start` / `npm run
  valkey:stop`.

  The Vite dev server runs with `--host`, so it listens on all network interfaces, not just
  localhost. Other devices on your network (e.g. a phone or tablet for testing) can reach it at
  http://{your-machine-ip}:5173.

- If you want to build the backend without the frontend:
  ```
  cd {root}/backend/
  cargo build --bin backend
  ```

- Or if you want to build and run the backend without the frontend, use 'run' instead of 'build':
  ```
  cd {root}/backend/
  cargo run --bin backend
  
  OR (from {root}/backend:)
  
  ./run-backend.sh
  ```
  
### Tests

- To run unit tests use the usual command:
  ```
  cargo test --lib
  ```
  
- To run integration tests, use file {root}/backend/run-tests.sh. It loads the .env.test config file, starts a throwaway test Postgres container (see compose.test.yaml), and runs the tests with cargo-nextest. Docker must be running, and nextest must be installed once with `cargo install --locked cargo-nextest`. The script's comments show how to run one file or one test instead of all of them. Just run:
  ```
  ./backend/run-tests.sh
  ```

# How It Works
The backend is written in Rust and the frontend in TypeScript.
The frontend uses React, Redux, and Material UI.
