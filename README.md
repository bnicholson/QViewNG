# QViewNG

[by [QuizStuff](https://www.quizstuff.com)]

QView is a React + Rust web application designed to manage tournaments for Quizzing events 
such as Bible Quizzing, Physics Quizzes, etc.

A demo of QViewNG is available for viewing at [qviewng-demo.teenbiblequiz.com](https://qviewng-demo.teenbiblequiz.com).

Project/Task management for QViewNG is done in [Jira](https://qviewng.atlassian.net/jira/core/projects/QVNG/board?filter=&groupBy=status). Access is limited to 10 developers max. Please contact Collyn Brenner at collynbrenner@gmail.com to request access.

This GH repo is planned to replace the [/bnicholson/QView repo](https://github.com/bnicholson/QView) eventually and then be renamed to 'QView'.

The vision for QViewNG can be found in the [GH Wiki pages](https://github.com/bnicholson/QViewNG/wiki) of this GH repo.

Tech stack overview:
- Rust provides the backend APIs and microservices.
- The frontend is designed using the React JavaScript framework.
- The UI is designed using the Material UI framework and components.
- The backend database is Postgresql.
- Redis/Valkey is planned to be used as a cache and as a inter QView server store.

At this time, contributions are being accepted only in the form of developer code contribubtions. If interested please context Collyn Brenner at collynbrenner@gmail.com.

<br/>

## Dev Environment Setup

Thise guide is aimed at Ubuntu 24.04 OS. You should be able to do these same things on MacOS, Windows OS and/or WSL2, however this guide will stick to Ubuntu 24.04 OS.

1) Install Ubuntu desktop on a machine or on a VM/Hypervisor/Virtualbox.

2) Install 'git' in Ubuntu.
   ```
   sudo apt install git
   ```

3) Clone the project in the working directory you desire.
   ```
   git clone https://github.com/bnicholson/qviewng.git
   cd qview
   ```

4) Run `getrustdev` (*this script loads all the required linux (debian) programs needed to develop
   such as Rust, PostgreSQL, etc.):
	```
   ./getrustdev
   ```

5) Now install the JavaScript libraries needed by the UI frontend:
   ```
   cd frontend
   yarn install  // or 'npm i'
   cd ..
   ```

6) Now create a development database. QViewNG uses PostgreSQL as the database. Create a standard
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
   The script is intended to replace steps #8, #9, #10 however the script has flaws so at this time is
   preferred to stick with the manual steps above until those are sorted out.

8) Copy and paste the '.env.example', '.env.release.example' and '.env.test,example' files in the same directory they are found in (the root dir), remove the '.example' portions from the file names, and manually populate it with the necessary values. Eaxh example file has comments descriing what each ENV VAR is used for and supposed to be (or at least it should if it has been maintained well).

9) Run the DB migrations on the datbase you just created to establish in the DB the schema and perhaps some data:
   ```
   cd {root}/backend
   diesel migration run
   ```

10) Remove the superuser permissions from the user you created earlier:
   ```
   sudo bash
   su - postgres
   psql
   ALTER USER steve with NOSUPERUSER;
   \q
   exit
   exit
   ```

- Now you are ready to run QView's frontend and backend together with one command. (Note: The Vite server usually listend on http://localhost:5173):
  ```
  cd {root}/frontend/
  npm run dev
  ```

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
  
- To run integration tests, use file {root}/backend/run-tests.sh. It loads the .env.test config file and provides a quick way to switch between one file and all integration tests, reducing the amount of CLI typing needed to run integration tests. Just run:
  ```
  ./backend/run-test.sh
  ```

# How It Works
The backend is written in Rust and the frontend in TypeScript.
The frontend uses React, Redux, and Material UI.
