# semverbump
Semantic Versioning Bumper


What does it do?
================

Will print current version if any and bumped version

How to use it?
==============

You can copy `semverbump` to `/usr/local/bin/semverbump` or somewhere available in your path

Available options are: patch, minor, major.

For example if current version tag is 0.1.1:

Using `patch` will bump `0.1.1` to `0.1.2`

    semverbump patch
    0.1.1 --> 0.1.2

Using `minor` will bump `0.1.1` to `0.2.0`

    semverbump minor
    0.1.1 --> 0.2.0


Using `major` will bump `0.1.1` to `1.0.0`

    semverbump major
    0.1.1 --> 1.0.0


# regex
    https://regex101.com/r/ahzkLW/1/
