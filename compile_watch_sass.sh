#!/bin/sh

gnome-terminal --command="sass style:style"
sleep 5
gnome-terminal --command="sass --watch style:style"
