# Filer CLI Tool

Filer is a command-line interface (CLI) tool designed to help users manage text files by saving versions, working on them, and comparing changes.

## Features

- **Create File:** Create a new text file.

    ```sh
    cargo run create [filename]
    ```

- **Delete File:** Delete an existing text file.

    ```sh
    cargo run delete [filename]
    ```

- **Create File Version:** Save a new version of an existing text file.

    ```sh
    cargo run version [filename]
    ```

- **Compare Versions:** Compare changes between two versions of a text file.

    ```sh
    cargo run cmp [filename]
    ```

- **Help:** Display usage information.

    ```sh
    cargo run help
    ```

## Prerequisites

- [Rust](https://www.rust-lang.org/)

## Getting Started

1. Clone the repository:

    ```sh
    git clone https://github.com/your-username/file-cli.git
    ```

2. Navigate to the project directory:

    ```sh
    cd file-cli
    ```

3. Build and run the CLI tool:

    ```sh
    cargo build
    ```

4. Execute commands:

    ```sh
    cargo run [command] [filename]
    ```

## Examples

- Create a new file:

    ```sh
    cargo run create example
    ```

- Delete a file:

    ```sh
    cargo run delete example
    ```

- Save a new version:

    ```sh
    cargo run version example
    ```

- Compare versions:

    ```sh
    cargo run cmp example
    ```

## Contributing

Contributions are welcome! If you have any ideas for improvement or find any issues, feel free to open an [issue](https://github.com/your-username/file-cli/issues) or submit a [pull request](https://github.com/your-username/file-cli/pulls).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
