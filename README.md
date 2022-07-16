# osu_db_pyo3
A Python module written in Rust for reading osu!.db file

# The project uses:
  * Python Launcher for Windows for launching scripts (https://www.python.org/)
  * PyO3 - Rust bindings for Python (https://github.com/PyO3/pyo3)
  * maturin - tool for building and publishing Rust-based Python packages with minimal configuration  (https://github.com/PyO3/maturin)

# Get Started:
```
git clone https://github.com/Murchik/osu_db_pyo3.git osu_db_pyo3
cd osu_db_pyo3

py -m pip install -U virtualenv
py -m venv env
.\env\Scripts\activate

py -m pip install -U pip wheel
py -m pip install -U -r .\requirements.txt

maturin develop
py main.py
```
