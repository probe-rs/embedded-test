---
name: Bug report or Problem
about: Create a report to help us improve
title: ''
labels: ''
assignees: ''

---

### Before you start

1. Please check whether you're experiencing an issue listed in the [FAQ](https://github.com/probe-rs/embedded-test/wiki/FAQ-and-common-errors).
2. Issues related to the host-side of this project are kept in the probe-rs repository (tagged [`component:embedded-test`](https://github.com/probe-rs/probe-rs/issues?q=is%3Aissue%20state%3Aopen%20label%3Acomponent%3Aembedded-test)) . If you already know the issue is related to the host-side, feel free to open the issue over there and to tag the maintainer of embedded-test (@t-moe).

Thank you!

------

**Describe the bug**
A clear and concise description of what the bug is.

**Version information**
* `probe-rs --version`
* the version of embedded test
* Include the output of `cargo tree | grep "embassy" -C3` if you are facing issues with embassy

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Additional context**
Add any other context about the problem here.
