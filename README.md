# Research Project: Rust API

## Setup
Project can be easily launched with Docker, just run the `docker-compose.yml`!


You can also opt to launch it the manual way for development.

**Note that you'll still need to host a MySQL database for this (also included in docker compose), this also assumes you have both NodeJS and Rust installed**
#### API setup
```
# Install MySQL tools (Linux)
$ sudo apt-get install libmysqlclient-dev

# Install Diesel CLI (with MySQL features)
$ cargo install diesel_cli --no-default-features --features mysql 

# Seed database
$ diesel database reset

# Install API dependencies and run
# cargo run
```
#### Frontend 
```
# go to Frontend directory
$ cd ./frontend

# Install dependencies
$ npm install

# Run dev server
$ npm run dev
```

## Usage

**IMPORTANT** 

Quite a few routes will require you to provide a JWT token and failing to do so will return a `403 FORBIDDEN` response, you can get a token from the frontend console after registering an account.


Base URL: `http:localhost:8000/api`

## Routes

### Users
`POST /users/`

Creates a user in database, requires Authorisation token for UID
#### Request
```json
{
    "username": "aaron"
}
```
#### Response
```json
{
    "id": 4,
    "uid": "9XppyFMscLe15daqncSpFXiSqKG2",
    "role": 0,
    "username": "aaron",
    "score": 0,
    "history": []
}
```
`GET /users/`

Fetch all users, requires admin

`GET /users/<ID>`

Fetch user by ID, requires admin

`GET /users/self`
Fetch yourself from database

`DELETE /users/

### Quizzes

`GET /quiz/`

Get a short version of all Quizzes

`GET /quiz/<ID>`
#### Response

```json
{
    "id": 1,
    "name": "Animal Group Names",
    "description": "Can you name the group names for these animals?",
    "questions": [
        {
            "id": 2,
            "question": "A group of which animals is referred to as a wake?",
            "answers": [
                {
                    "answer": "Vultures",
                    "is_correct": true
                },
                {
                    "answer": "Crocodiles",
                    "is_correct": false
                },
            ]
        },
    ]
}
```

`POST /quiz/<QUIZ_ID>/answer/<QUESTION_ID>`

#### Request
```json
{
    "answer": "Vultures"
}
```
#### Response
```json
{
    "success": true,
    "status": 200,
    "message": "",
    "data": {
        "correct": true,
        "correct_answer": "Vultures",
        "total_score": 1
    }
}
```
```json
{
    "success": false,
    "status": 400,
    "message": "User has already answered this question",
    "data": {}
}
```

`GET /quiz/<ID>/score`

Get your current score of queried Quiz




