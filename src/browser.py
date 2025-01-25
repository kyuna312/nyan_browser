from PyQt5.QtWidgets import (QApplication, QMainWindow, QWidget, QVBoxLayout,
                           QHBoxLayout, QPushButton, QTabWidget, QLineEdit, QLabel)
from PyQt5.QtWebEngineWidgets import QWebEngineView, QWebEnginePage
from PyQt5.QtCore import QUrl, Qt, QSize
from PyQt5.QtGui import QIcon, QPixmap
import os
import sys

class TabWidget(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.layout = QVBoxLayout(self)

        # Navigation bar
        nav_layout = QHBoxLayout()

        # Navigation buttons
        self.back_btn = QPushButton()
        self.forward_btn = QPushButton()
        self.reload_btn = QPushButton()
        self.home_btn = QPushButton()
        self.bookmark_btn = QPushButton()
        self.download_btn = QPushButton()
        self.settings_btn = QPushButton()

        # URL bar
        self.url_bar = QLineEdit()

        # Add navigation items to layout
        nav_layout.addWidget(self.back_btn)
        nav_layout.addWidget(self.forward_btn)
        nav_layout.addWidget(self.reload_btn)
        nav_layout.addWidget(self.home_btn)
        nav_layout.addWidget(self.bookmark_btn)
        nav_layout.addWidget(self.url_bar)
        nav_layout.addWidget(self.download_btn)
        nav_layout.addWidget(self.settings_btn)

        # Web view
        self.web_view = QWebEngineView()

        # Add to layout
        self.layout.addLayout(nav_layout)
        self.layout.addWidget(self.web_view)

        # Connect signals
        self.back_btn.clicked.connect(self.web_view.back)
        self.forward_btn.clicked.connect(self.web_view.forward)
        self.reload_btn.clicked.connect(self.web_view.reload)
        self.home_btn.clicked.connect(self.go_home)
        self.url_bar.returnPressed.connect(self.navigate_to_url)
        self.web_view.urlChanged.connect(self.update_url)

    def go_home(self):
        home_path = os.path.join(os.path.dirname(__file__),
                                "resources/html/homepage.html")
        self.web_view.setUrl(QUrl.fromLocalFile(home_path))

    def navigate_to_url(self):
        url = self.url_bar.text()
        if not url.startswith(('http://', 'https://')):
            url = 'https://' + url
        self.web_view.setUrl(QUrl(url))

    def update_url(self, url):
        self.url_bar.setText(url.toString())

    def setup_icons(self):
        # Set icon sizes
        icon_size = QSize(24, 24)
        for btn in [self.back_btn, self.forward_btn, self.reload_btn,
                   self.home_btn, self.bookmark_btn, self.download_btn,
                   self.settings_btn]:
            btn.setIconSize(icon_size)

        # Load SVG icons
        icon_path = os.path.join(os.path.dirname(__file__), "resources/icons/svg")
        self.back_btn.setIcon(QIcon(os.path.join(icon_path, "cat-back.svg")))
        self.forward_btn.setIcon(QIcon(os.path.join(icon_path, "cat-forward.svg")))
        self.reload_btn.setIcon(QIcon(os.path.join(icon_path, "cat-reload.svg")))
        self.home_btn.setIcon(QIcon(os.path.join(icon_path, "cat-home.svg")))
        self.bookmark_btn.setIcon(QIcon(os.path.join(icon_path, "cat-bookmark.svg")))
        self.download_btn.setIcon(QIcon(os.path.join(icon_path, "cat-download.svg")))
        self.settings_btn.setIcon(QIcon(os.path.join(icon_path, "cat-settings.svg")))

class Browser(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle('Nyan Browser')
        self.setGeometry(100, 100, 1024, 768)

        # Create central widget and layout
        self.central_widget = QWidget()
        self.setCentralWidget(self.central_widget)
        self.layout = QVBoxLayout(self.central_widget)

        # Create tab widget
        self.tabs = QTabWidget()
        self.tabs.setTabsClosable(True)
        self.tabs.tabCloseRequested.connect(self.close_tab)

        # Add tab button
        self.add_tab_btn = QPushButton("+")
        self.add_tab_btn.setFixedSize(30, 30)
        self.add_tab_btn.clicked.connect(self.add_tab)

        # Tab bar layout
        tab_layout = QHBoxLayout()
        tab_layout.addWidget(self.tabs)
        tab_layout.addWidget(self.add_tab_btn)

        self.layout.addLayout(tab_layout)

        # Add initial tab
        self.add_tab()

        # Set stylesheet
        self.setStyleSheet("""
            QTabWidget::pane {
                border: none;
                background: #f0f0f0;
            }
            QTabBar::tab {
                background: #ffe6f2;
                padding: 8px;
                margin-right: 2px;
                border-radius: 12px 12px 0 0;
                min-width: 100px;
                border: 2px solid #ff99cc;
                border-bottom: none;
            }
            QTabBar::tab:selected {
                background: #ff99cc;
                color: white;
            }
            QTabBar::tab:hover {
                background: #ffb3d9;
            }
            QPushButton {
                border: none;
                padding: 5px;
                border-radius: 4px;
                margin: 2px;
            }
            QPushButton:hover {
                background: #ff99cc;
            }
            QLineEdit {
                padding: 5px;
                border: 2px solid #ff99cc;
                border-radius: 12px;
                margin: 2px;
                min-width: 300px;
            }
            QLineEdit:focus {
                border-color: #ff66aa;
                background: #fff6f9;
            }
        """)

    def add_tab(self):
        tab = TabWidget()
        index = self.tabs.addTab(tab, "New Tab")
        self.tabs.setCurrentIndex(index)
        tab.go_home()

    def close_tab(self, index):
        if self.tabs.count() > 1:
            self.tabs.removeTab(index)
        else:
            self.add_tab()

def main():
    app = QApplication(sys.argv)
    browser = Browser()
    browser.show()
    sys.exit(app.exec_())

if __name__ == '__main__':
    main()
