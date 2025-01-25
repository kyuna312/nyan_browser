from PyQt5.QtWebEngineCore import QWebEngineUrlSchemeHandler
from PyQt5.QtWebEngineWidgets import QWebEngineProfile
from typing import Dict, Optional
import weakref

class WebViewService:
    _instance: Optional['WebViewService'] = None
    _profiles: Dict[str, 'BrowserProfile'] = weakref.WeakValueDictionary()

    @classmethod
    def instance(cls) -> 'WebViewService':
        if cls._instance is None:
            cls._instance = cls()
        return cls._instance

    def create_profile(self, name: str) -> 'BrowserProfile':
        if name in self._profiles:
            return self._profiles[name]

        profile = BrowserProfile(name)
        self._profiles[name] = profile
        return profile

class BrowserProfile:
    def __init__(self, name: str):
        self.profile = QWebEngineProfile(name)
        self.setup_profile()

    def setup_profile(self):
        # Optimize profile settings
        self.profile.setHttpCacheMaximumSize(100 * 1024 * 1024)  # 100MB
        self.profile.setHttpCacheType(QWebEngineProfile.MemoryHttpCache)
        self.profile.setPersistentCookiesPolicy(QWebEngineProfile.NoPersistentCookies)

        # Custom scheme handlers
        self.register_scheme_handlers()

    def register_scheme_handlers(self):
        # Register custom URL schemes
        for scheme in ['nyan', 'local']:
            handler = CustomSchemeHandler()
            self.profile.installUrlSchemeHandler(
                scheme.encode(), handler
            )
