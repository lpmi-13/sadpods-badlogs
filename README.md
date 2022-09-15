# Sad Pods

Inspired by the amazing [sad servers](https://sadservers.com), this is an attempt to create deterministic (and a bit random) learning environments for linux troubleshooting using [Gitpod](https://gitpod.io/docs).

## First exercise

Find the process that's writing to `/var/log/badlog.py` and stop it. There are three that are running (one python, one golang, and one rust), and each startup randomizes which one is writing to that particular log file.
