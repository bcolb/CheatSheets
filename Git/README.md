# Useful Git Commands

Git is a distributed version control system originally developed by Linux Torvalds for version control amongst the various developers of the Linux kernel. Git has since seen wide adoption in software development. Several sites, such as Github and Gitlab allow public and private hosting and sharing of git repositories. This is just a cheatsheet of useful commands, for extensive document see the [git documentation at git-scm](https://git-scm.com/).

## Installing git locally

If Git is not already installed locally, it can be done with the following commands.

Debian based Linux distros (i.e. Ubuntu, Linux Mint)
```
sudo apt install git-all
```

Redhat/Fedora
```
sudo dnf install git-all
```

Activate Git in MacOS
```
git version
```

For Windows download [Git for windows](https://gitforwindows.org/).

## Cloning a remote repository

It is common to create your repository first in a web hosted location like Github or Gitlab, then clone it locally to work on it. Most of the time you'll be cloning an existing repository to work on it.

For instance, to clone this CheatSheets repository over https, use the following command:

```
https://github.com/bcolb/CheatSheets.git
```

Working over https and having to authenticate with a username/password can get pretty tiresome. It is recommended to setup an SSH key, then clone via SSH.

```
git@github.com:bcolb/CheatSheets.git
```

Alternatively, is you install the Github CLI you could clone the same project with the CLI as such:

```
gh repo clone bcolb/CheatSheets
```

Learn more about [setting up an SSH key for Github here.](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/adding-a-new-ssh-key-to-your-github-account)

## Setting up a new local repository

First, you'll need to create your new project directory. We'll call our project **git-project**.

```
cd /path/to/directory/
mkdir git-project
```

### Initialize the repository

Once in the new project directory, run the following command.

```
git init .
```

### Set up a remote

To setup a project to track a remote repository, say in Github, you'll need to add a remote. 

If the project didn't already exist, you'd have to navigate to Github and create it. Then run the following command from your local repository.

```
git remote add origin https://github.com/bcolb/git-project.git
```

Then verify that it was setup correctly.

```
git remote -v
```

### Committing and pushing changes

README files are often the first files created for a git repo. They are commonly written in markdown with the .md file extension.

```
touch README.md
```

Then add and commit the files.

```
git add README.md
git commit -m 'added an initial project README'
```

Other files that are often created are:
- LICENSE - if you are licensing your code for use/reproduction/etc, otherwise it is default copyrighted
- CODE_OF_CONDUCT.md - community code of conduct if it is a community driven project
- CONTRIBUTING.md - instructions and guidelines on rontributing to the repo

[More info on licenses here](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository)

Push your changes to the remote repository with the following.

```
git push origin
```

## Fork a repository

Forking a repository creates a copy of it in your namespace (i.e. github.com/bcolb/someone-else-forked-repo). The original repository is then referred to as the "upstream" repository.

To work on another repo, first fork it, make and commit a change, then submit a pull request. Usually forking is done in the Github UI.

## Working on branches

Branches are often used to create a specific feature and then are merged back into a master or main branch. Some repositories will be set up with different branch names, such as production and development.

```
git branch feature-name-branch
git checkout feature-name-branch
```

### Push a local branch to a remote repository

```
git push origin feature-name-branch
```
### Merge a local branch into master

```
git checkout master
git merge feature-name-branch
```

### Delete a branch locally

```
git branch --delete feature-name-branch
```

### Reverting a change

Create a new branch to revert the change. You will need the commit ID that you want to revert back to. You can see recent commits using ```git log```.

```
git branch feature-name-branch-revert
git checkout feature-name-branch-revert
git revert -S 1a2b3c4d5a6b7c8d9a1b0c
git add changes
git commit -m 'reverted changes'
git push origin feature-name-branch-revert
```

## Other useful commands

Fetch and merge in a single command.

```
git pull upstream
```

Prepare an email submission for a path.

```
git format-patch
```