# Lines of Code Counter

## Overview

This repository contains a GitHub Actions workflow designed to provide insights into your project's development by tracking changes in your codebase. On every `push` to your repository, this workflow automatically calculates and reports the number of meaningful lines of code that have been added or removed.

The analysis focuses on key web development file types and excludes comments and other trivial changes to provide an accurate measure of your project's evolution.

## Features

- **Automated Analysis**: The workflow runs automatically on every push, so you always have up-to-date information.
- **Meaningful Metrics**: It intelligently ignores comments and trivial changes, focusing only on the code that matters.
- **Detailed Reporting**: Provides a clear summary of added and removed lines for supported file types.
- **Project Size Tracking**: In addition to line changes, it also reports the overall size of the project.
- **Seamless Integration**: All reports are available directly in the GitHub Actions job summary, so you don't have to leave GitHub to see the results.

## Supported File Types

The workflow currently analyzes the following file types:

- `.js` (JavaScript)
- `.json` (JSON)
- `.html` (HTML)
- `.css` (CSS)

## How to Use

1.  **Install the Workflow**: To use this in your own repository, copy the workflow file from `.github/workflows/` into your project's `.github/workflows/` directory.
2.  **Push a Change**: Make a commit and push it to your repository.
3.  **View the Report**: Navigate to the **Actions** tab in your repository. Click on the latest workflow run, and you will find the lines of code report in the job summary.

## Contributing

Contributions are welcome! If you have ideas for improvements or have found a bug, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
