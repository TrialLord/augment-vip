First sign out from the IDE (and the website)\
Then run the binary from the [releases](https://github.com/Ran-Mewo/augment-vip/releases) page\
After that you should create a new account and log onto that new account 😊

首先从IDE（以及网站）退出登录,并且关闭IDE\
然后从 [releases](https://github.com/Ran-Mewo/augment-vip/releases) 页面获取并运行二进制文件\
之后你应该创建一个新账户，然后登录那个新账户 😊

**Supported IDEs:**
- All JetBrains IDEs
- All VSCode-based IDEs

## Mac OS Compatibility

- The program now includes improved support for macOS, including proper file locking using both Unix permissions and the `chflags uchg` command to make files immutable on macOS systems.
- All file path handling and permission changes are now cross-platform.
- You can test Mac OS compatibility automatically using GitHub Actions (see the Actions tab in your forked repository).

## How to Test on Mac OS (without a Mac)

- This project uses GitHub Actions to build and test on macOS runners.
- To test, push your changes to your fork and trigger the workflow from the Actions tab on GitHub.
- The workflow will build the project for both Intel and Apple Silicon Macs and report any issues.
