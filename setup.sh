mkdir nyan_browser
cd nyan_browser
mkdir src
touch src/main.rs
touch src/browser.py
mkdir -p src/resources/icons
mkdir -p src/resources/html
mkdir -p src/resources/icons/svg
mkdir -p src/resources/images

# Create and activate a virtual environment
python3 -m venv venv
source venv/bin/activate

# Install required Python packages
pip install PyQt5 PyQtWebEngine requests pillow

cargo init
