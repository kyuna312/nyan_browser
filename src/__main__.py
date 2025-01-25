import sys
import os
import cProfile
import asyncio
from core.browser import NyanBrowser
from utils.process import ProcessManager
from utils.memory import MemoryTracker

async def main():
    # Setup optimizations
    process_manager = ProcessManager()
    memory_tracker = MemoryTracker()

    if '--profile' in sys.argv:
        profiler = cProfile.Profile()
        profiler.enable()

    # Enable Python optimizations
    if not sys.flags.optimize:
        os.environ['PYTHONOPTIMIZE'] = '1'

    # Start browser with optimizations
    browser = NyanBrowser.instance()

    # Monitor resources
    async def monitor():
        while True:
            process_manager.collect_metrics()
            memory_tracker.cleanup()
            await asyncio.sleep(60)

    monitor_task = asyncio.create_task(monitor())

    try:
        exit_code = await browser.run()
    finally:
        monitor_task.cancel()
        if '--profile' in sys.argv:
            profiler.disable()
            profiler.dump_stats('nyan_browser_profile.stats')

    sys.exit(exit_code)

if __name__ == '__main__':
    asyncio.run(main())
