#!/bin/bash

# Sourcing the .env file.
# This file is used to read in the DATABASE_URL environment variable that SQLx needs to
# setup and migrate the database.
if [[ -f .env ]]
then
    source .env
fi

# Using the absolute path to SQLx executable if CARGO_HOME environment is set.
# This case is usually true in official rust docker image builds.
if [[ -v CARGO_HOME ]]
then
    sqlx=$CARGO_HOME/bin/sqlx

    if ! command -v $sqlx
    then
        echo "SQLx installation not found"
        exit 1
    else
        echo "Setting up the database"
        $sqlx database setup
        echo "Done"
    fi
# If the absolute path could not be found, using the SQLx executable frop PATH as a fallback.
else
    if command -v sqlx
    then
        echo "Setting up the database"
        sqlx database setup
        echo "Done"
    else
        echo "SQLx installation not found. If you have SQLx installed, make sure it's accessible via PATH or you can set the CARGO_HOME environment variable"
        exit 1
    fi
fi
