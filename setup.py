"""Entry point for installing hyperdrive math python package"""

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="hyperdrivepy",
    version="0.18.0",
    packages=["hyperdrivepy"],
    package_dir={"": "bindings/hyperdrivepy/python"},
    rust_extensions=[
        RustExtension("hyperdrivepy.hyperdrivepy", binding=Binding.PyO3, path="bindings/hyperdrivepy/Cargo.toml"),
    ],
    # Rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    python_requires=">=3.8",
)
