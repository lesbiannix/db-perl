from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="transport-api-wrapper",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    rust_extensions=[
        RustExtension("transport_api_wrapper.transport_api_wrapper", "Cargo.toml", debug=False),
    ],
    packages=["transport_api_wrapper"],
    zip_safe=False,
)