from typing import Optional
from PyQt5.QtWidgets import QApplication
from ..ui.window import BrowserWindow
from ..services.history import HistoryService
from ..services.bookmarks import BookmarkService
from ..services.downloads import DownloadService
from ..utils.performance import PerformanceMonitor
from ..utils.memory import MemoryManager
from ..config.settings import Settings

class NyanBrowser:
    _instance: Optional['NyanBrowser'] = None

    def __init__(self):
        self.settings = Settings()
        self.history = HistoryService()
        self.bookmarks = BookmarkService()
        self.downloads = DownloadService()
        self.performance = PerformanceMonitor()
        self.memory = MemoryManager()
        self.windows = []

    @classmethod
    def instance(cls) -> 'NyanBrowser':
        if cls._instance is None:
            cls._instance = cls()
        return cls._instance

    def create_window(self) -> BrowserWindow:
        window = BrowserWindow(self)
        self.windows.append(window)
        return window

    def run(self):
        app = QApplication([])
        window = self.create_window()
        window.show()
        return app.exec_()
