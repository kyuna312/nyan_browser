import json
import os
from pathlib import Path
from typing import Any, Dict

class Settings:
    def __init__(self):
        self.config_dir = Path.home() / '.config' / 'nyan_browser'
        self.config_file = self.config_dir / 'settings.json'
        self.defaults = {
            'theme': 'light',
            'start_page': 'nyan://home',
            'search_engine': 'https://www.google.com/search?q={}',
            'enable_javascript': True,
            'enable_plugins': False,
            'user_agent': 'NyanBrowser/1.0',
            'cache_size': 100 * 1024 * 1024,  # 100MB
            'performance': {
                'hardware_acceleration': True,
                'process_model': 'per_site',
                'memory_limit': 500 * 1024 * 1024  # 500MB
            },
            'privacy': {
                'do_not_track': True,
                'clear_on_exit': False,
                'block_third_party_cookies': True
            }
        }
        self.settings = self.load_settings()

    def load_settings(self) -> Dict[str, Any]:
        if not self.config_dir.exists():
            self.config_dir.mkdir(parents=True)

        if not self.config_file.exists():
            return self.defaults.copy()

        try:
            with open(self.config_file, 'r') as f:
                settings = json.load(f)
            return {**self.defaults, **settings}
        except Exception:
            return self.defaults.copy()

    def save_settings(self):
        with open(self.config_file, 'w') as f:
            json.dump(self.settings, f, indent=2)

    def get(self, key: str, default: Any = None) -> Any:
        return self.settings.get(key, default)

    def set(self, key: str, value: Any):
        self.settings[key] = value
        self.save_settings()
