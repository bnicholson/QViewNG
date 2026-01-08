# QView

[by [QuizStuff](https://www.quizstuff.com)]

QView is a React + Rust web application designed to manage tournaments for Quizzing events 
such as Bible Quizzing, Physics Quizzes, etc.

This GH repo is intended to replace the [/bnicholson/QView repo](https://github.com/bnicholson/QView) some time in the future.

The vision for the QView project (meaning the most detailed information about QView's future) can be found in this [Wiki pages](https://github.com/bnicholson/QViewNG/wiki) of this GH repo.

Rust provides the backend APIs and microservices.
The frontend is designed using the React JavaScript framework.
The UI is designed using the Material UI framework and components.

The backend database is Postgresql.
Redis/Valkey is planned to be used as a cache and as a inter QView server store if we 
end up having two application servers.

If you'd like to support the project by contributing your development time please don't hesitate to reach out to Collyn Brenner at collynbrenner@gmail.com. We're looking to make this a reality - and there is plenty to do!

Note:  A beta/UAT development environment may be available for viewing at http://qview.quizstuff.com:3000

<br/>

## Setting Up the Dev Environment (*requirements*)

1) Install Ubuntu desktop on a machine or on a VM/Hypervisor/Virtualbox. (It is best if you have a computer with Ubuntu 24.04+ installed on it.)

2) Install 'git' in Ubuntu.
   ```
   sudo apt install git
   ```

3) Clone the project in the working directory you desire.
   ```
   git clone https://github.com/bnicholson/qviewng.git
   cd qview
   ```

4) `getrustdev`: this script loads all the required linux (debian) programs needed to develop
   such as Rust, PostgreSQL, etc.
	```
   ./getrustdev
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
   The script is intended to replace steps #8, #9, #10 however the script has flaws so you may
   want to stick with the manual steps above until those are sorted out.

7) Copy and paste the '.env.example' file in the same directory it exists in (the root dir) and
   manually populate it with the necessary values (specifically for the database conneciton string).

8) Run the DB migrations on the datbase you just created to establish in the DB the schema and perhaps some data:
   ```
   cd {root}/backend
   diesel migration run
   ```

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
  
  OR (from {root}:)
  
  ./run-backend.sh
  ```

# How It Works
The backend is written in Rust and the frontend in TypeScript.
The frontend uses React, Redux, and Material UI.
