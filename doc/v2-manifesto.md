## What sucks about Squirrel v1's Design™

* Squirrel.Windows and Squirrel.Mac are completely different
  - This confuses the living fuck out of people

* Squirrel.Mac is missing lots of features
  - Delta updates
  - Updates via static HTTPS (i.e. S3)
  - Rollout updates 

* Squirrel.Windows uses NuGet files
  - Not worth it now that Electron is such a huge client
  - NuGet library gives us some tooling but also we're pinned to their garbage
  - NuGet compression / decompression library sucks out loud
  - Squirrel.Mac just uses Basic-Ass Zip Files, much better

* Squirrel.Mac relies on lots of Xcode libraries that bitrot
  - RAC is cool, but Xcode breaks every library all the time
  - Nobody can build Squirrel.Mac anymore

* Squirrel.Windows builds the tooling code and the installation code in the same library
  - This makes Update.exe way bigger than it needs to be, and makes the flags for Update.exe bananas.
  - Make code that we actually ship in the box as small as possible

* Squirrel.Windows has two separate APIs
  - One API is through C#, the other API is via parameters to shell out to Update.exe
  - That's no way to live
