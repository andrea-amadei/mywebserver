#!/bin/bash

if [ -z "$1" ]
  then
    echo "Target IP address not set"
    exit 0
fi

ping -c 1 "$1" > /dev/null
if [ $? -eq 0 ]
  then
    echo "OK"
    exit 0
  else
    echo "ERROR"
fi
