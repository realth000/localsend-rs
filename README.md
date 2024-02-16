# localsend-rs

## Introduction

This repo is working on a cli version of localsend for some situations when using the official GUI version on desktop
plarforms:

* Frequent same file transfer is not convenient enough.
* Single time file transfer can be lighter and easier to use.
* Automatically transfer could be done with full configuration.

# Motivation

Actually the motivation of this repo is some linux style convenient command experience in terminal.

When testing flutter apps on psychical devices:

1. Open localsend.
2. Open the file browser.
3. In the file browser, find the file path which is `$repo_root/build/app/outputs/flutter-apk/app.apk`.
4. Drag the file into localsend UI.
5. Select the target device in localsend.

Repeating these steps is tiring and a shell script is not good enough.

So here comes the idea of cli version localsend.

# Goal

* [ ] Single time file send.
* [ ] Configure the receiver and automatically send files to the target.
* [ ] Waiting for the receiver to start.
* [ ] Receive file from configured sender.
