#! /bin/bash

trap on_sigint SIGINT
on_sigint() { echo "SIGINT"; exit 0; }

trap on_sigterm SIGTERM
on_sigterm() { echo "SIGTERM"; exit 0; }

echo "Waiting for signals"
while true; do
    sleep 1
done
