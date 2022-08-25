#!/bin/bash

if [ -z "$1" ]
  then
    echo "Target MAC address not set"
    exit 1
fi

sudo etherwake -i eth0 "$1"
