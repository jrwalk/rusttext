from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="rusttext",
    version="0.0.1",
    packages=["rusttext"],
    rust_extensions=[RustExtension("rusttext", "Cargo.toml", debug=False)],
    include_package_data=True,
    zip_safe=False,
)
