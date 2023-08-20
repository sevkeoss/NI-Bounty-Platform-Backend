#!/bin/sh

psql -U sevag -d ni-bounty -a -f database/init.sql
sea-orm-cli generate entity -o src/models/