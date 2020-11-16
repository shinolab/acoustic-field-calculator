# Requirements (when using GPU)

With GPU, this library use [Vulkano (Vulkan API wrapper)](https://github.com/vulkano-rs/vulkano) and [shaderc-rs](https://github.com/google/shaderc-rs), so you have to install shaderc and set SHADERC_LIB_DIR environment variable.
Unless you install shaderc, following programs must be installed and available in PATH to build from source. For more information, please refer to each repository.

* CMake
* Ninja
* Python 

# Example

* With CPU
    ```
    cargo run --release --example main
    ```

* With GPU
    ```
    cargo run --release --example main --features gpu
    ```

# Author

Shun Suzuki, 2020
