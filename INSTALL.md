Build your own local version of Whuffie on OS X
--------------------------------------

First, you will need install Meteor and Neo4j, which you can do by running these commands (this requires [homebrew](http://brew.sh/)):
```bash
curl https://install.meteor.com | /bin/sh   # installs Meteor and MongoDB
brew update																	# updates homebrew formulae
brew install neo4j                          # installs neo4j
```

After installing Meteor, clone down the repo:
```bash
git clone https://github.com/sunny-g/whuffie
```

Install and build the dependencies with [gulp](http://gulpjs.com/) (currently only stellar-lib):
```bash
cd whuffie/.gulp
npm install
gulp
```

Go back to the root directory of the repo, then run Neo4j and Meteor. Enjoy!
```bash
cd ..
neo4j start
meteor
```