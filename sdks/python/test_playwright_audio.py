import asyncio
from playwright.async_api import async_playwright
import json

async def main():
    print("Initializing browser testing...")
    async with async_playwright() as p:
        # Launch Chromium headless
        browser = await p.chromium.launch(headless=True)
        page = await browser.new_page()
        
        url = "http://localhost:8001/media/video_generator/harmonics_en/index.html"
        print(f"Navigating to {url}...")
        
        await page.goto(url)
        print("Page loaded. Waiting for video-player...")
        
        # Wait for player to be in the DOM
        await page.wait_for_load_state("networkidle")
        
        # Give it a couple of seconds to initialize audio context
        await asyncio.sleep(5)
        
        print("Evaluating background audio state...")
        
        js_code = """
        () => {
            const player = window.b4n1_player;
            if (!player) return null;
            return {
                narrationReady: player.audio?.readyState > 0,
                musicReady: player.music?.readyState > 0,
                currentTime: player.audio?.currentTime || 0,
                musicTime: player.music?.currentTime || 0,
                musicVolume: player.music?.volume || 0,
                musicPaused: player.music?.paused,
                audioPaused: player.audio?.paused
            };
        }
        """
        
        result = await page.evaluate(js_code)
        
        print("Evaluation Result:")
        print(json.dumps(result, indent=2))
        
        if result.get("musicReady"):
            print("SUCCESS: Music player state accessible and ready!")
            if result.get("musicVolume") > 0:
                print(f"SUCCESS: Music volume is correctly set at {result.get('musicVolume')}")
            else:
                print("WARNING: Music volume is 0 or undefined.")
        else:
            print("WARNING: Player state might not be fully initialized or playing yet.")
            
        await browser.close()

if __name__ == "__main__":
    asyncio.run(main())
