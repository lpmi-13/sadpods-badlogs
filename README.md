# Sad Pods

Inspired by the amazing [sad servers](https://sadservers.com), this is an attempt to create deterministic (and a bit random) learning environments for linux troubleshooting using [Gitpod](https://gitpod.io/docs).

## Run in Gitpod

You can run this entirely in gitpod if you want.

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/lpmi-13/sadpods)

## First exercise

Find the process that's writing to `/var/log/bad.log` and stop it. There are three that are running (one python, one golang, and one rust), and each startup randomizes which one is writing to that particular log file.
