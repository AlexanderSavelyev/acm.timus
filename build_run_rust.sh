#!/bin/sh

rustc $1 --test && ./$2 --nocapture