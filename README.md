[woofie]() — the reputation cryptocurrency
==================================================
<!--
Contribution Guides
--------------------------------------

In the spirit of open source software development, jQuery always encourages community code contribution. To help you get started and before you jump into writing code, be sure to read these important contribution guidelines thoroughly:

1. [Getting Involved](http://contribute.jquery.org/)
2. [Core Style Guide](http://contribute.jquery.org/style-guide/js/)
3. [Writing Code for jQuery Foundation Projects](http://contribute.jquery.org/code/)


Environments in which to use jQuery
--------------------------------------

- [Browser support](http://jquery.com/browser-support/) differs between the master branch and the compat branch. Specifically, the master branch does not support legacy browsers such as IE8. The jQuery team continues to provide support for legacy browsers on the compat branch. Use the latest compat release if support for those browsers is required. See [browser support](http://jquery.com/browser-support/) for more info.
- To use jQuery in Node, browser extensions, and other non-browser environments, use only master branch releases given the name "jquery" rather than "jquery-compat". The compat branch does not support these environments.
-->

How to build your own Woofie
--------------------------------------

In order to build Woofie, you need to have Meteor installed, which you can do by running this command: 
```bash
curl https://install.meteor.com | /bin/sh
```

After installing Meteor, clone down the repo:
```bash
git clone https://github.com/sunny-g/wufi
```

Install and build the dependencies:
```bash
cd .gulp
npm install
gulp
```

Go back to the root directory and run Meteor!
```bash
meteor
```

----------------------------

Questions?
----------

If you have any questions, please feel free to email me. Thanks for checking this out!
