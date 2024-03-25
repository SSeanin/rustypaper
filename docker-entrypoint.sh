#!/bin/bash

# Sourcing the .env file.
# This file is used to read in the DATABASE_URL environment variable that SQLx needs to
# setup and migrate the database.
# This script also checks for presence of /run/secrets folder since Docker mounts the secrets in this directory.
# If secrets are present then we're in a production environment and DATABASE_URL is created using the secrets
if [[ -d /run/secrets ]]
then
    export DATABASE_URL=postgres://$(cat $RUSTYPAPER_DATABASE_USER_FILE):$(cat $RUSTYPAPER_DATABASE_PASSWORD_FILE)@$(echo $RUSTYPAPER_DATABASE_HOST):5432/$(echo $RUSTYPAPER_DATABASE_NAME)
elif [[ -f .env ]]
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

        echo $DATABASE_URL

        if [[ $? -ne 0 ]]
        then
            echo "Error encountered in database setup"
            exit 1
        else
            echo "Done"
        fi
    fi
# If the absolute path could not be found, using the SQLx executable from PATH as a fallback.
else
    if ! command -v sqlx
    then
        echo "SQLx installation not found. If you have SQLx installed, make sure it's accessible via PATH or you can set the CARGO_HOME environment variable"
        exit 1
    else
        echo "Setting up the database"
        sqlx database setup

        if [[ $? -ne 0 ]]
        then
            echo "Error encountered in database setup"
            exit 1
        else
            echo "Done"
        fi
    fi
fi

# Running the app as the first argument
sh -c $1
