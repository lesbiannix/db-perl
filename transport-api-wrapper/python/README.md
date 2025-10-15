# transport-api-wrapper Python Bindings

This directory contains the Python bindings for the `transport-api-wrapper` Rust library.

## Building and Installing

To build and install the Python module, you will need to have Rust and Cargo installed. You will also need to have Python 3 and `pip` installed.

1.  **Build the Rust library:**

    ```bash
    cd transport-api-wrapper
    cargo build --features python --release
    ```

2.  **Install the Python module:**

    ```bash
    cd ..
    pip install -e .
    ```

## Usage

Once the module is installed, you can use it in your Python code as follows:

```python
import transport_api_wrapper

locations = transport_api_wrapper.locations("berlin")
print(locations)
```