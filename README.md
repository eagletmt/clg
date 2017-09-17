# clg
Manage remote Git repository clones

## Usage
```
% clg clone eagletmt/clg
Cloning into '/home/eagletmt/.ghq/github.com/eagletmt/clg'...
remote: Counting objects: 4, done.
remote: Compressing objects: 100% (3/3), done.
remote: Total 4 (delta 0), reused 0 (delta 0), pack-reused 0
Unpacking objects: 100% (4/4), done.
% clg look clg
chdir /home/eagletmt/.ghq/github.com/eagletmt/clg
% git remote -v
origin  https://github.com/eagletmt/clg (fetch)
origin  https://github.com/eagletmt/clg (push)
% exit
% clg look eagletmt/clg
chdir /home/eagletmt/.ghq/github.com/eagletmt/clg
% exit
% clg list | grep cl
github.com/eagletmt/clg
```

### Clone
You can also use scp-like syntax.

```
% clg clone git@github.com:eagletmt/clg
Cloning into '/home/eagletmt/.ghq/github.com/eagletmt/clg'...
remote: Counting objects: 4, done.
remote: Compressing objects: 100% (3/3), done.
remote: Total 4 (delta 0), reused 0 (delta 0), pack-reused 0
Receiving objects: 100% (4/4), done.
% clg look clg
chdir /home/eagletmt/.ghq/github.com/eagletmt/clg
% git remote -v
origin  ssh://git@github.com/eagletmt/clg (fetch)
origin  ssh://git@github.com/eagletmt/clg (push)
```

## Acknowledgments
This software is based on [ghq](https://github.com/motemen/ghq) written by motemen.
